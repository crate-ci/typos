#[cfg(feature = "codegen")]
pub fn generate_map<'d, W: std::io::Write, V: std::fmt::Display>(
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

impl<'s> phf_shared::PhfHash for crate::InsensitiveStr<'s> {
    #[inline]
    fn phf_hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(self, state)
    }
}

impl<'s> phf_shared::FmtConst for crate::InsensitiveStr<'s> {
    fn fmt_const(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            crate::InsensitiveStr::Ascii(_) => f.write_str("dictgen::InsensitiveStr::Ascii(")?,
            crate::InsensitiveStr::Unicode(_) => {
                f.write_str("dictgen::InsensitiveStr::Unicode(")?
            }
        }

        self.into_inner().fmt_const(f)?;
        f.write_str(")")
    }
}

impl<'b, 'a: 'b> phf_shared::PhfBorrow<crate::InsensitiveStr<'b>> for crate::InsensitiveStr<'a> {
    fn borrow(&self) -> &crate::InsensitiveStr<'b> {
        self
    }
}
