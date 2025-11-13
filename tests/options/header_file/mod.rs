use super::*;

#[test]
fn _0001() {
  let tc = test_context!().set_up();
  let mut cmd = tc.command();
  cmd
    .current_dir(tc.current_dir())
    .arg("-b")
    .arg("--header-file=header.html")
    .arg("--print-header-footer")
    .arg("--margin=2.5cm 0 0 0")
    .arg("single")
    .arg("H_000010.html")
    .arg("actual.pdf")
    .assert()
    .success();
  let mut comparison_options = ComparisonOptions::default();
  comparison_options.percentage_limit = tc.percentage_limit().into();
  let file_1 = File::open(tc.expected()).unwrap();
  let file_2 = File::open(tc.actual()).unwrap();
  let comparison_result = compare(file_1, file_2, &comparison_options);
  let test_result = matches!(comparison_result, ComparisonResult::SimilarPercentage(_, _));
  assert!(test_result, "{comparison_result:?}");
  tc.tear_down();
}

#[test]
fn _0002() {
  let tc = test_context!();
  let mut cmd = tc.command();
  cmd
    .current_dir(tc.current_dir)
    .arg("-b")
    .arg("--header-file=non-existing.html")
    .arg("single")
    .arg("H_000010.html")
    .arg("actual.pdf")
    .assert()
    .stderr("Error: reading file non-existing.html failed with reason: No such file or directory (os error 2)\n");
}
