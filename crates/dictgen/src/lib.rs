pub fn generate_table<'d, W: std::io::Write, V: std::fmt::Display>(
    file: &mut W,
    name: &str,
    value_type: &str,
    data: impl Iterator<Item = (&'d str, V)>,
) -> Result<(), std::io::Error> {
    let mut data: Vec<_> = data.collect();
    data.sort_unstable_by_key(|v| unicase::UniCase::new(v.0));

    let mut smallest = usize::MAX;
    let mut largest = usize::MIN;

    writeln!(
        file,
        "pub static {}: dictgen::DictTable<{}> = dictgen::DictTable {{",
        name, value_type
    )?;
    writeln!(file, "    table: &[")?;
    for (key, value) in data {
        smallest = std::cmp::min(smallest, key.len());
        largest = std::cmp::max(largest, key.len());

        let key = if key.is_ascii() {
            format!("dictgen::InsensitiveStr::Ascii({:?})", key)
        } else {
            format!("dictgen::InsensitiveStr::Unicode({:?})", key)
        };

        writeln!(file, "      ({}, {}),", key, value)?;
    }
    writeln!(file, "   ],")?;
    writeln!(file, "   range: {}..={},", smallest, largest)?;
    writeln!(file, "}};")?;

    Ok(())
}

pub struct DictTable<V: 'static> {
    pub table: &'static [(InsensitiveStr, V)],
    pub range: std::ops::RangeInclusive<usize>,
}

impl<V> DictTable<V> {
    pub fn find(&self, word: &'_ unicase::UniCase<&str>) -> Option<&'static V> {
        if self.range.contains(&word.len()) {
            self.table
                .binary_search_by_key(word, |(key, _)| key.convert())
                .map(|i| &self.table[i].1)
                .ok()
        } else {
            None
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (unicase::UniCase<&'static str>, &'static V)> {
        self.table.iter().map(|row| (row.0.convert(), &row.1))
    }
}

// Avoid unicase's use of const-fn so large tables don't OOM
#[derive(Copy, Clone, Debug)]
pub enum InsensitiveStr {
    Unicode(&'static str),
    Ascii(&'static str),
}

impl InsensitiveStr {
    fn convert(self) -> unicase::UniCase<&'static str> {
        match self {
            InsensitiveStr::Unicode(s) => unicase::UniCase::unicode(s),
            InsensitiveStr::Ascii(s) => unicase::UniCase::ascii(s),
        }
    }
}
