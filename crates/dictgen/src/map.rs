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

        writeln!(
            file,
            "pub static {name}: dictgen::DictTable<{value_type}> = dictgen::DictTable {{"
        )?;
        writeln!(file, "    keys: &[")?;
        for (key, _value) in data.iter() {
            smallest = std::cmp::min(smallest, key.len());
            largest = std::cmp::max(largest, key.len());

            let key = if key.is_ascii() {
                format!("dictgen::InsensitiveStr::Ascii({key:?})")
            } else {
                format!("dictgen::InsensitiveStr::Unicode({key:?})")
            };

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
}

pub struct DictMap<V: 'static> {
    pub map: phf::Map<crate::InsensitiveStr<'static>, V>,
    pub range: std::ops::RangeInclusive<usize>,
}

impl<V> DictMap<V> {
    pub fn find(&self, word: &'_ unicase::UniCase<&str>) -> Option<&V> {
        if self.range.contains(&word.len()) {
            self.map.get(&(*word).into())
        } else {
            None
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (unicase::UniCase<&str>, &V)> + '_ {
        self.map.entries().map(|(k, v)| (k.convert(), v))
    }
}
