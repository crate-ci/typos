#[cfg(feature = "codegen")]
pub struct MapGen<'g> {
    pub(crate) gen: crate::DictGen<'g>,
    pub(crate) unicase: bool,
    pub(crate) unicode: bool,
}

#[cfg(feature = "codegen")]
impl MapGen<'_> {
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
        for (key, _) in data.iter() {
            let key = key.as_ref();
            smallest = std::cmp::min(smallest, key.len());
            largest = std::cmp::max(largest, key.len());
        }
        if largest == 0 {
            smallest = 0;
        }

        writeln!(
            file,
            "pub static {name}: dictgen::Map<{key_type}, {value_type}> = dictgen::Map {{"
        )?;

        match (self.unicase, self.unicode) {
            (true, true) => {
                let mut builder = phf_codegen::Map::new();
                let data = data
                    .iter()
                    .map(|(key, value)| {
                        let key = key.as_ref();
                        (
                            if key.is_ascii() {
                                crate::InsensitiveStr::Ascii(key)
                            } else {
                                crate::InsensitiveStr::Unicode(key)
                            },
                            value.to_string(),
                        )
                    })
                    .collect::<Vec<_>>();
                for (key, value) in data.iter() {
                    builder.entry(key, value.as_str());
                }
                let builder = builder.build();
                writeln!(file, "    map: {builder},")?;
            }
            (true, false) => {
                let mut builder = phf_codegen::Map::new();
                let data = data
                    .iter()
                    .map(|(key, value)| (crate::InsensitiveAscii(key.as_ref()), value.to_string()))
                    .collect::<Vec<_>>();
                for (key, value) in data.iter() {
                    builder.entry(key, value.as_str());
                }
                let builder = builder.build();
                writeln!(file, "    map: {builder},")?;
            }
            (false, _) => {
                let mut builder = phf_codegen::Map::new();
                let data = data
                    .iter()
                    .map(|(key, value)| (key, value.to_string()))
                    .collect::<Vec<_>>();
                for (key, value) in data.iter() {
                    builder.entry(key.as_ref(), value.as_str());
                }
                let builder = builder.build();
                writeln!(file, "    map: {builder},")?;
            }
        }

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
}

pub struct Map<K: 'static, V: 'static> {
    pub map: phf::Map<K, V>,
    pub range: std::ops::RangeInclusive<usize>,
}

impl<V> Map<crate::InsensitiveStr<'_>, V> {
    #[inline]
    pub fn find(&self, word: &'_ unicase::UniCase<&str>) -> Option<&V> {
        if self.range.contains(&word.len()) {
            self.map.get(&(*word).into())
        } else {
            None
        }
    }
}

impl<V> Map<crate::InsensitiveAscii<'_>, V> {
    #[inline]
    pub fn find(&self, word: &'_ unicase::Ascii<&str>) -> Option<&V> {
        if self.range.contains(&word.len()) {
            self.map.get(&(*word).into())
        } else {
            None
        }
    }
}

impl<V> Map<&str, V> {
    #[inline]
    pub fn find(&self, word: &'_ &str) -> Option<&V> {
        if self.range.contains(&word.len()) {
            self.map.get(word)
        } else {
            None
        }
    }
}
