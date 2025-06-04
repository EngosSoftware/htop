use super::*;

/// Tests the `--landscape` option (landscape paper orientation).
#[test]
fn _0001() {
  let tc = test_context!().set_up().with_suffix("long");
  tc.command()
    .current_dir(tc.current_dir())
    .arg("-b")
    .arg("--landscape")
    .arg("single")
    .arg("H_000010.html")
    .arg(tc.actual_name())
    .assert()
    .success()
    .stdout("")
    .stderr("");
  tc.assert_similar();
}

/// Tests the `-l` option (landscape paper orientation).
#[test]
fn _0002() {
  let tc = test_context!().set_up().with_suffix("short");
  tc.command()
    .current_dir(tc.current_dir())
    .arg("-b")
    .arg("-l")
    .arg("single")
    .arg("H_000010.html")
    .arg(tc.actual_name())
    .assert()
    .success()
    .stdout("")
    .stderr("");
  tc.assert_similar();
}
