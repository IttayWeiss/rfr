use std::error::Error;
use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn Error>>;

const PRG: &str = "rfr";

//#[test]
//fn echo_search_phrase() -> TestResult {
//    Command::cargo_bin(PRG)?
//        .args(&["one", "two", "three", "-l=tests/test.html"])
//        .assert()
//        .success()
//        .stdout("one two three\n");
//    Ok(())
//}

#[test]
fn read_non_existent_file() -> TestResult {
    Command::cargo_bin(PRG)?
        .args(&["linear logic", "-l=this_file_does_not_exist.never"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("No such file"));
    Ok(())
}

#[test]
fn display_articles() -> TestResult {
    Command::cargo_bin(PRG)?
        .args(&["linear logic", "-l=tests/test.html"])
        .assert()
        .success()
        .stdout(predicate::str::contains("4)\tA tableau construction for finite linear-time temporal logic"));
    Ok(())
}

#[test]
fn exit_on_no_articles() -> TestResult {
    Command::cargo_bin(PRG)?
        .args(&["linear logic", "-l=tests/emptyzbmath.html"])
        .assert()
        .success()
        .stdout("No articles found.\n");
    Ok(())
}

#[test]
fn too_many_to_display() -> TestResult {
    Command::cargo_bin(PRG)?
        .args(&["linear logic", "-l=tests/test.html"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Number of articles exceeds"));
    Ok(())
}