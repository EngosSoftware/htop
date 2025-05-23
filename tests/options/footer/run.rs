use assert_cmd::assert::OutputAssertExt;
use assert_cmd::cargo::CommandCargoExt;
use std::process::Command;

#[test]
fn _0001() {
  let mut cmd = Command::cargo_bin("htop").unwrap();
  cmd
    .current_dir("tests/options/footer")
    .arg("-b")
    .arg("--footer=<div style='width:100%; font-size:12pt; font-weight:bold; text-align:center; border: 3px solid green;'>CUSTOM FOOTER</div>")
    .arg("--print-header-footer")
    .arg("--margin=0 0 2.5cm 0")
    .arg("single")
    .arg("H_000010.html")
    .arg("actual.pdf")
    .assert()
    .success();
  // let actual = fs::read("tests/options/footer/actual.pdf").unwrap();
  // let expected = fs::read("tests/options/footer/expected.pdf").unwrap();
}
