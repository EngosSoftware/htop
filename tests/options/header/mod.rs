use super::*;
use assert_cmd::assert::OutputAssertExt;
use assert_cmd::cargo::CommandCargoExt;
use std::process::Command;

#[test]
fn _0001() {
  let tc = test_context!().set_up();
  let mut cmd = Command::cargo_bin("htop").unwrap();
  cmd
    .current_dir(tc.current_dir())
    .arg("-b")
    .arg("--header=<div style='width:100%; font-size:12pt; font-weight:bold; text-align:center; border: 3px solid red;'>CUSTOM HEADER</div>")
    .arg("--print-header-footer")
    .arg("--margin=2.5cm 0 0 0")
    .arg("single")
    .arg("H_000010.html")
    .arg(tc.actual_name())
    .assert()
    .success();
  tc.assert_similar();
}
