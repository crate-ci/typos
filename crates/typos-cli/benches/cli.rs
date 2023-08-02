pub static CORPUS: &str = include_str!("../../typos-dict/assets/words.csv");

fn setup() {
    std::fs::write("words.csv", CORPUS);
}

iai_callgrind::main!(
    setup = setup;
    run = cmd = env!("CARGO_BIN_EXE_typos-cli"), args = ["words.csv"]
);
