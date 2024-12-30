#[cfg(feature = "codegen")]
pub struct TrieGen<'g> {
    pub(crate) gen: crate::DictGen<'g>,
    pub(crate) limit: usize,
}

#[cfg(feature = "codegen")]
impl TrieGen<'_> {
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    /// # Panics
    ///
    /// - On duplicate entry
    pub fn write<'d, W: std::io::Write, V: std::fmt::Display>(
        &self,
        file: &mut W,
        data: impl Iterator<Item = (&'d str, V)>,
    ) -> Result<(), std::io::Error> {
        let name = self.gen.name;
        let value_type = self.gen.value_type;
        codegen::generate_trie(file, name, value_type, data, self.limit)
    }
}

pub struct Trie<V: 'static> {
    pub root: &'static TrieNode<V>,
    pub unicode: &'static crate::OrderedMap<crate::InsensitiveStr<'static>, V>,
    pub range: core::ops::RangeInclusive<usize>,
}

impl<V> Trie<V> {
    #[inline]
    pub fn find(&self, word: &'_ unicase::UniCase<&str>) -> Option<&'static V> {
        if word
            .into_inner()
            .as_bytes()
            .iter()
            .all(|b| b.is_ascii_alphabetic())
        {
            if self.range.contains(&word.len()) {
                self.find_ascii(word.as_bytes())
            } else {
                None
            }
        } else {
            self.unicode.find(word)
        }
    }

    fn find_ascii(&self, word: &[u8]) -> Option<&'static V> {
        let mut child = &self.root;
        for i in 0..word.len() {
            match child.children {
                TrieChild::Nested(n) => {
                    let byte = word[i];
                    let index = if byte.is_ascii_lowercase() {
                        byte - b'a'
                    } else if byte.is_ascii_uppercase() {
                        byte - b'A'
                    } else {
                        return None;
                    };
                    debug_assert!(index < 26);
                    if let Some(next) = n[index as usize].as_ref() {
                        child = next;
                    } else {
                        return None;
                    }
                }
                TrieChild::Flat(t) => {
                    let remaining = &word[i..word.len()];
                    // Unsafe: Everything before has been proven to be ASCII, so this should be
                    // safe.
                    let remaining = unsafe { core::str::from_utf8_unchecked(remaining) };
                    let remaining = unicase::Ascii::new(remaining);
                    return t.find(&remaining);
                }
            }
        }
        child.value.as_ref()
    }
}

pub struct TrieNode<V: 'static> {
    pub children: TrieChild<V>,
    pub value: Option<V>,
}

pub enum TrieChild<V: 'static> {
    Nested(&'static [Option<&'static TrieNode<V>>; 26]),
    Flat(&'static crate::OrderedMap<crate::InsensitiveAscii<'static>, V>),
}

#[cfg(feature = "codegen")]
mod codegen {
    pub(super) fn generate_trie<'d, W: std::io::Write, V: std::fmt::Display>(
        file: &mut W,
        name: &str,
        value_type: &str,
        data: impl Iterator<Item = (&'d str, V)>,
        limit: usize,
    ) -> Result<(), std::io::Error> {
        let mut root = DynRoot::new(data);
        root.burst(limit);

        let unicode_table_name = format!("{name}_UNICODE_TABLE");

        writeln!(
            file,
            "pub static {name}: dictgen::Trie<{value_type}> = dictgen::Trie {{"
        )?;
        writeln!(file, "    root: &{},", gen_node_name(name, ""))?;
        writeln!(file, "    unicode: &{},", &unicode_table_name)?;
        writeln!(
            file,
            "    range: {}..={},",
            root.range.start(),
            root.range.end()
        )?;
        writeln!(file, "}};")?;
        writeln!(file)?;

        crate::DictGen::new()
            .name(&unicode_table_name)
            .value_type(value_type)
            .ordered_map()
            .write(file, root.unicode.into_iter())?;
        writeln!(file)?;

        let mut nodes = vec![("".to_owned(), &root.root)];
        while let Some((start, node)) = nodes.pop() {
            let node_name = gen_node_name(name, &start);
            let children_name = gen_children_name(name, &start);
            writeln!(
                file,
                "static {node_name}: dictgen::TrieNode<{value_type}> = dictgen::TrieNode {{"
            )?;
            writeln!(
                file,
                "    children: {}(&{}),",
                gen_type_name(&node.children),
                children_name
            )?;
            if let Some(value) = node.value.as_ref() {
                writeln!(file, "    value: Some({value}),")?;
            } else {
                writeln!(file, "    value: None,")?;
            }
            writeln!(file, "}};")?;
            writeln!(file)?;

            match &node.children {
                DynChild::Nested(n) => {
                    writeln!(
                        file,
                        "static {children_name}: [Option<&dictgen::TrieNode<{value_type}>>; 26] = [",
                    )?;
                    for b in b'a'..=b'z' {
                        if let Some(child) = n.get(&b) {
                            let c = b as char;
                            let next_start = format!("{start}{c}");
                            writeln!(file, "    Some(&{}),", gen_node_name(name, &next_start))?;
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
                    crate::DictGen::new()
                        .name(&children_name)
                        .value_type(value_type)
                        .ordered_map()
                        .unicode(false)
                        .write(file, table_input)?;
                }
            }
            writeln!(file)?;
            writeln!(file)?;
        }

        Ok(())
    }

    fn gen_node_name(prefix: &str, start: &str) -> String {
        if start.is_empty() {
            format!("{prefix}_NODE")
        } else {
            let mut start = start.to_owned();
            start.make_ascii_uppercase();
            format!("{prefix}_{start}_NODE")
        }
    }

    fn gen_children_name(prefix: &str, start: &str) -> String {
        if start.is_empty() {
            format!("{prefix}_CHILDREN")
        } else {
            let mut start = start.to_owned();
            start.make_ascii_uppercase();
            format!("{prefix}_{start}_CHILDREN")
        }
    }

    fn gen_type_name<V>(leaf: &DynChild<'_, V>) -> &'static str {
        match leaf {
            DynChild::Nested(_) => "dictgen::TrieChild::Nested",
            DynChild::Flat(_) => "dictgen::TrieChild::Flat",
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
                    panic!("Duplicate present: {key}");
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

    impl<V> DynNode<'_, V> {
        fn burst(&mut self, limit: usize) {
            self.children.burst(limit);
        }
    }

    enum DynChild<'s, V> {
        Nested(std::collections::BTreeMap<u8, DynNode<'s, V>>),
        Flat(Vec<(&'s [u8], V)>),
    }

    impl<V> DynChild<'_, V> {
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
