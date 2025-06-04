use super::*;

#[test]
fn _0001() {
  let tc = test_context!().set_up().with_format("jpg");
  tc.command()
    .current_dir(tc.current_dir())
    .arg("--jpeg")
    .arg("multiple")
    .arg(".")
    .arg(".")
    .assert()
    .success()
    .stdout("")
    .stderr("");
  tc.assert_similar_files(&tc.path("expected-H_000010.jpg"), &tc.path("H_000010.jpg"));
  tc.assert_similar_files(&tc.path("expected-H_000011.jpg"), &tc.path("H_000011.jpg"));
  tc.assert_similar_files(&tc.path("expected-H_000020.jpg"), &tc.path("H_000020.jpg"));
  tc.assert_similar_files(&tc.path("expected-H_000021.jpg"), &tc.path("H_000021.jpg"));
  tc.tear_down();
}
