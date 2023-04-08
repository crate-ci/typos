use bstr::ByteSlice;
use tower_lsp::lsp_types::*;
use typos_cli::*;

// mimics typos_cli::file::FileChecker::check_file

pub(crate) fn check_text(buffer: &str, policy: &policy::Policy) -> Vec<Diagnostic> {
    // TODO: check filenames

    let mut accum = AccumulatePosition::new();

    // TODO: support ignores

    typos::check_str(buffer, policy.tokenizer, policy.dict)
        .map(|typo| {
            tracing::debug!("typo: {:?}", typo);

            let (line_num, line_pos) = accum.pos(buffer.as_bytes(), typo.byte_offset);

            Diagnostic::new(
                Range::new(
                    Position::new(line_num as u32, line_pos as u32),
                    Position::new(line_num as u32, (line_pos + typo.typo.len()) as u32),
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
        })
        .collect()
}

struct AccumulatePosition {
    line_num: usize,
    line_pos: usize,
    last_offset: usize,
}

impl AccumulatePosition {
    fn new() -> Self {
        Self {
            // LSP ranges are 0-indexed see https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#range
            line_num: 0,
            line_pos: 0,
            last_offset: 0,
        }
    }

    fn pos(&mut self, buffer: &[u8], byte_offset: usize) -> (usize, usize) {
        assert!(self.last_offset <= byte_offset);
        let slice = &buffer[self.last_offset..byte_offset];
        let newlines = slice.find_iter(b"\n").count();
        let line_num = self.line_num + newlines;

        let line_start = buffer[0..byte_offset]
            .rfind_byte(b'\n')
            // Skip the newline
            .map(|s| s + 1)
            .unwrap_or(0);

        self.line_num = line_num;
        self.line_pos = byte_offset - line_start;
        self.last_offset = byte_offset;

        (self.line_num, self.line_pos)
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
