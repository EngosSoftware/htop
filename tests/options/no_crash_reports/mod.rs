use super::*;

#[test]
fn _0001() {
  let tc = test_context!().set_up();
  let mut cmd = tc.command();
  cmd
    .current_dir(tc.current_dir())
    .arg("-bv")
    .arg("--no-crash-reports")
    .arg("--paper-format=A4")
    .arg("single")
    .arg("H_000010.html")
    .arg("actual.pdf")
    .assert()
    .success()
    .stderr("");

  assert_eq!(
    4,
    String::from_utf8(cmd.output().unwrap().stdout)
      .unwrap()
      .lines()
      .filter(|line| line.contains("[htop] "))
      .count()
  );

  let mut comparison_options = ComparisonOptions::default();
  comparison_options.percentage_limit = tc.percentage_limit().into();
  let file_1 = File::open(tc.expected()).unwrap();
  let file_2 = File::open(tc.actual()).unwrap();
  let comparison_result = compare(file_1, file_2, &comparison_options);
  let test_result = matches!(comparison_result, ComparisonResult::SimilarPercentage(_, _));
  assert!(test_result, "{comparison_result:?}");
  tc.tear_down();
}
