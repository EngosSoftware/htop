use super::*;

#[test]
fn _0001() {
  let mut cmd = Command::new(cargo_bin!());
  cmd.assert().success().stdout(
    r#"htop 0.3.1

HTML to PDF converter

htop: missing subcommand
Try 'htop --help' for more information.
"#,
  );
}
