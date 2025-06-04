use assert_cmd::assert::OutputAssertExt;
use assert_cmd::cargo::CommandCargoExt;
use std::process::Command;

#[test]
fn _0001() {
  let mut cmd = Command::cargo_bin("htop").unwrap();
  cmd.assert().success().stdout(
    r#"htop 0.3.0

HTML to PDF converter

htop: missing subcommand
Try 'htop --help' for more information.
"#,
  );
}
