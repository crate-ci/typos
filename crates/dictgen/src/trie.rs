/// # Panics
///
/// - On duplicate entry
#[cfg(feature = "codegen")]
pub fn generate_trie<'d, W: std::io::Write, V: std::fmt::Display>(
    file: &mut W,
    prefix: &str,
    value_type: &str,
    data: impl Iterator<Item = (&'d str, V)>,
    limit: usize,
) -> Result<(), std::io::Error> {
    codegen::generate_trie(file, prefix, value_type, data, limit)
}

pub struct DictTrie<V: 'static> {
    pub root: &'static DictTrieNode<V>,
    pub unicode: &'static crate::DictTable<V>,
    pub range: core::ops::RangeInclusive<usize>,
}

impl<V> DictTrie<V> {
    pub fn find(&self, word: &'_ unicase::UniCase<&str>) -> Option<&'static V> {
        if self.range.contains(&word.len()) {
            let bytes = word.as_bytes();

            let mut child = &self.root;
            for i in 0..bytes.len() {
                match child.children {
                    DictTrieChild::Nested(n) => {
                        let byte = bytes[i];
                        let index = if byte.is_ascii_lowercase() {
                            byte - b'a'
                        } else if byte.is_ascii_uppercase() {
                            byte - b'A'
                        } else {
                            return self.unicode.find(word);
                        };
                        debug_assert!(index < 26);
                        if let Some(next) = n[index as usize].as_ref() {
                            child = next;
                        } else {
                            return None;
                        }
                    }
                    DictTrieChild::Flat(t) => {
                        let remaining = &bytes[i..bytes.len()];
                        // Unsafe: Everything before has been proven to be ASCII, so this should be
                        // safe.
                        let remaining = unsafe { core::str::from_utf8_unchecked(remaining) };
                        // Reuse the prior ascii check, rather than doing it again
                        let remaining = if word.is_ascii() {
                            unicase::UniCase::ascii(remaining)
                        } else {
                            unicase::UniCase::unicode(remaining)
                        };
                        return t.find(&remaining);
                    }
                }
            }
            child.value.as_ref()
        } else {
            None
        }
    }
}

pub struct DictTrieNode<V: 'static> {
    pub children: DictTrieChild<V>,
    pub value: Option<V>,
}

pub enum DictTrieChild<V: 'static> {
    Nested(&'static [Option<&'static DictTrieNode<V>>; 26]),
    Flat(&'static crate::DictTable<V>),
}

#[cfg(feature = "codegen")]
mod codegen {
    pub(super) fn generate_trie<'d, W: std::io::Write, V: std::fmt::Display>(
        file: &mut W,
        prefix: &str,
        value_type: &str,
        data: impl Iterator<Item = (&'d str, V)>,
        limit: usize,
    ) -> Result<(), std::io::Error> {
        let mut root = DynRoot::new(data);
        root.burst(limit);

        let unicode_table_name = format!("{}_UNICODE_TABLE", prefix);

        writeln!(
            file,
            "pub static {}_TRIE: dictgen::DictTrie<{}> = dictgen::DictTrie {{",
            prefix, value_type
        )?;
        writeln!(file, "    root: &{},", gen_node_name(prefix, ""))?;
        writeln!(file, "    unicode: &{},", &unicode_table_name)?;
        writeln!(
            file,
            "    range: {}..={},",
            root.range.start(),
            root.range.end()
        )?;
        writeln!(file, "}};")?;
        writeln!(file)?;

        crate::generate_table(
            file,
            &unicode_table_name,
            value_type,
            root.unicode.into_iter(),
        )?;
        writeln!(file)?;

        let mut nodes = vec![("".to_owned(), &root.root)];
        while let Some((start, node)) = nodes.pop() {
            let node_name = gen_node_name(prefix, &start);
            let children_name = gen_children_name(prefix, &start);
            writeln!(
                file,
                "static {}: dictgen::DictTrieNode<{}> = dictgen::DictTrieNode {{",
                node_name, value_type
            )?;
            writeln!(
                file,
                "    children: {}(&{}),",
                gen_type_name(&node.children),
                children_name
            )?;
            if let Some(value) = node.value.as_ref() {
                writeln!(file, "    value: Some({}),", value)?;
            } else {
                writeln!(file, "    value: None,")?;
            }
            writeln!(file, "}};")?;
            writeln!(file)?;

            match &node.children {
                DynChild::Nested(n) => {
                    writeln!(
                        file,
                        "static {}: [Option<&dictgen::DictTrieNode<{}>>; 26] = [",
                        children_name, value_type,
                    )?;
                    for b in b'a'..=b'z' {
                        if let Some(child) = n.get(&b) {
                            let c = b as char;
                            let next_start = format!("{}{}", start, c);
                            writeln!(file, "    Some(&{}),", gen_node_name(prefix, &next_start))?;
                            nodes.push((next_start, child));
                        } else {
                            writeln!(file, "    None,")?;
                        }
                    }
                    writeln!(file, "];")?;
                }
                DynChild::Flat(v) => {
                    let table_input = v.iter().map(|(k, v)| {
                        let k = std::str::from_utf8(k).expect("this was originally a `str`");
                        (k, v)
                    });
                    crate::generate_table(file, &children_name, value_type, table_input)?;
                }
            }
            writeln!(file)?;
            writeln!(file)?;
        }

        Ok(())
    }

