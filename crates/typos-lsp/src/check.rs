use typos_cli::*;
use bstr::ByteSlice;

pub(crate) fn check_file(
    path: &std::path::Path,
    explicit: bool,
    policy: &policy::Policy,
    reporter: &dyn report::Report,
) -> Result<(), std::io::Error> {
    if policy.check_files {
        let (buffer, content_type) = read_file(path, reporter)?;
        if !explicit && !policy.binary && content_type.is_binary() {
            let msg = report::BinaryFile { path };
            reporter.report(msg.into())?;
        } else {
            let mut accum_line_num = AccumulateLineNum::new();
            let mut ignores: Option<Ignores> = None;
            for typo in typos::check_bytes(&buffer, policy.tokenizer, policy.dict) {
                if ignores
                    .get_or_insert_with(|| Ignores::new(&buffer, policy.ignore))
                    .is_ignored(typo.span())
                {
                    continue;
                }
                let line_num = accum_line_num.line_num(&buffer, typo.byte_offset);
                let (line, line_offset) = extract_line(&buffer, typo.byte_offset);
                let msg = report::Typo {
                    context: Some(report::FileContext { path, line_num }.into()),
                    buffer: std::borrow::Cow::Borrowed(line),
                    byte_offset: line_offset,
                    typo: typo.typo.as_ref(),
                    corrections: typo.corrections,
                };
                reporter.report(msg.into())?;
            }
        }
    }

    Ok(())
}


fn read_file(
    path: &std::path::Path,
    reporter: &dyn report::Report,
) -> Result<(Vec<u8>, content_inspector::ContentType), std::io::Error> {
    let buffer = if path == std::path::Path::new("-") {
        let mut buffer = Vec::new();
        report_result(std::io::stdin().read_to_end(&mut buffer), reporter)?;
        buffer
    } else {
        report_result(std::fs::read(path), reporter)?
    };

    let content_type = content_inspector::inspect(&buffer);

    let (buffer, content_type) = match content_type {
        content_inspector::ContentType::BINARY |
        // HACK: We don't support UTF-32 yet
        content_inspector::ContentType::UTF_32LE |
        content_inspector::ContentType::UTF_32BE => {
            (buffer, content_inspector::ContentType::BINARY)
        },
        content_inspector::ContentType::UTF_8 |
        content_inspector::ContentType::UTF_8_BOM => {
            (buffer, content_type)
        },
        content_inspector::ContentType::UTF_16LE => {
            let buffer = report_result(encoding::all::UTF_16LE.decode(&buffer, encoding::DecoderTrap::Strict), reporter)?;
            (buffer.into_bytes(), content_type)
        }
        content_inspector::ContentType::UTF_16BE => {
            let buffer = report_result(encoding::all::UTF_16BE.decode(&buffer, encoding::DecoderTrap::Strict), reporter)?;
            (buffer.into_bytes(), content_type)
        },
    };

    Ok((buffer, content_type))
}

struct AccumulateLineNum {
    line_num: usize,
    last_offset: usize,
}

impl AccumulateLineNum {
    fn new() -> Self {
        Self {
            // 1-indexed
            line_num: 1,
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
