pub use ::aho_corasick::automaton::Automaton;
pub use ::aho_corasick::dfa::Builder;
pub use ::aho_corasick::dfa::DFA;
pub use ::aho_corasick::Anchored;
pub use ::aho_corasick::Input;
pub use ::aho_corasick::MatchKind;
pub use ::aho_corasick::StartKind;

#[cfg(feature = "codegen")]
pub struct AhoCorasickGen<'g> {
    pub(crate) gen: crate::DictGen<'g>,
}

#[cfg(feature = "codegen")]
impl AhoCorasickGen<'_> {
    pub fn write<W: std::io::Write, V: std::fmt::Display>(
        &self,
        file: &mut W,
        data: impl Iterator<Item = (impl AsRef<str>, V)>,
    ) -> Result<(), std::io::Error> {
        let mut data: Vec<_> = data.collect();
        data.sort_unstable_by_key(|v| unicase::UniCase::new(v.0.as_ref().to_owned()));

        let name = self.gen.name;
        let value_type = self.gen.value_type;

        writeln!(file, "pub struct {name} {{")?;
        writeln!(file, "    dfa: dictgen::aho_corasick::DFA,")?;
        writeln!(file, "    unicode: &'static dictgen::OrderedMap<dictgen::InsensitiveStr<'static>, {value_type}>,")?;
        writeln!(file, "}}")?;
        writeln!(file)?;
        writeln!(file, "impl {name} {{")?;
        writeln!(file, "    pub fn new() -> Self {{")?;
        writeln!(
            file,
            "        static NEEDLES: &'static [&'static [u8]] = &["
        )?;
        for (key, _value) in data.iter().filter(|(k, _)| k.as_ref().is_ascii()) {
            let key = key.as_ref();
            writeln!(file, "            b{key:?},")?;
        }
        writeln!(file, "        ];")?;
        writeln!(
            file,
            "        let dfa = dictgen::aho_corasick::Builder::new()"
        )?;
        writeln!(
            file,
            "            .match_kind(dictgen::aho_corasick::MatchKind::LeftmostLongest)"
        )?;
        writeln!(
            file,
            "            .start_kind(dictgen::aho_corasick::StartKind::Anchored)"
        )?;
        writeln!(file, "            .ascii_case_insensitive(true)")?;
        writeln!(file, "            .build(NEEDLES)")?;
        writeln!(file, "            .unwrap();")?;
        crate::DictGen::new()
            .name("UNICODE_TABLE")
            .value_type(value_type)
            .ordered_map()
            .write(
                file,
                data.iter()
                    .filter(|(k, _)| !k.as_ref().is_ascii())
                    .map(|(k, v)| (k.as_ref(), v)),
            )?;
        writeln!(file)?;
        writeln!(file, "        Self {{")?;
        writeln!(file, "            dfa,")?;
        writeln!(file, "            unicode: &UNICODE_TABLE,")?;
        writeln!(file, "        }}")?;
        writeln!(file, "    }}")?;
        writeln!(file)?;
        writeln!(
            file,
            "    pub fn find(&self, word: &'_ unicase::UniCase<&str>) -> Option<&'static {value_type}> {{"
        )?;
        writeln!(
            file,
            "        static PATTERNID_MAP: &'static [{value_type}] = &["
        )?;
        for (_key, value) in data.iter().filter(|(k, _)| k.as_ref().is_ascii()) {
            writeln!(file, "            {value},")?;
        }
        writeln!(file, "        ];")?;
        writeln!(file, "        if word.is_ascii() {{")?;
        writeln!(
            file,
            "            use dictgen::aho_corasick::Automaton as _;"
        )?;
        writeln!(file, "            let input = dictgen::aho_corasick::Input::new(word.into_inner().as_bytes()).anchored(dictgen::aho_corasick::Anchored::Yes);")?;
        writeln!(
            file,
            "            let mat = self.dfa.try_find(&input).unwrap()?;"
        )?;
        writeln!(
            file,
            "            if mat.end() == word.into_inner().len() {{"
        )?;
        writeln!(file, "                return None;")?;
        writeln!(file, "            }}")?;
        writeln!(file, "            Some(&PATTERNID_MAP[mat.pattern()])")?;
        writeln!(file, "        }} else {{")?;
        writeln!(file, "            self.unicode.find(word)")?;
        writeln!(file, "        }}")?;
        writeln!(file, "    }}")?;
        writeln!(file, "}}")?;

        Ok(())
    }
}
