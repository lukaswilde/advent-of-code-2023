use std::process::Command;

use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn simple() -> Result<()> {
    let mut cmd = Command::cargo_bin("day06")?;
    cmd.args(["-i", "../day06/simple_puzzle.txt"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("288"))
        .stdout(predicate::str::contains("71503"));
    Ok(())
}

#[test]
fn complex() -> Result<()> {
    let mut cmd = Command::cargo_bin("day06")?;
    cmd.args(["-i", "../day06/puzzle.txt"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("449550"))
        .stdout(predicate::str::contains("28360140"));

    Ok(())
}
