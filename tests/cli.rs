use assert_cmd::Command;
use predicates::prelude::*;
use std::error::Error;

type TestResult = Result<(), Box<dyn Error>>;

const PRG: &str = "rfr";

#[test]
fn exit_on_no_articles() -> TestResult {
    Command::cargo_bin(PRG)?
        .args(&["being correct", "-l=tests/emptyzbmath.rfr"])
        .assert()
        .success()
        .stdout("No articles found.\n");
    Ok(())
}

#[test]
fn read_non_existent_file() -> TestResult {
    Command::cargo_bin(PRG)?
        .args(&["being correct", "-l=this_file_does_not_exist.never"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("No such file"));
    Ok(())
}

#[test]
fn default_found() -> TestResult {
    Command::cargo_bin(PRG)?
        .args(&["being correct is not enough", "-l=tests/response.rfr"])
        .assert()
        .success()
        .stdout(predicates::str::contains("one article found"));
    Ok(())
}

#[test]
fn default_not_found() -> TestResult {
    Command::cargo_bin(PRG)?
        .args(&["being correct not enough", "-l=tests/response.rfr"])
        .assert()
        .success()
        .stdout(predicates::str::contains("No articles found"));
    Ok(())
}

// todo:
// #[test]
// fn display_articles() -> TestResult {
//     Command::cargo_bin(PRG)?
//         .args(&["linear logic", "-l=tests/test.inhtml"])
//         .assert()
//         .success()
//         .stdout(predicate::str::contains(
//             "4)\tA tableau construction for finite linear-time temporal logic",
//         ));
//     Ok(())
// }

// todo:
// #[test]
// fn too_many_to_display() -> TestResult {
//     Command::cargo_bin(PRG)?
//         .args(&["linear logic", "-l=tests/test.inhtml"])
//         .assert()
//         .success()
//         .stdout(predicate::str::contains("Number of articles exceeds"));
//     Ok(())
// }
// #[test]
//fn exact_title_found() -> TestResult {
//    let expected = fs::read_to_string("tests/exact_match.rfr")?;
//    Command::cargo_bin(PRG)?
//        .args(&[
//            "linear formulas in continuous logic",
//            "-l=tests/test.inhtml",
//            "-e",
//        ])
//        .assert()
//        .success()
//        .stdout(expected);
//    Ok(())
//}
//#[test]
//fn exact_title_not_found() -> TestResult {
//    Command::cargo_bin(PRG)?
//        .args(&["this title does not exist", "-l=tests/test.inhtml", "-e"])
//        .assert()
//        .success()
//        .stdout(predicate::str::contains("no exact match found"));
//    Ok(())
//}

// Todo: Add a test for choose_interactively
