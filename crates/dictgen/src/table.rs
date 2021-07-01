#[cfg(feature = "codegen")]
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
    writeln!(file, "    keys: &[")?;
    for (key, _value) in data.iter() {
        smallest = std::cmp::min(smallest, key.len());
        largest = std::cmp::max(largest, key.len());

        let key = if key.is_ascii() {
            format!("dictgen::InsensitiveStr::Ascii({:?})", key)
        } else {
            format!("dictgen::InsensitiveStr::Unicode({:?})", key)
        };

        writeln!(file, "      {},", key)?;
    }
    if largest == 0 {
        smallest = 0;
    }
    writeln!(file, "    ],")?;
    writeln!(file, "    values: &[")?;
    for (_key, value) in data.iter() {
        writeln!(file, "      {},", value)?;
    }
    writeln!(file, "    ],")?;
    writeln!(file, "    range: {}..={},", smallest, largest)?;
    writeln!(file, "}};")?;

    Ok(())
}

pub struct DictTable<V: 'static> {
    pub keys: &'static [InsensitiveStr<'static>],
    pub values: &'static [V],
    pub range: core::ops::RangeInclusive<usize>,
}

impl<V> DictTable<V> {
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

    pub fn iter(&self) -> impl Iterator<Item = (unicase::UniCase<&'static str>, &'static V)> + '_ {
        (0..self.keys.len()).map(move |i| (self.keys[i].convert(), &self.values[i]))
    }
}

/// UniCase look-alike that avoids const-fn so large tables don't OOM
#[derive(Copy, Clone)]
pub enum InsensitiveStr<'s> {
    Unicode(&'s str),
    Ascii(&'s str),
}

impl<'s> InsensitiveStr<'s> {
    pub fn convert(self) -> unicase::UniCase<&'s str> {
        match self {
            InsensitiveStr::Unicode(s) => unicase::UniCase::unicode(s),
            InsensitiveStr::Ascii(s) => unicase::UniCase::ascii(s),
        }
    }

    pub fn into_inner(self) -> &'s str {
        match self {
            InsensitiveStr::Unicode(s) | InsensitiveStr::Ascii(s) => s,
        }
    }
}

impl<'s> From<unicase::UniCase<&'s str>> for InsensitiveStr<'s> {
    fn from(other: unicase::UniCase<&'s str>) -> Self {
        if other.is_ascii() {
            InsensitiveStr::Ascii(other.into_inner())
        } else {
            InsensitiveStr::Unicode(other.into_inner())
        }
    }
}

impl<'s1, 's2> PartialEq<InsensitiveStr<'s2>> for InsensitiveStr<'s1> {
    #[inline]
    fn eq(&self, other: &InsensitiveStr<'s2>) -> bool {
        self.convert() == other.convert()
    }
}

impl<'s> Eq for InsensitiveStr<'s> {}

impl<'s> core::hash::Hash for InsensitiveStr<'s> {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, hasher: &mut H) {
        self.convert().hash(hasher)
    }
}

impl<'s> core::fmt::Debug for InsensitiveStr<'s> {
    #[inline]
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Debug::fmt(self.into_inner(), fmt)
    }
}

impl<'s> core::fmt::Display for InsensitiveStr<'s> {
    #[inline]
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Display::fmt(self.into_inner(), fmt)
    }
}
