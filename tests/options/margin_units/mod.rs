use super::*;

#[test]
fn _0001() {
  let tc = test_context!().set_up().with_suffix("cm");
  let mut cmd = tc.command();
  cmd
    .current_dir(tc.current_dir())
    .arg("-b")
    .arg("-m")
    .arg("0cm")
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
  let tc = test_context!().set_up().with_suffix("mm");
  let mut cmd = tc.command();
  cmd
    .current_dir(tc.current_dir())
    .arg("-b")
    .arg("-m")
    .arg("0mm")
    .arg("single")
    .arg("margin.html")
    .arg(tc.actual_name())
    .assert()
    .success()
    .stdout("");
  tc.assert_similar();
}

#[test]
fn _0003() {
  let tc = test_context!().set_up().with_suffix("Q");
  let mut cmd = tc.command();
  cmd
    .current_dir(tc.current_dir())
    .arg("-b")
    .arg("-m")
    .arg("0Q")
    .arg("single")
    .arg("margin.html")
    .arg(tc.actual_name())
    .assert()
    .success()
    .stdout("");
  tc.assert_similar();
}

#[test]
fn _0004() {
  let tc = test_context!().set_up().with_suffix("in");
  let mut cmd = tc.command();
  cmd
    .current_dir(tc.current_dir())
    .arg("-b")
    .arg("-m")
    .arg("0in")
    .arg("single")
    .arg("margin.html")
    .arg(tc.actual_name())
    .assert()
    .success()
    .stdout("");
  tc.assert_similar();
}

#[test]
fn _0005() {
  let tc = test_context!().set_up().with_suffix("pc");
  let mut cmd = tc.command();
  cmd
    .current_dir(tc.current_dir())
    .arg("-b")
    .arg("-m")
    .arg("0pc")
    .arg("single")
    .arg("margin.html")
    .arg(tc.actual_name())
    .assert()
    .success()
    .stdout("");
  tc.assert_similar();
}

#[test]
fn _0006() {
  let tc = test_context!().set_up().with_suffix("pt");
  let mut cmd = tc.command();
  cmd
    .current_dir(tc.current_dir())
    .arg("-b")
    .arg("-m")
    .arg("0pt")
    .arg("single")
    .arg("margin.html")
    .arg(tc.actual_name())
    .assert()
    .success()
    .stdout("");
  tc.assert_similar();
}

#[test]
fn _0007() {
  let tc = test_context!().set_up().with_suffix("px");
  let mut cmd = tc.command();
  cmd
    .current_dir(tc.current_dir())
    .arg("-b")
    .arg("-m")
    .arg("0px")
    .arg("single")
    .arg("margin.html")
    .arg(tc.actual_name())
    .assert()
    .success()
    .stdout("");
  tc.assert_similar();
}

#[test]
fn _0008() {
  let tc = test_context!().set_up();
  tc.command()
    .current_dir(tc.current_dir())
    .arg("-b")
    .arg("-m")
    .arg("1a2px")
    .arg("single")
    .arg("margin.html")
    .arg(tc.actual_name())
    .assert()
    .code(1)
    .stdout("")
    .stderr("Error: invalid number: 1a2\n");
  tc.tear_down();
}
