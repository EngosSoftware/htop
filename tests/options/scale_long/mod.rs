use super::*;

#[test]
fn _0001() {
  let tc = test_context!().set_up();
  tc.command()
    .current_dir(tc.current_dir())
    .arg("-b")
    .arg("--scale=2.0")
    .arg("single")
    .arg("H_000010.html")
    .arg(tc.actual_name())
    .assert()
    .success();
  tc.assert_similar();
}

#[test]
fn _0002() {
  let tc = test_context!().set_up();
  tc.command()
    .current_dir(tc.current_dir())
    .arg("-b")
    .arg("--scale=2a0")
    .arg("single")
    .arg("H_000010.html")
    .arg(tc.actual_name())
    .assert()
    .code(1)
    .stdout("")
    .stderr("Error: invalid number: 2a0\n");
  tc.tear_down();
}
