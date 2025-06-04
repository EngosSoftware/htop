use super::*;

/// Tests generating JPEG screenshot when there is no write permission for destination directory.
#[cfg(target_os = "linux")]
#[test]
fn _0001() {
  use std::os::unix::fs::PermissionsExt;
  let tc = test_context!().set_up();
  let dst_path = tc.current_dir().join("dst");
  let permissions = fs::Permissions::from_mode(0o555);
  fs::set_permissions(dst_path, permissions).unwrap();
  tc.command()
    .current_dir(tc.current_dir())
    .arg("--jpeg")
    .arg("single")
    .arg("H_000010.html")
    .arg("./dst/output.jpg")
    .assert()
    .code(1)
    .stdout("")
    .stderr("Error: writing file ./dst/output.jpg failed with reason: Permission denied (os error 13)\n");
  tc.tear_down(false);
}

/// Tests generating PNG screenshot when there is no write permission for destination directory.
#[cfg(target_os = "linux")]
#[test]
fn _0002() {
  use std::os::unix::fs::PermissionsExt;
  let tc = test_context!().set_up();
  let dst_path = tc.current_dir().join("dst");
  let permissions = fs::Permissions::from_mode(0o555);
  fs::set_permissions(dst_path, permissions).unwrap();
  tc.command()
    .current_dir(tc.current_dir())
    .arg("--png")
    .arg("single")
    .arg("H_000010.html")
    .arg("./dst/output.png")
    .assert()
    .code(1)
    .stdout("")
    .stderr("Error: writing file ./dst/output.png failed with reason: Permission denied (os error 13)\n");
  tc.tear_down(false);
}

/// Tests generating WEBP screenshot when there is no write permission for destination directory.
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
    .arg("--webp")
    .arg("single")
    .arg("H_000010.html")
    .arg("./dst/output.webp")
    .assert()
    .code(1)
    .stdout("")
    .stderr("Error: writing file ./dst/output.webp failed with reason: Permission denied (os error 13)\n");
  tc.tear_down(false);
}
