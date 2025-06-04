mod derived_output_name;
mod invalid_output_dir;
mod invalid_output_name;

use super::*;

#[test]
fn _0001() {
  let tc = test_context!().set_up();
  tc.command()
    .current_dir(tc.current_dir())
    .arg("single")
    .arg("non-existing.html")
    .assert()
    .code(1)
    .stdout("")
    .stderr("Error: canonicalizing failed for path non-existing.html with reason: No such file or directory (os error 2)\n");
  tc.tear_down();
}
