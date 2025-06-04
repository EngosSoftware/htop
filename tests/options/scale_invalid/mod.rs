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
    .arg("--scale=2a0%")
    .arg("single")
    .arg("H_000010.html")
    .arg("actual.pdf")
    .assert()
    .stderr("Error: invalid number: 2a0\n");
}

#[test]
fn _0002() {
  let tc = test_context!().set_up();
  let mut cmd = Command::cargo_bin("htop").unwrap();
  cmd
    .current_dir(tc.current_dir())
    .arg("-b")
    .arg("--scale=800%")
    .arg("single")
    .arg("H_000010.html")
    .arg("actual.pdf")
    .assert()
    .stderr("Error: headless chrome failed with reason: Method call error -32602: scale is outside of [0.1 - 2] range\n");
}

#[test]
fn _0003() {
  let tc = test_context!().set_up();
  let mut cmd = Command::cargo_bin("htop").unwrap();
  cmd
    .current_dir(tc.current_dir())
    .arg("-b")
    .arg("--scale=8")
    .arg("single")
    .arg("H_000010.html")
    .arg("actual.pdf")
    .assert()
    .stderr("Error: headless chrome failed with reason: Method call error -32602: scale is outside of [0.1 - 2] range\n");
}

#[test]
fn _0004() {
  let tc = test_context!().set_up();
  let mut cmd = Command::cargo_bin("htop").unwrap();
  cmd
    .current_dir(tc.current_dir())
    .arg("-b")
    .arg("--scale=0.09")
    .arg("single")
    .arg("H_000010.html")
    .arg("actual.pdf")
    .assert()
    .stderr("Error: headless chrome failed with reason: Method call error -32602: scale is outside of [0.1 - 2] range\n");
}

#[test]
fn _0005() {
  let tc = test_context!().set_up();
  let mut cmd = Command::cargo_bin("htop").unwrap();
  cmd
    .current_dir(tc.current_dir())
    .arg("-b")
    .arg("--scale=9%")
    .arg("single")
    .arg("H_000010.html")
    .arg("actual.pdf")
    .assert()
    .stderr("Error: headless chrome failed with reason: Method call error -32602: scale is outside of [0.1 - 2] range\n");
}
