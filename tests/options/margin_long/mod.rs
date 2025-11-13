use super::*;

#[test]
fn _0001() {
  let tc = test_context!().set_up();
  let mut cmd = tc.command();
  cmd
    .current_dir(tc.current_dir())
    .arg("--print-background")
    .arg("--margin=1cm 15mm 200pt 300px")
    .arg("single")
    .arg("margin.html")
    .arg("actual.pdf")
    .assert()
    .success()
    .stdout("");
  let mut comparison_options = ComparisonOptions::default();
  comparison_options.percentage_limit = tc.percentage_limit().into();
  let file_1 = File::open(tc.expected()).unwrap();
  let file_2 = File::open(tc.actual()).unwrap();
  let comparison_result = compare(file_1, file_2, &comparison_options);
  let test_result = matches!(comparison_result, ComparisonResult::SimilarPercentage(_, _));
  assert!(test_result, "{comparison_result:?}");
  tc.tear_down();
}
