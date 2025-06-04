use super::*;

#[test]
fn _0001() {
  let tc = test_context!().set_up();
  let mut cmd = Command::cargo_bin("htop").unwrap();
  cmd
    .current_dir(tc.current_dir())
    .arg("single")
    .arg("H_000010.html")
    .assert()
    .success()
    .stdout("")
    .stderr("");
  tc.compare_similar_files(&tc.path("expected.pdf"), &tc.path("H_000010.pdf"));
  tc.delete("H_000010.pdf");
  tc.tear_down(false);
}
