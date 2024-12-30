/// `UniCase` look-alike that avoids const-fn so large tables don't OOM
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

    pub fn is_empty(self) -> bool {
        match self {
            InsensitiveStr::Unicode(s) | InsensitiveStr::Ascii(s) => s.is_empty(),
        }
    }

    pub fn len(self) -> usize {
        match self {
            InsensitiveStr::Unicode(s) | InsensitiveStr::Ascii(s) => s.len(),
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

impl<'s2> PartialEq<InsensitiveStr<'s2>> for InsensitiveStr<'_> {
    #[inline]
    fn eq(&self, other: &InsensitiveStr<'s2>) -> bool {
        self.convert() == other.convert()
    }
}

impl Eq for InsensitiveStr<'_> {}

impl PartialOrd for InsensitiveStr<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for InsensitiveStr<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.convert().cmp(&other.convert())
    }
}

impl core::hash::Hash for InsensitiveStr<'_> {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, hasher: &mut H) {
        self.convert().hash(hasher);
    }
}

impl core::fmt::Debug for InsensitiveStr<'_> {
    #[inline]
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self.into_inner(), fmt)
    }
}

impl core::fmt::Display for InsensitiveStr<'_> {
    #[inline]
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(self.into_inner(), fmt)
    }
}

#[cfg(feature = "map")]
impl phf_shared::PhfHash for InsensitiveStr<'_> {
    #[inline]
    fn phf_hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(self, state);
    }
}

#[cfg(feature = "map")]
impl phf_shared::FmtConst for InsensitiveStr<'_> {
    fn fmt_const(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            InsensitiveStr::Ascii(_) => f.write_str("dictgen::InsensitiveStr::Ascii(")?,
            InsensitiveStr::Unicode(_) => {
                f.write_str("dictgen::InsensitiveStr::Unicode(")?;
            }
        }

        self.into_inner().fmt_const(f)?;
        f.write_str(")")
    }
}

#[cfg(feature = "map")]
impl<'b, 'a: 'b> phf_shared::PhfBorrow<InsensitiveStr<'b>> for InsensitiveStr<'a> {
    fn borrow(&self) -> &InsensitiveStr<'b> {
        self
    }
}

/// `UniCase` look-alike that avoids const-fn so large tables don't OOM
#[derive(Copy, Clone)]
pub struct InsensitiveAscii<'s>(pub &'s str);

impl<'s> InsensitiveAscii<'s> {
    pub fn convert(self) -> unicase::Ascii<&'s str> {
        unicase::Ascii::new(self.0)
    }

    pub fn into_inner(self) -> &'s str {
        self.0
    }

    pub fn is_empty(self) -> bool {
        self.0.is_empty()
    }

    pub fn len(self) -> usize {
        self.0.len()
    }
}

impl<'s> From<unicase::Ascii<&'s str>> for InsensitiveAscii<'s> {
    fn from(other: unicase::Ascii<&'s str>) -> Self {
        Self(other.into_inner())
    }
}

impl<'s2> PartialEq<InsensitiveAscii<'s2>> for InsensitiveAscii<'_> {
    #[inline]
    fn eq(&self, other: &InsensitiveAscii<'s2>) -> bool {
        self.convert() == other.convert()
    }
}

impl Eq for InsensitiveAscii<'_> {}

impl PartialOrd for InsensitiveAscii<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for InsensitiveAscii<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.convert().cmp(&other.convert())
    }
}

impl core::hash::Hash for InsensitiveAscii<'_> {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, hasher: &mut H) {
        self.convert().hash(hasher);
    }
}

impl core::fmt::Debug for InsensitiveAscii<'_> {
    #[inline]
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self.into_inner(), fmt)
    }
}

impl core::fmt::Display for InsensitiveAscii<'_> {
    #[inline]
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(self.into_inner(), fmt)
    }
}

#[cfg(feature = "map")]
impl phf_shared::PhfHash for InsensitiveAscii<'_> {
    #[inline]
    fn phf_hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(self, state);
    }
}

#[cfg(feature = "map")]
impl phf_shared::FmtConst for InsensitiveAscii<'_> {
    fn fmt_const(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("dictgen::InsensitiveAscii(")?;
        self.into_inner().fmt_const(f)?;
        f.write_str(")")
    }
}

#[cfg(feature = "map")]
impl<'b, 'a: 'b> phf_shared::PhfBorrow<InsensitiveAscii<'b>> for InsensitiveAscii<'a> {
    fn borrow(&self) -> &InsensitiveAscii<'b> {
        self
    }
}
