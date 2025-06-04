use super::*;

#[test]
fn _0001() {
  let tc = test_context!().set_up().with_format("webp");
  let mut cmd = Command::cargo_bin("htop").unwrap();
  cmd
    .current_dir(tc.current_dir())
    .arg("-W")
    .arg("single")
    .arg("H_000010.html")
    .arg(tc.actual_name())
    .assert()
    .success()
    .stdout("")
    .stderr("");
  tc.assert_similar();
}

#[test]
fn _0002() {
  let tc = test_context!().set_up().with_format("webp");
  tc.command()
    .current_dir(tc.current_dir())
    .arg("--webp")
    .arg("single")
    .arg("H_000010.html")
    .assert()
    .success()
    .stdout("")
    .stderr("");
  tc.assert_similar_files(&tc.path("expected.webp"), &tc.path("H_000010.webp"));
}
