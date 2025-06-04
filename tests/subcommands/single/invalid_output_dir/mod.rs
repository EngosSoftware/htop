use super::*;

#[test]
fn _0001() {
  let tc = test_context!().set_up();
  let mut cmd = Command::cargo_bin("htop").unwrap();
  cmd
    .current_dir(tc.current_dir())
    .arg("single")
    .arg("H_000010.html")
    .arg("out/actual.pdf")
    .assert()
    .code(1)
    .stdout("")
    .stderr("Error: writing file out/actual.pdf failed with reason: No such file or directory (os error 2)\n");
  tc.tear_down();
}
