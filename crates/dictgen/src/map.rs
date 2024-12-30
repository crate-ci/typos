#[cfg(feature = "codegen")]
pub struct DictMapGen<'g> {
    pub(crate) gen: crate::DictGen<'g>,
}

#[cfg(feature = "codegen")]
impl DictMapGen<'_> {
    pub fn write<'d, W: std::io::Write, V: std::fmt::Display>(
        &self,
        file: &mut W,
        data: impl Iterator<Item = (&'d str, V)>,
    ) -> Result<(), std::io::Error> {
        let mut data: Vec<_> = data.collect();
        data.sort_unstable_by_key(|v| unicase::UniCase::new(v.0));

        let name = self.gen.name;
        let value_type = self.gen.value_type;

        let mut smallest = usize::MAX;
        let mut largest = usize::MIN;
        let mut builder = phf_codegen::Map::new();
        let data = data
            .iter()
            .map(|(key, value)| {
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
            smallest = std::cmp::min(smallest, key.len());
            largest = std::cmp::max(largest, key.len());
            builder.entry(key, value.as_str());
        }
        let builder = builder.build();
        if largest == 0 {
            smallest = 0;
        }

        writeln!(
            file,
            "pub static {name}: dictgen::DictMap<{value_type}> = dictgen::DictMap {{"
        )?;
        writeln!(file, "    map: {builder},")?;
        writeln!(file, "    range: {smallest}..={largest},")?;
        writeln!(file, "}};")?;

        Ok(())
    }
}

pub struct DictMap<V: 'static> {
    pub map: phf::Map<crate::InsensitiveStr<'static>, V>,
    pub range: std::ops::RangeInclusive<usize>,
}

impl<V> DictMap<V> {
    #[inline]
    pub fn find(&self, word: &'_ unicase::UniCase<&str>) -> Option<&V> {
        if self.range.contains(&word.len()) {
            self.map.get(&(*word).into())
        } else {
            None
        }
    }
}
