use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn simple() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day01")?;
    cmd.arg(
        "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
    );
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("142"))
        .stdout(predicate::str::contains("142"));
    Ok(())
}

#[test]
fn simple_part2() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day01")?;
    cmd.arg(
        "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
    );
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("undefined"))
        .stdout(predicate::str::contains("281"));
    Ok(())
}

#[test]
fn complex() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day01")?;
    cmd.args(["-i", "../day01/puzzle.txt"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("55017"))
        .stdout(predicate::str::contains("53539"));

    Ok(())
}
