use std::process::Command;

use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn simple() -> Result<()> {
    let mut cmd = Command::cargo_bin("day03")?;
    cmd.args(["-i", "../day03/simple_puzzle.txt"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("4361"))
        .stdout(predicate::str::contains("467835"));
    Ok(())
}

#[test]
fn complex() -> Result<()> {
    let mut cmd = Command::cargo_bin("day03")?;
    cmd.args(["-i", "../day03/puzzle.txt"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("533775"))
        .stdout(predicate::str::contains("78236071"));

    Ok(())
}
