use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn app_dies_with_no_args() {
    let mut cmd = Command::cargo_bin("recho").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
}

#[test]
fn dies_with_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("recho")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("recho").unwrap();
    cmd.arg("test").assert().success();
}

#[test]
fn hello1_should_be_expected() {
    let outputfile = "tests/expected/hello1.txt";
    let expected = fs::read_to_string(outputfile).unwrap();
    let mut cmd = Command::cargo_bin("recho").unwrap();
    cmd.arg("Hello there").assert().success().stdout(expected);
}

fn run_test(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("recho")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    run_test(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello1_no_newline() -> TestResult {
    run_test(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2() -> TestResult {
    run_test(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello2_no_newline() -> TestResult {
    run_test(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}
