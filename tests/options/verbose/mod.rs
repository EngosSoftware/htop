use super::*;

/// Tests the `--verbose` (verbosity) option.
#[test]
fn _0001() {
  let tc = test_context!().set_up().with_suffix("long");
  let mut cmd = tc.command();
  cmd
    .current_dir(tc.current_dir())
    .arg("--verbose")
    .arg("single")
    .arg("H_000010.html")
    .arg(tc.actual_name())
    .assert()
    .success()
    .stderr("");
  assert_eq!(1, tc.stdout_lines(&mut cmd).iter().filter(|line| line.contains("[htop] Printing completed:")).count());
  tc.assert_similar();
}

/// Tests the `-v` (verbosity) option.
#[test]
fn _0002() {
  let tc = test_context!().set_up().with_suffix("short");
  let mut cmd = tc.command();
  cmd
    .current_dir(tc.current_dir())
    .arg("-v")
    .arg("single")
    .arg("H_000010.html")
    .arg(tc.actual_name())
    .assert()
    .success()
    .stderr("");
  assert_eq!(1, tc.stdout_lines(&mut cmd).iter().filter(|line| line.contains("[htop] Printing completed:")).count());
  tc.assert_similar();
}
