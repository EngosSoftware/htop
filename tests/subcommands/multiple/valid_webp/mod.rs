use super::*;

#[test]
fn _0001() {
  let tc = test_context!().set_up().with_format("jpg");
  tc.command()
    .current_dir(tc.current_dir())
    .arg("--webp")
    .arg("multiple")
    .arg(".")
    .arg(".")
    .assert()
    .success()
    .stdout("")
    .stderr("");
  tc.assert_similar_files(&tc.path("expected-H_000010.webp"), &tc.path("H_000010.webp"));
  tc.assert_similar_files(&tc.path("expected-H_000011.webp"), &tc.path("H_000011.webp"));
  tc.assert_similar_files(&tc.path("expected-H_000020.webp"), &tc.path("H_000020.webp"));
  tc.assert_similar_files(&tc.path("expected-H_000021.webp"), &tc.path("H_000021.webp"));
  tc.tear_down();
}
