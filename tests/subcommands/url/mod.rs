use super::*;

mod existing_page;
mod existing_page_no_output_name;

#[test]
fn _0001() {
  let tc = test_context!().set_up();
  tc.command()
    .arg("-J")
    .arg("url")
    .arg("https://www.non-existing-web-page-123456789.com")
    .assert()
    .code(1)
    .stdout("")
    .stderr("Error: headless chrome failed with reason: Navigate failed: net::ERR_NAME_NOT_RESOLVED\n");
  tc.tear_down();
}

#[test]
fn _0002() {
  let tc = test_context!().set_up();
  tc.command()
    .arg("url")
    .arg("https://www.non-existing-web-page-123456789.com")
    .assert()
    .code(1)
    .stdout("")
    .stderr("Error: headless chrome failed with reason: Navigate failed: net::ERR_NAME_NOT_RESOLVED\n");
  tc.tear_down();
}
