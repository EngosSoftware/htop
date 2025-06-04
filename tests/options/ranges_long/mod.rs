use super::*;
use assert_cmd::assert::OutputAssertExt;
use assert_cmd::cargo::CommandCargoExt;
use biff::{compare, ComparisonOptions, ComparisonResult};
use std::fs::File;
use std::process::Command;

#[test]
fn _0001() {
  let tc = test_context!().set_up();
  let mut cmd = Command::cargo_bin("htop").unwrap();
  cmd
    .current_dir(tc.current_dir())
    .arg("--ranges")
    .arg("1-2,4,6-9")
    .arg("single")
    .arg("ranges.html")
    .arg("actual.pdf")
    .assert()
    .success();
  let mut comparison_options = ComparisonOptions::default();
  comparison_options.percentage_limit = tc.percentage_limit().into();
  let file_1 = File::open(tc.expected()).unwrap();
  let file_2 = File::open(tc.actual()).unwrap();
  let comparison_result = compare(file_1, file_2, &comparison_options);
  let test_result = matches!(comparison_result, ComparisonResult::SimilarPercentage(_, _));
  tc.tear_down(test_result);
  assert!(test_result, "{comparison_result:?}");
}
