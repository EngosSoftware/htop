use super::*;

#[test]
fn _0001() {
  let tc = test_context!().set_up();
  let mut cmd = tc.command();
  cmd
    .current_dir(tc.current_dir())
    .arg("url")
    .arg("https://engos.de")
    .arg(tc.actual_name())
    .assert()
    .success();
  tc.assert_similar();
}
