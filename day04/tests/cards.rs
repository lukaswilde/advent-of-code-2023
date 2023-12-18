use std::process::Command;

use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn simple() -> Result<()> {
    let mut cmd = Command::cargo_bin("day04")?;
    cmd.args(["-i", "../day04/simple_puzzle.txt"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("13"))
        .stdout(predicate::str::contains("30"));
    Ok(())
}

#[test]
fn complex() -> Result<()> {
    let mut cmd = Command::cargo_bin("day04")?;
    cmd.args(["-i", "../day04/puzzle.txt"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("27845"))
        .stdout(predicate::str::contains("9496801"));

    Ok(())
}
