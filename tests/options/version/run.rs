use super::*;
use assert_cmd::assert::OutputAssertExt;
use assert_cmd::cargo::CommandCargoExt;
use std::process::Command;

#[test]
fn _0001() {
  let mut cmd = Command::cargo_bin("htop").unwrap();
  cmd.arg("-V").assert().success().stdout(EXPECTED_TXT);
}
