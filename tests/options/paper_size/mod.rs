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
    .arg("-v")
    .arg("--margin=0")
    .arg("--paper-size=212mm,299mm")
    .arg("single")
    .arg("H_000010.html")
    .arg(tc.actual_name())
    .assert()
    .success();
  assert_eq!(
    "[htop] Page size: 212 x 299 mm,  8.3 x 11.8 in, 601.2 x 847.44 pts",
    String::from_utf8(cmd.output().unwrap().stdout)
      .unwrap()
      .lines()
      .filter(|line| line.contains("[htop] Page size:"))
      .collect::<Vec<_>>()
      .join("\n")
  );
  let mut cmd = Command::new("pdfinfo");
  cmd.current_dir(tc.current_dir()).arg(tc.actual_name()).assert().success();
  assert_eq!(
    "Page size:       601.92 x 847.92 pts",
    String::from_utf8(cmd.output().unwrap().stdout)
      .unwrap()
      .lines()
      .filter(|line| line.contains("Page size:"))
      .collect::<Vec<_>>()
      .join("\n")
  );
  tc.tear_down(true);
}

#[test]
fn _0002() {
  let tc = test_context!().set_up().with_suffix("invalid-size");
  let mut cmd = Command::cargo_bin("htop").unwrap();
  cmd
    .current_dir(tc.current_dir())
    .arg("-b")
    .arg("--paper-size=212mm:299mm")
    .arg("url")
    .arg("https://engos.de")
    .arg(tc.actual_name())
    .assert()
    .code(1)
    .stderr("Error: invalid paper size: 212mm:299mm\n");
  tc.tear_down(false);
}

#[test]
fn _0003() {
  let tc = test_context!().set_up().with_suffix("invalid-size");
  let mut cmd = Command::cargo_bin("htop").unwrap();
  cmd
    .current_dir(tc.current_dir())
    .arg("-b")
    .arg("--paper-size=212mm,299")
    .arg("url")
    .arg("https://engos.de")
    .arg(tc.actual_name())
    .assert()
    .code(1)
    .stderr("Error: invalid length: 299\n");
  tc.tear_down(false);
}

#[test]
fn _0004() {
  let tc = test_context!().set_up().with_suffix("invalid-size");
  let mut cmd = Command::cargo_bin("htop").unwrap();
  cmd
    .current_dir(tc.current_dir())
    .arg("-b")
    .arg("--paper-size=212,299mm")
    .arg("url")
    .arg("https://engos.de")
    .arg(tc.actual_name())
    .assert()
    .code(1)
    .stderr("Error: invalid length: 212\n");
  tc.tear_down(false);
}
