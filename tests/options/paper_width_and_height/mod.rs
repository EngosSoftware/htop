use super::*;
use assert_cmd::assert::OutputAssertExt;
use assert_cmd::cargo::CommandCargoExt;
use std::process::Command;

#[test]
fn _0001() {
  let tc = test_context!().set_up().with_suffix("width-height");
  let mut cmd = Command::cargo_bin("htop").unwrap();
  cmd
    .current_dir(tc.current_dir())
    .arg("-v")
    .arg("--margin=0")
    .arg("--paper-width=212mm")
    .arg("--paper-height=299mm")
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
  tc.tear_down();
}

#[test]
fn _0002() {
  let tc = test_context!().set_up();
  let mut cmd = Command::cargo_bin("htop").unwrap();
  cmd
    .current_dir(tc.current_dir())
    .arg("-p")
    .arg("A4")
    .arg("--paper-height=397")
    .arg("single")
    .arg("H_000010.html")
    .arg(tc.actual_name())
    .assert()
    .code(2)
    .stderr(
      r#"error: the argument '--paper-format <FORMAT>' cannot be used with '--paper-height <PAPER_HEIGHT>'

Usage: htop --paper-format <FORMAT>

For more information, try '--help'.
"#,
    );
  tc.tear_down();
}

#[test]
fn _0003() {
  let tc = test_context!().set_up();
  let mut cmd = Command::cargo_bin("htop").unwrap();
  cmd
    .current_dir(tc.current_dir())
    .arg("-p")
    .arg("A4")
    .arg("--paper-width=310")
    .arg("single")
    .arg("H_000010.html")
    .arg(tc.actual_name())
    .assert()
    .code(2)
    .stderr(
      r#"error: the argument '--paper-format <FORMAT>' cannot be used with '--paper-width <PAPER_WIDTH>'

Usage: htop --paper-format <FORMAT>

For more information, try '--help'.
"#,
    );
  tc.tear_down();
}

#[test]
fn _0004() {
  let tc = test_context!().set_up();
  let mut cmd = Command::cargo_bin("htop").unwrap();
  cmd
    .current_dir(tc.current_dir())
    .arg("--paper-height=297")
    .arg("single")
    .arg("H_000010.html")
    .arg(tc.actual_name())
    .assert()
    .code(2)
    .stderr(
      r#"error: the following required arguments were not provided:
  --paper-width <PAPER_WIDTH>

Usage: htop --paper-height <PAPER_HEIGHT> --paper-width <PAPER_WIDTH>

For more information, try '--help'.
"#,
    );
  tc.tear_down();
}

#[test]
fn _0005() {
  let tc = test_context!().set_up();
  let mut cmd = Command::cargo_bin("htop").unwrap();
  cmd
    .current_dir(tc.current_dir())
    .arg("--paper-width=210")
    .arg("single")
    .arg("H_000010.html")
    .arg(tc.actual_name())
    .assert()
    .code(2)
    .stderr(
      r#"error: the following required arguments were not provided:
  --paper-height <PAPER_HEIGHT>

Usage: htop --paper-width <PAPER_WIDTH> --paper-height <PAPER_HEIGHT>

For more information, try '--help'.
"#,
    );
  tc.tear_down();
}

#[test]
fn _0006() {
  let tc = test_context!().set_up();
  let mut cmd = Command::cargo_bin("htop").unwrap();
  cmd
    .current_dir(tc.current_dir())
    .arg("--paper-width=210mm")
    .arg("--paper-height=2540mm")
    .arg("single")
    .arg("H_000010.html")
    .arg(tc.actual_name())
    .assert()
    .code(1)
    .stderr("Error: paper height is out of range (0.19..100 in): 100.0125 in\n");
  tc.tear_down();
}

#[test]
fn _0007() {
  let tc = test_context!().set_up();
  let mut cmd = Command::cargo_bin("htop").unwrap();
  cmd
    .current_dir(tc.current_dir())
    .arg("--paper-width=210mm")
    .arg("--paper-height=4mm")
    .arg("single")
    .arg("H_000010.html")
    .arg(tc.actual_name())
    .assert()
    .code(1)
    .stderr("Error: paper height is out of range (0.19..100 in): 0.1575 in\n");
  tc.tear_down();
}

#[test]
fn _0008() {
  let tc = test_context!().set_up();
  let mut cmd = Command::cargo_bin("htop").unwrap();
  cmd
    .current_dir(tc.current_dir())
    .arg("--paper-width=210mm")
    .arg("--paper-height=4a1mm")
    .arg("single")
    .arg("H_000010.html")
    .arg(tc.actual_name())
    .assert()
    .code(1)
    .stderr("Error: invalid number: 4a1\n");
  tc.tear_down();
}

#[test]
fn _0009() {
  let tc = test_context!().set_up();
  let mut cmd = Command::cargo_bin("htop").unwrap();
  cmd
    .current_dir(tc.current_dir())
    .arg("--paper-width=2550mm")
    .arg("--paper-height=297mm")
    .arg("single")
    .arg("H_000010.html")
    .arg(tc.actual_name())
    .assert()
    .code(1)
    .stderr("Error: paper width is out of range (0.19..100 in): 100.40625 in\n");
  tc.tear_down();
}

#[test]
fn _0010() {
  let tc = test_context!().set_up();
  let mut cmd = Command::cargo_bin("htop").unwrap();
  cmd
    .current_dir(tc.current_dir())
    .arg("--paper-width=4mm")
    .arg("--paper-height=297mm")
    .arg("single")
    .arg("H_000010.html")
    .arg(tc.actual_name())
    .assert()
    .code(1)
    .stderr("Error: paper width is out of range (0.19..100 in): 0.1575 in\n");
  tc.tear_down();
}

#[test]
fn _0011() {
  let tc = test_context!().set_up();
  let mut cmd = Command::cargo_bin("htop").unwrap();
  cmd
    .current_dir(tc.current_dir())
    .arg("--paper-width=4a1mm")
    .arg("--paper-height=297mm")
    .arg("single")
    .arg("H_000010.html")
    .arg(tc.actual_name())
    .assert()
    .code(1)
    .stderr("Error: invalid number: 4a1\n");
  tc.tear_down();
}
