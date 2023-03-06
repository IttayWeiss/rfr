use std::error::Error;
use assert_cmd::Command;

type TestResult = Result<(), Box<dyn Error>>;

const PRG: &str = "rfr";

#[test]
fn echo_search_phrase() -> TestResult {
    Command::cargo_bin(PRG)?
        .args(&["one", "two", "three"])
        .assert()
        .success()
        .stdout("one two three\n");

    Ok(())
}