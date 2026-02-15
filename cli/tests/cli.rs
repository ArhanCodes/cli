use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn prints_matches_with_line_numbers() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("a.txt");
    std::fs::write(&file, "hello\nworld\nhello again\n").unwrap();

    let mut cmd = Command::cargo_bin("naan").unwrap();
    cmd.arg("hello").arg(&file);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("1:"))
        .stdout(predicate::str::contains("3:"));
}

#[test]
fn count_works() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("a.txt");
    std::fs::write(&file, "a\nb\na\n").unwrap();

    let mut cmd = Command::cargo_bin("naan").unwrap();
    cmd.args(["-c", "a"]).arg(&file);

    cmd.assert().success().stdout("2\n");
}
