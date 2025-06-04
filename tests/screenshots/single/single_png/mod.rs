use super::*;

#[test]
fn _0001() {
  let tc = test_context!().set_up().with_format("png");
  tc.command()
    .current_dir(tc.current_dir())
    .arg("-P")
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
  let tc = test_context!().set_up().with_format("png");
  tc.command()
    .current_dir(tc.current_dir())
    .arg("--png")
    .arg("single")
    .arg("H_000010.html")
    .assert()
    .success()
    .stdout("")
    .stderr("");
  tc.assert_similar_files(&tc.path("expected.png"), &tc.path("H_000010.png"));
}
