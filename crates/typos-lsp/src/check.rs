use bstr::ByteSlice;
use tower_lsp::lsp_types::*;
use typos_cli::*;

// mimics typos_cli::file::FileChecker::check_file

pub(crate) fn check_text(buffer: &str, policy: &policy::Policy) -> Vec<Diagnostic> {
    // TODO: check filenames

    let mut accum_line_num = AccumulateLineNum::new();

    // TODO: ignores

    for typo in typos::check_str(buffer, policy.tokenizer, policy.dict) {
        let line_num = accum_line_num.line_num(buffer.as_bytes(), typo.byte_offset);
        let (line, line_offset) = extract_line(buffer.as_bytes(), typo.byte_offset);
    }

    typos::check_str(buffer, policy.tokenizer, policy.dict)
        .map(|typo| {
            tracing::info!("typo: {:?}", typo);

            let line_num = accum_line_num.line_num(buffer.as_bytes(), typo.byte_offset);
            let (line, line_offset) = extract_line(buffer.as_bytes(), typo.byte_offset);

            Diagnostic::new(
                Range::new(
                    Position::new((line_num - 1) as u32, line_offset as u32),
                    Position::new(
                        (line_num - 1) as u32,
                        (line_offset + typo.typo.len()) as u32,
                    ),
                ),
                Some(DiagnosticSeverity::WARNING),
                None,
                Some(env!("CARGO_PKG_NAME").to_string()),
                match typo.corrections {
                    typos::Status::Invalid => format!("`{}` is disallowed", typo.typo),
                    typos::Status::Corrections(corrections) => format!(
                        "`{}` should be {}",
                        typo.typo,
                        itertools::join(corrections.iter().map(|s| format!("`{}`", s)), ", ")
                    ),
                    typos::Status::Valid => panic!("unexpected typos::Status::Valid"),
                },
                None,
                None,
            )
        }).collect()

}

struct AccumulateLineNum {
    line_num: usize,
    line_offset: usize,
    last_offset: usize,
}

impl AccumulateLineNum {
    fn new() -> Self {
        Self {
            // 1-indexed
            line_num: 1,
            line_offset: 1,
            last_offset: 0,
        }
    }

    fn line_num(&mut self, buffer: &[u8], byte_offset: usize) -> usize {
        assert!(self.last_offset <= byte_offset);
        let slice = &buffer[self.last_offset..byte_offset];
        let newlines = slice.find_iter(b"\n").count();
        let line_num = self.line_num + newlines;
        self.line_num = line_num;
        self.last_offset = byte_offset;
        line_num
    }
}

#[derive(Clone, Debug)]
struct Ignores {
    blocks: Vec<std::ops::Range<usize>>,
}

impl Ignores {
    fn new(content: &[u8], ignores: &[regex::Regex]) -> Self {
        let mut blocks = Vec::new();
        if let Ok(content) = std::str::from_utf8(content) {
            for ignore in ignores {
                for mat in ignore.find_iter(content) {
                    blocks.push(mat.range());
                }
            }
        }
        Self { blocks }
    }

    fn is_ignored(&self, span: std::ops::Range<usize>) -> bool {
        let start = span.start;
        let end = span.end.saturating_sub(1);
        self.blocks
            .iter()
            .any(|block| block.contains(&start) || block.contains(&end))
    }
}

fn extract_line(buffer: &[u8], byte_offset: usize) -> (&[u8], usize) {
    let line_start = buffer[0..byte_offset]
        .rfind_byte(b'\n')
        // Skip the newline
        .map(|s| s + 1)
        .unwrap_or(0);
    let line = buffer[line_start..]
        .lines()
        .next()
        .expect("should always be at least a line");
    let line_offset = byte_offset - line_start;
    (line, line_offset)
}
