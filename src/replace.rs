use std::collections::BTreeMap;
use std::path;

#[derive(Clone, Debug, Default)]
pub(crate) struct Deferred {
    pub(crate) content: BTreeMap<path::PathBuf, BTreeMap<usize, Vec<Correction>>>,
    pub(crate) paths: BTreeMap<path::PathBuf, Vec<Correction>>,
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub(crate) struct Correction {
    pub(crate) byte_offset: usize,
    pub(crate) typo: Vec<u8>,
    pub(crate) correction: Vec<u8>,
}

impl Correction {
    pub(crate) fn new(byte_offset: usize, typo: &str, correction: &str) -> Self {
        Self {
            byte_offset,
            typo: typo.as_bytes().to_vec(),
            correction: correction.as_bytes().to_vec(),
        }
    }
}

pub(crate) fn correct(mut line: Vec<u8>, corrections: &[Correction]) -> Vec<u8> {
    let mut corrections: Vec<_> = corrections.iter().collect();
    corrections.sort_unstable();
    corrections.reverse();

    for correction in corrections {
        let start = correction.byte_offset;
        let end = start + correction.typo.len();
        line.splice(start..end, correction.correction.iter().copied());
    }

    line
}

#[cfg(test)]
mod test {
    use super::*;

    fn simple_correct(line: &str, corrections: Vec<(usize, &str, &str)>) -> String {
        let line = line.as_bytes().to_vec();
        let corrections: Vec<_> = corrections
            .into_iter()
            .map(|(byte_offset, typo, correction)| Correction {
                byte_offset,
                typo: typo.as_bytes().to_vec(),
                correction: correction.as_bytes().to_vec(),
            })
            .collect();
        let actual = correct(line, &corrections);
        String::from_utf8(actual).unwrap()
    }

    #[test]
    fn test_correct_single() {
        let actual = simple_correct("foo foo foo", vec![(4, "foo", "bar")]);
        assert_eq!(actual, "foo bar foo");
    }

    #[test]
    fn test_correct_single_grow() {
        let actual = simple_correct("foo foo foo", vec![(4, "foo", "happy")]);
        assert_eq!(actual, "foo happy foo");
    }

    #[test]
    fn test_correct_single_shrink() {
        let actual = simple_correct("foo foo foo", vec![(4, "foo", "if")]);
        assert_eq!(actual, "foo if foo");
    }

    #[test]
    fn test_correct_start() {
        let actual = simple_correct("foo foo foo", vec![(0, "foo", "bar")]);
        assert_eq!(actual, "bar foo foo");
    }

    #[test]
    fn test_correct_end() {
        let actual = simple_correct("foo foo foo", vec![(8, "foo", "bar")]);
        assert_eq!(actual, "foo foo bar");
    }

    #[test]
    fn test_correct_end_grow() {
        let actual = simple_correct("foo foo foo", vec![(8, "foo", "happy")]);
        assert_eq!(actual, "foo foo happy");
    }

    #[test]
    fn test_correct_multiple() {
        let actual = simple_correct(
            "foo foo foo",
            vec![(4, "foo", "happy"), (8, "foo", "world")],
        );
        assert_eq!(actual, "foo happy world");
    }
}
