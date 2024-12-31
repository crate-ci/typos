#[cfg(feature = "codegen")]
pub struct MatchGen<'g> {
    pub(crate) gen: crate::DictGen<'g>,
}

#[cfg(feature = "codegen")]
impl MatchGen<'_> {
    pub fn write<W: std::io::Write, V: std::fmt::Display>(
        &self,
        file: &mut W,
        data: impl Iterator<Item = (impl AsRef<str>, V)>,
    ) -> Result<(), std::io::Error> {
        let mut data: Vec<_> = data.collect();
        data.sort_unstable_by_key(|v| unicase::UniCase::new(v.0.as_ref().to_owned()));

        let name = self.gen.name;
        let value_type = self.gen.value_type;

        writeln!(file, "pub struct {name};")?;
        writeln!(file, "impl {name} {{")?;
        writeln!(
            file,
            "    pub fn find(&self, word: &&str) -> Option<&'static {value_type}> {{"
        )?;
        writeln!(file, "        match *word {{")?;
        for (key, value) in data.iter() {
            let key = key.as_ref();
            writeln!(file, "            {key:?} => Some(&{value}.as_slice()),")?;
        }
        writeln!(file, "            _ => None,")?;
        writeln!(file, "        }}")?;
        writeln!(file, "    }}")?;
        writeln!(file, "}}")?;

        Ok(())
    }
}
