use std::process::Command;

use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn simple() -> Result<()> {
    let mut cmd = Command::cargo_bin("day05")?;
    cmd.args(["-i", "../day05/simple_puzzle.txt"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("35"))
        .stdout(predicate::str::contains("46"));
    Ok(())
}

#[test]
fn complex() -> Result<()> {
    let mut cmd = Command::cargo_bin("day05")?;
    cmd.args(["-i", "../day05/puzzle.txt"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("107430936"))
        .stdout(predicate::str::contains("23738616"));

    Ok(())
}
