use assert_cmd::assert::OutputAssertExt;
use assert_cmd::cargo_bin;
use std::process::Command;

#[test]
fn _0001() {
  let mut cmd = Command::new(cargo_bin!());
  cmd.arg("-V").assert().success().stdout(
    r#"htop 0.3.1
"#,
  );
}