    fn gen_node_name(prefix: &str, start: &str) -> String {
        if start.is_empty() {
            format!("{}_NODE", prefix)
        } else {
            let mut start = start.to_owned();
            start.make_ascii_uppercase();
            format!("{}_{}_NODE", prefix, start)
        }
    }

    fn gen_children_name(prefix: &str, start: &str) -> String {
        if start.is_empty() {
            format!("{}_CHILDREN", prefix)
        } else {
            let mut start = start.to_owned();
            start.make_ascii_uppercase();
            format!("{}_{}_CHILDREN", prefix, start)
        }
    }

    fn gen_type_name<V>(leaf: &DynChild<V>) -> &'static str {
        match leaf {
            DynChild::Nested(_) => "dictgen::DictTrieChild::Nested",
            DynChild::Flat(_) => "dictgen::DictTrieChild::Flat",
        }
    }

    struct DynRoot<'s, V> {
        root: DynNode<'s, V>,
        unicode: Vec<(&'s str, V)>,
        range: std::ops::RangeInclusive<usize>,
    }

    impl<'s, V> DynRoot<'s, V> {
        fn new(data: impl Iterator<Item = (&'s str, V)>) -> Self {
            let mut overflow = Vec::new();
            let mut unicode = Vec::default();
            let mut smallest = usize::MAX;
            let mut largest = usize::MIN;
            let mut existing = std::collections::HashSet::new();
            let mut empty = None;
            for (key, value) in data {
                if existing.contains(key) {
                    panic!("Duplicate present: {}", key);
                }
                existing.insert(key);

                if key.is_empty() {
                    empty = Some(value);
                } else {
                    smallest = std::cmp::min(smallest, key.len());
                    largest = std::cmp::max(largest, key.len());
                    if key.bytes().all(|b| b.is_ascii_alphabetic()) {
                        overflow.push((key.as_bytes(), value));
                    } else {
                        unicode.push((key, value));
                    }
                }
            }
            Self {
                root: DynNode {
                    children: DynChild::Flat(overflow),
                    value: empty,
                },
                unicode,
                range: smallest..=largest,
            }
        }

        fn burst(&mut self, limit: usize) {
            self.root.burst(limit);
        }
    }

    struct DynNode<'s, V> {
        children: DynChild<'s, V>,
        value: Option<V>,
    }

    impl<'s, V> DynNode<'s, V> {
        fn burst(&mut self, limit: usize) {
            self.children.burst(limit)
        }
    }

    enum DynChild<'s, V> {
        Nested(std::collections::BTreeMap<u8, DynNode<'s, V>>),
        Flat(Vec<(&'s [u8], V)>),
    }

    impl<'s, V> DynChild<'s, V> {
        fn burst(&mut self, limit: usize) {
            match self {
                DynChild::Nested(children) => {
                    for child in children.values_mut() {
                        child.burst(limit);
                    }
                }
                DynChild::Flat(v) if v.len() < limit => (),
                DynChild::Flat(v) => {
                    let mut old_v = Vec::new();
                    std::mem::swap(&mut old_v, v);
                    let mut nodes = std::collections::BTreeMap::new();
                    for (key, value) in old_v {
                        assert!(!key.is_empty());
                        let start = key[0].to_ascii_lowercase();
                        assert!(start.is_ascii_alphabetic());
                        let node = nodes.entry(start).or_insert_with(|| DynNode {
                            children: DynChild::Flat(Vec::new()),
                            value: None,
                        });
                        let remaining = &key[1..];
                        if remaining.is_empty() {
                            assert!(node.value.is_none());
                            node.value = Some(value);
                        } else {
                            match &mut node.children {
                                DynChild::Nested(_) => {
                                    unreachable!("Only overflow at this point")
                                }
                                DynChild::Flat(ref mut v) => {
                                    v.push((remaining, value));
                                }
                            }
                        }
                    }
                    *self = DynChild::Nested(nodes);
                    self.burst(limit);
                }
            }
        }
    }
}
