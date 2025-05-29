use super::*;
use assert_cmd::assert::OutputAssertExt;
use assert_cmd::cargo::CommandCargoExt;
use biff::{compare, ComparisonOptions, ComparisonResult};
use std::fs::File;
use std::process::Command;

#[test]
fn _0001() {
  let mut cmd = Command::cargo_bin("htop").unwrap();
  cmd
    .current_dir(current_dir(file!()))
    .arg("-b")
    .arg("--footer=<div style='width:100%; font-size:12pt; font-weight:bold; text-align:center; border: 3px solid green;'>CUSTOM FOOTER</div>")
    .arg("--print-header-footer")
    .arg("--margin=0 0 2.5cm 0")
    .arg("single")
    .arg("H_000010.html")
    .arg("actual.pdf")
    .assert()
    .success();

  let mut comparison_options = ComparisonOptions::default();
  let percentage_limit = 0.03;
  comparison_options.percentage_limit = percentage_limit.into();
  let file_1 = File::open(current_file(file!(), "expected.pdf")).unwrap();
  let file_2 = File::open(current_file(file!(), "actual.pdf")).unwrap();
  let comparison_result = compare(file_1, file_2, &comparison_options);
  assert!(matches!(comparison_result, ComparisonResult::SimilarPercentage(_, _)), "{:?}", comparison_result);
}
