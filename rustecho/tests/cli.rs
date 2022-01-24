use assert_cmd::Command;
use predicates::prelude::*; 
use std::fs;


type TestResult = Result<(), Box<dyn std::error::Error>>;


#[test]
fn dies_no_args() -> TestResult {
    Command::cargo_bin("rustecho")?
         .assert() 
        .failure()
        .stderr(predicate::str::contains("USAGE"));
     Ok(())
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("rustecho")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn hello_one() -> TestResult {
run(&["Hello there"], "/home/linux-pg/Desktop/rustecho/tests/textfiles/hello1.txt")
}

#[test]
fn hello_two() -> TestResult {
    run(&["Hello", "there"], "home/linux-pg/Desktop/rustecho/tests/textfiles/hello2.txt") 
}

#[test]
fn hello_one_no_newline() -> TestResult {
run ( &["Hello there", "-n"], "home/linux-pg/Desktop/rustecho/tests/textfiles/hello1.n.txt")
}


#[test]
fn hello_two_no_newline() -> TestResult {
run(&["-n", "Hello", "there"], "home/linux-pg/Desktop/rustecho/tests/textfiles/hello2.n.txt")
}
