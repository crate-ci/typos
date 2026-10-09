#[test]
#[cfg(feature = "dict")]
fn cli_tests() {
    trycmd::TestCases::new()
        .env("LC_ALL", "C.UTF-8")
        .case("tests/cmd/*.toml");
}
