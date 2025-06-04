use super::*;

#[test]
fn _0001() {
  let tc = test_context!().set_up().with_suffix("1");
  let mut cmd = tc.command();
  cmd
    .current_dir(tc.current_dir())
    .arg("--log-level=info")
    .arg("single")
    .arg("H_000010.html")
    .arg(tc.actual_name())
    .assert()
    .success()
    .stdout("");
  assert!(
    String::from_utf8(cmd.output().unwrap().stderr)
      .unwrap()
      .lines()
      .filter(|line| line.contains("Z INFO  "))
      .count()
      > 10
  );
  tc.assert_similar()
}

#[test]
fn _0002() {
  let tc = test_context!().set_up().with_suffix("2");
  let mut cmd = tc.command();
  cmd
    .env("RUST_LOG", "info")
    .current_dir(tc.current_dir())
    .arg("single")
    .arg("H_000010.html")
    .arg(tc.actual_name())
    .assert()
    .success()
    .stdout("");
  assert!(
    String::from_utf8(cmd.output().unwrap().stderr)
      .unwrap()
      .lines()
      .filter(|line| line.contains("Z INFO  "))
      .count()
      > 10
  );
  tc.assert_similar()
}
