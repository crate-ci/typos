#[test]
#[cfg(feature = "dict")]
fn cli_tests() {
    trycmd::TestCases::new().case("tests/cmd/*.toml");
}
// Appended to crates/typos-cli/tests/cli_tests.rs — step 1: pin CURRENT behaviour.

#[test]
#[cfg(feature = "dict")]
fn extend_exclude_absolute_path_argument() {
    // `extend-exclude` patterns behave differently depending on how the *path
    // argument* is spelled. The matcher is rooted at the literal ".", so for a
    // relative argument the walked candidates happen to line up with the
    // patterns, while for an absolute argument (the same directory!) anchored
    // patterns never match and the exclusion is silently lost (#1075).
    //
    // This test pins the current behaviour; the fix flips the absolute case.
    let temp = assert_fs::TempDir::new().unwrap();
    let root = temp.path();

    std::fs::write(
        root.join("_typos.toml"),
        "[files]\nextend-exclude = [\"/skip\"]\n",
    )
    .unwrap();
    std::fs::create_dir(root.join("skip")).unwrap();
    std::fs::write(root.join("skip").join("data.txt"), "teh quick fox\n").unwrap();
    std::fs::write(root.join("kept.txt"), "teh quick fox\n").unwrap();

    let bin = env!("CARGO_BIN_EXE_typos");

    // Relative argument: the anchored pattern applies, `skip/` is excluded.
    let out = std::process::Command::new(bin)
        .arg("--files")
        .arg(".")
        .current_dir(root)
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("kept.txt"),
        "relative run must list kept.txt, got:\n{stdout}"
    );
    assert!(
        !stdout.contains("data.txt"),
        "relative run must exclude skip/, got:\n{stdout}"
    );

    // Absolute argument, same directory: the exclusion is lost today.
    let out = std::process::Command::new(bin)
        .arg("--files")
        .arg(root)
        .current_dir(root)
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("kept.txt"),
        "absolute run must list kept.txt, got:\n{stdout}"
    );
    assert!(
        stdout.contains("data.txt"),
        "absolute run currently walks into skip/ (#1075), got:\n{stdout}"
    );
}
