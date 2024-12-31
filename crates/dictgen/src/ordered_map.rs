#[cfg(feature = "codegen")]
pub struct OrderedMapGen<'g> {
    pub(crate) gen: crate::DictGen<'g>,
    pub(crate) unicase: bool,
    pub(crate) unicode: bool,
}

#[cfg(feature = "codegen")]
impl OrderedMapGen<'_> {
    pub fn unicase(mut self, yes: bool) -> Self {
        self.unicase = yes;
        self
    }

    pub fn unicode(mut self, yes: bool) -> Self {
        self.unicode = yes;
        self
    }

    pub fn write<W: std::io::Write, V: std::fmt::Display>(
        &self,
        file: &mut W,
        data: impl Iterator<Item = (impl AsRef<str>, V)>,
    ) -> Result<(), std::io::Error> {
        let mut data: Vec<_> = data.collect();
        data.sort_unstable_by_key(|v| unicase::UniCase::new(v.0.as_ref().to_owned()));

        let name = self.gen.name;
        let key_type = self.key_type();
        let value_type = self.gen.value_type;

        let mut smallest = usize::MAX;
        let mut largest = usize::MIN;

        writeln!(
            file,
            "pub static {name}: dictgen::OrderedMap<{key_type}, {value_type}> = dictgen::OrderedMap {{"
        )?;
        writeln!(file, "    keys: &[")?;
        for (key, _value) in data.iter() {
            let key = key.as_ref();
            smallest = std::cmp::min(smallest, key.len());
            largest = std::cmp::max(largest, key.len());

            let key = self.key_new(key);

            writeln!(file, "      {key},")?;
        }
        if largest == 0 {
            smallest = 0;
        }
        writeln!(file, "    ],")?;
        writeln!(file, "    values: &[")?;
        for (_key, value) in data.iter() {
            writeln!(file, "      {value},")?;
        }
        writeln!(file, "    ],")?;
        writeln!(file, "    range: {smallest}..={largest},")?;
        writeln!(file, "}};")?;

        Ok(())
    }

    fn key_type(&self) -> &'static str {
        match (self.unicase, self.unicode) {
            (true, true) => "dictgen::InsensitiveStr<'static>",
            (true, false) => "dictgen::InsensitiveAscii<'static>",
            (false, _) => "&'static str",
        }
    }

    fn key_new(&self, key: &str) -> String {
        match (self.unicase, self.unicode) {
            (true, true) => {
                if key.is_ascii() {
                    format!("dictgen::InsensitiveStr::Ascii({key:?})")
                } else {
                    format!("dictgen::InsensitiveStr::Unicode({key:?})")
                }
            }
            (true, false) => format!("dictgen::InsensitiveAscii({key:?})"),
            (false, _) => format!("{key:?}"),
        }
    }
}

pub struct OrderedMap<K: 'static, V: 'static> {
    pub keys: &'static [K],
    pub values: &'static [V],
    pub range: core::ops::RangeInclusive<usize>,
}

impl<V> OrderedMap<crate::InsensitiveStr<'_>, V> {
    #[inline]
    pub fn find(&self, word: &'_ unicase::UniCase<&str>) -> Option<&'static V> {
        if self.range.contains(&word.len()) {
            self.keys
                .binary_search_by_key(word, |key| key.convert())
                .map(|i| &self.values[i])
                .ok()
        } else {
            None
        }
    }
}

impl<V> OrderedMap<crate::InsensitiveAscii<'_>, V> {
    #[inline]
    pub fn find(&self, word: &'_ unicase::Ascii<&str>) -> Option<&'static V> {
        if self.range.contains(&word.len()) {
            self.keys
                .binary_search_by_key(word, |key| key.convert())
                .map(|i| &self.values[i])
                .ok()
        } else {
            None
        }
    }
}

impl<V> OrderedMap<&str, V> {
    #[inline]
    pub fn find(&self, word: &'_ &str) -> Option<&'static V> {
        if self.range.contains(&word.len()) {
            self.keys.binary_search(word).map(|i| &self.values[i]).ok()
        } else {
            None
        }
    }
}
