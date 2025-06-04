use super::*;

#[test]
fn _0001() {
  let tc = test_context!().set_up();
  tc.command()
    .current_dir(tc.current_dir())
    .arg("single")
    .arg("H_000010.html")
    .arg("/")
    .assert()
    .code(1)
    .stdout("")
    .stderr("Error: writing file / failed with reason: Is a directory (os error 21)\n");
  tc.tear_down();
}
