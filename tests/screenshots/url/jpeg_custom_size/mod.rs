use super::*;

/// Reasonable window size should work.
#[test]
fn _0001() {
  let tc = test_context!().set_up().with_format("jpg");
  tc.command()
    .current_dir(tc.current_dir())
    .arg("-Jw")
    .arg("1200,1200")
    .arg("url")
    .arg("https://engos.de")
    .arg(tc.actual_name())
    .assert()
    .success()
    .stdout("")
    .stderr("");
  tc.assert_similar();
}
