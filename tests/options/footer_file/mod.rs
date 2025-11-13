use super::*;

#[test]
fn _0001() {
  let tc = test_context!().set_up();
  let mut cmd = tc.command();
  cmd
    .current_dir(tc.current_dir())
    .arg("-b")
    .arg("--footer-file=footer.html")
    .arg("--print-header-footer")
    .arg("--margin=0 0 2.5cm 0")
    .arg("single")
    .arg("H_000010.html")
    .arg(tc.actual_name())
    .assert()
    .success();
  tc.assert_similar();
}

#[test]
fn _0002() {
  let tc = test_context!().set_up().with_suffix("non-existing");
  let mut cmd = tc.command();
  cmd
    .current_dir(tc.current_dir())
    .arg("-b")
    .arg("--footer-file=non-existing.html")
    .arg("single")
    .arg("H_000010.html")
    .arg(tc.actual_name())
    .assert()
    .stderr("Error: reading file non-existing.html failed with reason: No such file or directory (os error 2)\n");
  tc.tear_down();
}
