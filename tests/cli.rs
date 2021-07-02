use assert_cmd::Command;

#[test]
#[cfg(feature = "dict")]
fn test_stdin_success() {
    let mut cmd = Command::cargo_bin("typos").unwrap();
    cmd.arg("-").write_stdin("Hello world");
    cmd.assert().success();
}

#[test]
#[cfg(feature = "dict")]
fn test_stdin_failure() {
    let mut cmd = Command::cargo_bin("typos").unwrap();
    cmd.arg("-").write_stdin("Apropriate world");
    cmd.assert().code(2);
}

#[test]
#[cfg(feature = "dict")]
fn test_stdin_correct() {
    let mut cmd = Command::cargo_bin("typos").unwrap();
    cmd.arg("-")
        .arg("--write-changes")
        .write_stdin("Apropriate world");
    cmd.assert().success().stdout("Appropriate world");
}

#[test]
#[cfg(feature = "dict")]
fn test_file_failure() {
    let mut cmd = Command::cargo_bin("typos").unwrap();
    cmd.arg("README.md");
    cmd.assert().code(2);
}

#[test]
#[cfg(feature = "dict")]
fn test_relative_dir_failure() {
    let mut cmd = Command::cargo_bin("typos").unwrap();
    cmd.arg(".");
    cmd.assert().code(2);
}

#[test]
#[cfg(feature = "dict")]
fn test_assumed_dir_failure() {
    let mut cmd = Command::cargo_bin("typos").unwrap();
    cmd.assert().code(2);
}
