use std::collections::BTreeMap;
use std::io::Write;
use std::path;
use std::sync;

use bstr::ByteSlice;

pub struct Replace<'r> {
    reporter: &'r dyn typos::report::Report,
    deferred: sync::Mutex<Deferred>,
}

impl<'r> Replace<'r> {
    pub(crate) fn new(reporter: &'r dyn typos::report::Report) -> Self {
        Self {
            reporter,
            deferred: sync::Mutex::new(Deferred::default()),
        }
    }

    pub fn write(&self) -> Result<(), std::io::Error> {
        let deferred = self.deferred.lock().unwrap();

        for (path, corrections) in deferred.content.iter() {
            let buffer = std::fs::read(path)?;

            let mut file = std::fs::File::create(path)?;
            for (line_idx, line) in buffer.lines_with_terminator().enumerate() {
                let line_num = line_idx + 1;
                if let Some(corrections) = corrections.get(&line_num) {
                    let line = line.to_vec();
                    let line = correct(line, &corrections);
                    file.write_all(&line)?;
                } else {
                    file.write_all(&line)?;
                }
            }
        }

        for (path, corrections) in deferred.paths.iter() {
            let orig_name = path
                .file_name()
                .and_then(|s| s.to_str())
                .expect("generating a correction requires the filename to be valid.")
                .to_owned()
                .into_bytes();
            let new_name = correct(orig_name, &corrections);
            let new_name = String::from_utf8(new_name).expect("corrections are valid utf-8");
            let new_path = path.with_file_name(new_name);
            std::fs::rename(path, new_path)?;
        }

        Ok(())
    }
}

impl<'r> typos::report::Report for Replace<'r> {
    fn report(&self, msg: typos::report::Message<'_>) -> bool {
        match msg {
            typos::report::Message::Correction(msg) => {
                if msg.corrections.len() == 1 {
                    let path = msg.path.to_owned();
                    let line_num = msg.line_num;
                    let correction = Correction::from_content(msg);
                    let mut deferred = self.deferred.lock().unwrap();
                    let content = deferred
                        .content
                        .entry(path)
                        .or_insert_with(BTreeMap::new)
                        .entry(line_num)
                        .or_insert_with(Vec::new);
                    content.push(correction);
                    false
                } else {
                    self.reporter
                        .report(typos::report::Message::Correction(msg))
                }
            }
            typos::report::Message::PathCorrection(msg) => {
                if msg.corrections.len() == 1 {
                    let path = msg.path.to_owned();
                    let correction = Correction::from_path(msg);
                    let mut deferred = self.deferred.lock().unwrap();
                    let content = deferred.paths.entry(path).or_insert_with(Vec::new);
                    content.push(correction);
                    false
                } else {
                    self.reporter
                        .report(typos::report::Message::PathCorrection(msg))
                }
            }
            _ => self.reporter.report(msg),
        }
    }
}

#[derive(Clone, Debug, Default)]
struct Deferred {
    content: BTreeMap<path::PathBuf, BTreeMap<usize, Vec<Correction>>>,
    paths: BTreeMap<path::PathBuf, Vec<Correction>>,
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
struct Correction {
    pub byte_offset: usize,
    pub typo: Vec<u8>,
    pub correction: Vec<u8>,
}

impl Correction {
    fn from_content(other: typos::report::Correction<'_>) -> Self {
        assert_eq!(other.corrections.len(), 1);
        Self {
            byte_offset: other.byte_offset,
            typo: other.typo.as_bytes().to_vec(),
            correction: other.corrections[0].as_bytes().to_vec(),
        }
    }

    fn from_path(other: typos::report::PathCorrection<'_>) -> Self {
        assert_eq!(other.corrections.len(), 1);
        Self {
            byte_offset: other.byte_offset,
            typo: other.typo.as_bytes().to_vec(),
            correction: other.corrections[0].as_bytes().to_vec(),
        }
    }
}

fn correct(mut line: Vec<u8>, corrections: &[Correction]) -> Vec<u8> {
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

    use assert_fs::prelude::*;
    use typos::report::Report;

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

    #[test]
    fn test_replace_content() {
        let temp = assert_fs::TempDir::new().unwrap();
        let input_file = temp.child("foo.txt");
        input_file.write_str("1 foo 2\n3 4 5").unwrap();

        let primary = typos::report::PrintSilent;
        let replace = Replace::new(&primary);
        replace.report(
            typos::report::Correction::default()
                .path(input_file.path())
                .line(b"1 foo 2\n3 4 5")
                .line_num(1)
                .byte_offset(2)
                .typo("foo")
                .corrections(vec![std::borrow::Cow::Borrowed("bar")])
                .into(),
        );
        replace.write().unwrap();

        input_file.assert("1 bar 2\n3 4 5");
    }

    #[test]
    fn test_replace_path() {
        let temp = assert_fs::TempDir::new().unwrap();
        let input_file = temp.child("foo.txt");
        input_file.write_str("foo foo foo").unwrap();

        let primary = typos::report::PrintSilent;
        let replace = Replace::new(&primary);
        replace.report(
            typos::report::PathCorrection::default()
                .path(input_file.path())
                .byte_offset(0)
                .typo("foo")
                .corrections(vec![std::borrow::Cow::Borrowed("bar")])
                .into(),
        );
        replace.write().unwrap();

        input_file.assert(predicates::path::missing());
        temp.child("bar.txt").assert("foo foo foo");
    }
}
