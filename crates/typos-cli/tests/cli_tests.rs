#[test]
#[cfg(feature = "dict")]
fn cli_tests() {
    trycmd::TestCases::new().case("tests/cmd/*.toml");
}
