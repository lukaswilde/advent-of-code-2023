use std::process::Command;

use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn simple() -> Result<()> {
    let mut cmd = Command::cargo_bin("day02")?;
    cmd.args(["-i", "../day02/simple_puzzle.txt"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("8"))
        .stdout(predicate::str::contains("2286"));
    Ok(())
}

#[test]
fn complex() -> Result<()> {
    let mut cmd = Command::cargo_bin("day02")?;
    cmd.args(["-i", "../day02/puzzle.txt"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("2727"))
        .stdout(predicate::str::contains("56580"));

    Ok(())
}
