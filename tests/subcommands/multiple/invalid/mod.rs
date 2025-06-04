use super::*;

#[test]
fn _0001() {
  let tc = test_context!().set_up();
  tc.command()
    .current_dir(tc.current_dir())
    .arg("multiple")
    .arg("./a")
    .arg("./b")
    .assert()
    .code(1)
    .stdout("")
    .stderr("Error: reading directory ./a failed with reason: No such file or directory (os error 2)\n");
  tc.tear_down(false);
}

#[test]
fn _0002() {
  let tc = test_context!().set_up();
  tc.command()
    .current_dir(tc.current_dir())
    .arg("multiple")
    .arg("./x")
    .assert()
    .code(1)
    .stdout("")
    .stderr("Error: reading directory ./x failed with reason: No such file or directory (os error 2)\n");
  tc.tear_down(false);
}

#[cfg(target_os = "linux")]
#[test]
fn _0003() {
  use std::os::unix::fs::PermissionsExt;
  let tc = test_context!().set_up();
  let dst_path = tc.current_dir().join("dst");
  let permissions = fs::Permissions::from_mode(0o555);
  fs::set_permissions(dst_path, permissions).unwrap();
  tc.command()
    .current_dir(tc.current_dir())
    .arg("multiple")
    .arg("./src")
    .arg("./dst")
    .assert()
    .code(1)
    .stdout("")
    .stderr("Error: writing file ./dst/H_000010.pdf failed with reason: Permission denied (os error 13)\n");
  tc.tear_down(false);
}
