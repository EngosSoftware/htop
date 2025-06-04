use super::*;

#[test]
fn _0001() {
  let tc = test_context!().set_up().with_format("jpg");
  let mut cmd = Command::cargo_bin("htop").unwrap();
  cmd
    .current_dir(tc.current_dir())
    .arg("--jpeg")
    .arg("url")
    .arg("https://engos.de")
    .arg(tc.actual_name())
    .assert()
    .success()
    .stdout("")
    .stderr("");
  tc.assert_similar();
}
