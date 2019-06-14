#[derive(Copy, Clone, Debug, Serialize)]
pub struct Message<'m> {
    pub path: &'m std::path::Path,
    #[serde(skip)]
    pub line: &'m [u8],
    pub line_num: usize,
    pub col_num: usize,
    pub word: &'m str,
    pub correction: &'m str,
    #[serde(skip)]
    pub(crate) non_exhaustive: (),
}

pub type Report = fn(msg: Message);

pub fn print_silent(_: Message) {}

pub fn print_brief(msg: Message) {
    println!(
        "{}:{}:{}: {} -> {}",
        msg.path.display(),
        msg.line_num,
        msg.col_num,
        msg.word,
        msg.correction
    );
}

pub fn print_long(msg: Message) {
    let line_num = msg.line_num.to_string();
    let line_indent: String = itertools::repeat_n(" ", line_num.len()).collect();

    let hl_indent: String = itertools::repeat_n(" ", msg.col_num).collect();
    let hl: String = itertools::repeat_n("^", msg.word.len()).collect();

    println!("error: `{}` should be `{}`", msg.word, msg.correction);
    println!(
        "  --> {}:{}:{}",
        msg.path.display(),
        msg.line_num,
        msg.col_num
    );
    println!("{} |", line_indent);
    println!(
        "{} | {}",
        msg.line_num,
        String::from_utf8_lossy(msg.line).trim_end()
    );
    println!("{} | {}{}", line_indent, hl_indent, hl);
    println!("{} |", line_indent);
}

pub fn print_json(msg: Message) {
    println!("{}", serde_json::to_string(&msg).unwrap());
}
