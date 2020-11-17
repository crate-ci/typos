use std::collections::BTreeMap;
use std::sync;

use bstr::ByteSlice;

pub struct Diff<'r> {
    reporter: &'r dyn typos::report::Report,
    deferred: sync::Mutex<crate::replace::Deferred>,
}

impl<'r> Diff<'r> {
    pub(crate) fn new(reporter: &'r dyn typos::report::Report) -> Self {
        Self {
            reporter,
            deferred: sync::Mutex::new(crate::replace::Deferred::default()),
        }
    }

    pub fn show(&self) -> Result<(), std::io::Error> {
        let deferred = self.deferred.lock().unwrap();

        for (path, corrections) in deferred.content.iter() {
            let buffer = std::fs::read(path)?;

            let mut original = Vec::new();
            let mut corrected = Vec::new();
            for (line_idx, line) in buffer.lines_with_terminator().enumerate() {
                original.push(String::from_utf8_lossy(line).into_owned());

                let line_num = line_idx + 1;
                let line = if let Some(corrections) = corrections.get(&line_num) {
                    let line = line.to_vec();
                    crate::replace::correct(line, &corrections)
                } else {
                    line.to_owned()
                };
                corrected.push(String::from_utf8_lossy(&line).into_owned())
            }

            let display_path = path.display().to_string();
            let diff = difflib::unified_diff(
                &original,
                &corrected,
                display_path.as_str(),
                display_path.as_str(),
                "original",
                "corrected",
                0,
            );
            for line in diff {
                print!("{}", line);
            }
        }

        Ok(())
    }
}

impl<'r> typos::report::Report for Diff<'r> {
    fn report(&self, msg: typos::report::Message<'_>) -> Result<(), std::io::Error> {
        let typo = match &msg {
            typos::report::Message::Typo(typo) => typo,
            _ => return self.reporter.report(msg),
        };

        let corrections = match &typo.corrections {
            typos::Status::Corrections(corrections) if corrections.len() == 1 => corrections,
            _ => return self.reporter.report(msg),
        };

        match &typo.context {
            Some(typos::report::Context::File(file)) => {
                let path = file.path.to_owned();
                let line_num = file.line_num;
                let correction = crate::replace::Correction::new(
                    typo.byte_offset,
                    typo.typo,
                    corrections[0].as_ref(),
                );
                let mut deferred = self.deferred.lock().unwrap();
                let content = deferred
                    .content
                    .entry(path)
                    .or_insert_with(BTreeMap::new)
                    .entry(line_num)
                    .or_insert_with(Vec::new);
                content.push(correction);
                Ok(())
            }
            _ => self.reporter.report(msg),
        }
    }
}
