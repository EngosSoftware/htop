use super::*;

/// Tests timeout when converting a single file.
#[test]
fn _0001() {
  let tc = test_context!().set_up();
  tc.command()
    .current_dir(tc.current_dir())
    .arg("-t")
    .arg("500")
    .arg("single")
    .arg("index.html")
    .arg(tc.actual_name())
    .assert()
    .code(1)
    .stdout("")
    .stderr("Error: headless chrome failed with reason: The event waited for never came\n");
  tc.tear_down();
}

/// Tests timeout when converting URL.
#[test]
fn _0002() {
  let tc = test_context!().set_up();
  let url = format!("file://{}/index.html", tc.current_dir().canonicalize().unwrap().to_string_lossy());
  tc.command()
    .current_dir(tc.current_dir())
    .arg("-t")
    .arg("500")
    .arg("url")
    .arg(url)
    .arg(tc.actual_name())
    .assert()
    .code(1)
    .stdout("")
    .stderr("Error: headless chrome failed with reason: The event waited for never came\n");
  tc.tear_down();
}

/// Tests timeout when converting multiple files.
#[test]
fn _0003() {
  let tc = test_context!().set_up();
  tc.command()
    .current_dir(tc.current_dir())
    .arg("-t")
    .arg("500")
    .arg("multiple")
    .arg(".")
    .arg(".")
    .assert()
    .code(1)
    .stdout("")
    .stderr("Error: headless chrome failed with reason: The event waited for never came\n");
  tc.tear_down();
}

/// Tests timeout when taking a screenshot.
#[test]
fn _0004() {
  let tc = test_context!().set_up();
  tc.command()
    .current_dir(tc.current_dir())
    .arg("-J")
    .arg("-t")
    .arg("500")
    .arg("single")
    .arg("index.html")
    .assert()
    .code(1)
    .stdout("")
    .stderr("Error: headless chrome failed with reason: The event waited for never came\n");
  tc.tear_down();
}

#[test]
fn _0005() {
  let tc = test_context!().set_up();
  let mut cmd = tc.command();
  cmd
    .current_dir(tc.current_dir())
    .arg("-t")
    .arg("50s")
    .arg("single")
    .arg("index.html")
    .arg(tc.actual_name())
    .assert()
    .code(1)
    .stdout("")
    .stderr("Error: invalid timeout: 50s\n");
  tc.tear_down();
}

#[test]
fn _0006() {
  let tc = test_context!().set_up();
  let mut cmd = tc.command();
  cmd
    .current_dir(tc.current_dir())
    .arg("--timeout=-20")
    .arg("single")
    .arg("index.html")
    .arg(tc.actual_name())
    .assert()
    .code(1)
    .stdout("")
    .stderr("Error: invalid timeout: -20\n");
  tc.tear_down();
}
