use super::*;

#[test]
fn _0001() {
  let tc = test_context!().set_up();
  let mut cmd = tc.command();
  cmd
    .current_dir(tc.current_dir())
    .arg("--print-background")
    .arg("--margin=0")
    .arg("single")
    .arg("margin.html")
    .arg(tc.actual_name())
    .assert()
    .success()
    .stdout("")
    .stderr("");
  tc.assert_similar();
}

#[test]
fn _0002() {
  let tc = test_context!().set_up().with_suffix("1");
  let mut cmd = tc.command();
  cmd
    .current_dir(tc.current_dir())
    .arg("-b")
    .arg("-m")
    .arg("0")
    .arg("single")
    .arg("margin.html")
    .arg(tc.actual_name())
    .assert()
    .success()
    .stdout("")
    .stderr("");
  tc.assert_similar();
}

#[test]
fn _0003() {
  let tc = test_context!().set_up().with_suffix("1d");
  let mut cmd = tc.command();
  cmd
    .current_dir(tc.current_dir())
    .arg("-b")
    .arg("-m")
    .arg("0.0")
    .arg("single")
    .arg("margin.html")
    .arg(tc.actual_name())
    .assert()
    .success()
    .stdout("")
    .stderr("");
  tc.assert_similar();
}

#[test]
fn _0004() {
  let tc = test_context!().set_up().with_suffix("2");
  let mut cmd = tc.command();
  cmd
    .current_dir(tc.current_dir())
    .arg("-b")
    .arg("-m")
    .arg("0 0")
    .arg("single")
    .arg("margin.html")
    .arg(tc.actual_name())
    .assert()
    .success()
    .stdout("")
    .stderr("");
  tc.assert_similar();
}

#[test]
fn _0005() {
  let tc = test_context!().set_up().with_suffix("2d");
  let mut cmd = tc.command();
  cmd
    .current_dir(tc.current_dir())
    .arg("-b")
    .arg("-m")
    .arg("0.0 0.0")
    .arg("single")
    .arg("margin.html")
    .arg(tc.actual_name())
    .assert()
    .success()
    .stdout("")
    .stderr("");
  tc.assert_similar();
}

#[test]
fn _0006() {
  let tc = test_context!().set_up().with_suffix("3");
  tc.command()
    .current_dir(tc.current_dir())
    .arg("-b")
    .arg("-m")
    .arg("0 0 0")
    .arg("single")
    .arg("margin.html")
    .arg(tc.actual_name())
    .assert()
    .code(1)
    .stdout("")
    .stderr("Error: invalid margin: 0 0 0\n");
  tc.tear_down();
}

#[test]
fn _0007() {
  let tc = test_context!().set_up().with_suffix("3d");
  tc.command()
    .current_dir(tc.current_dir())
    .arg("-b")
    .arg("-m")
    .arg("0.0 0.0 0.0")
    .arg("single")
    .arg("margin.html")
    .arg(tc.actual_name())
    .assert()
    .code(1)
    .stdout("")
    .stderr("Error: invalid margin: 0.0 0.0 0.0\n");
  tc.tear_down();
}

#[test]
fn _0008() {
  let tc = test_context!().set_up().with_suffix("4");
  let mut cmd = tc.command();
  cmd
    .current_dir(tc.current_dir())
    .arg("-b")
    .arg("-m")
    .arg("0 0 0 0")
    .arg("single")
    .arg("margin.html")
    .arg(tc.actual_name())
    .assert()
    .success()
    .stdout("")
    .stderr("");
  tc.assert_similar();
}

#[test]
fn _0009() {
  let tc = test_context!().set_up().with_suffix("4d");
  let mut cmd = tc.command();
  cmd
    .current_dir(tc.current_dir())
    .arg("-b")
    .arg("-m")
    .arg("0.0 0.0 0.0 0.0")
    .arg("single")
    .arg("margin.html")
    .arg(tc.actual_name())
    .assert()
    .success()
    .stdout("")
    .stderr("");
  tc.assert_similar();
}

#[test]
fn _0010() {
  let tc = test_context!().set_up().with_suffix("5");
  tc.command()
    .current_dir(tc.current_dir())
    .arg("-b")
    .arg("-m")
    .arg("0 0 0 0 0")
    .arg("single")
    .arg("margin.html")
    .arg(tc.actual_name())
    .assert()
    .code(1)
    .stdout("")
    .stderr("Error: invalid margin: 0 0 0 0 0\n");
  tc.tear_down();
}

#[test]
fn _0011() {
  let tc = test_context!().set_up().with_suffix("5d");
  tc.command()
    .current_dir(tc.current_dir())
    .arg("-b")
    .arg("-m")
    .arg("0.0 0.0 0.0 0.0 0.0")
    .arg("single")
    .arg("margin.html")
    .arg(tc.actual_name())
    .assert()
    .code(1)
    .stdout("")
    .stderr("Error: invalid margin: 0.0 0.0 0.0 0.0 0.0\n");
  tc.tear_down();
}
