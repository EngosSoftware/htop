use super::*;

#[test]
fn _0001() {
  let tc = test_context!().set_up();
  let mut cmd = Command::cargo_bin("htop").unwrap();
  cmd
    .current_dir(tc.current_dir())
    .arg("--print-background")
    .arg("-m")
    .arg("1cm 300px 200pt 15mm")
    .arg("single")
    .arg("margin.html")
    .arg(tc.actual_name())
    .assert()
    .success()
    .stdout("");
  tc.assert_similar();
}

#[test]
fn _0002() {
  let tc = test_context!().set_up();
  let mut cmd = Command::cargo_bin("htop").unwrap();
  cmd
    .current_dir(tc.current_dir())
    .arg("--print-background")
    .arg("-m")
    .arg("1cm 300.0 200pt 15mm")
    .arg("single")
    .arg("margin.html")
    .arg(tc.actual_name())
    .assert()
    .code(1)
    .stdout("")
    .stderr("Error: invalid length: 300.0\n");
  tc.tear_down();
}

#[test]
fn _0003() {
  let tc = test_context!().set_up();
  let mut cmd = Command::cargo_bin("htop").unwrap();
  cmd
    .current_dir(tc.current_dir())
    .arg("--print-background")
    .arg("-m")
    .arg("1cm 3a0.0 200pt 15mm")
    .arg("single")
    .arg("margin.html")
    .arg(tc.actual_name())
    .assert()
    .code(1)
    .stdout("")
    .stderr("Error: invalid number: 3a0.0\n");
  tc.tear_down();
}
