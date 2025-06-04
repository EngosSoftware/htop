use super::*;

#[test]
fn _0001() {
  let tc = test_context!().set_up();
  let mut cmd = Command::cargo_bin("htop").unwrap();
  cmd
    .current_dir(tc.current_dir())
    .arg("-p")
    .arg("P23")
    .arg("single")
    .arg("H_000010.html")
    .arg(tc.actual_name())
    .assert()
    .code(1)
    .stderr("Error: invalid paper format 'P23'\n");
  tc.tear_down();
}
