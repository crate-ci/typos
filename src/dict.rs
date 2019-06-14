include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

#[derive(Default)]
pub struct Dictionary {}

impl Dictionary {
    pub fn new() -> Self {
        Dictionary {}
    }

    pub fn correct_str<'s, 'w>(&'s self, word: &'w str) -> Option<&'s str> {
        map_lookup(&DICTIONARY, word)
    }

    pub fn correct_bytes<'s, 'w>(&'s self, word: &'w [u8]) -> Option<&'s str> {
        std::str::from_utf8(word)
            .ok()
            .and_then(|word| self.correct_str(word))
    }
}

fn map_lookup(
    map: &'static phf::Map<UniCase<&'static str>, &'static str>,
    key: &str,
) -> Option<&'static str> {
    // This transmute should be safe as `get` will not store the reference with
    // the expanded lifetime. This is due to `Borrow` being overly strict and
    // can't have an impl for `&'static str` to `Borrow<&'a str>`.
    //
    // See https://github.com/rust-lang/rust/issues/28853#issuecomment-158735548
    unsafe {
        let key = ::std::mem::transmute::<_, &'static str>(key);
        map.get(&UniCase(key)).cloned()
    }
}
