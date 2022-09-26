use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs
use std::env;

#[test]
fn help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    cmd.arg("-h");
    cmd.assert().success();

    Ok(())
}

#[test]
#[cfg(target_os = "windows")]
fn inherit_env() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    let var_name = "variableA";
    let var_value = "Hellow";
    env::set_var(var_name, var_value);

    cmd.arg("-c")
       .arg("echo $env:variableA")
       .arg("echo")
       .assert()
       .stdout(predicate::str::contains(var_value));

    Ok(())
}
