use super::*;

#[test]
fn _0001() {
  let tc = test_context!().set_up();
  tc.command()
    .current_dir(tc.current_dir())
    .arg("--margin=0")
    .arg("--paper-format=A3")
    .arg("single")
    .arg("H_000010.html")
    .arg(tc.actual_name())
    .assert()
    .success()
    .stdout("")
    .stderr("");
  assert_eq!("Page size:       841.92 x 1191.12 pts (A3)", tc.pdf_info_page_size());
  tc.tear_down();
}
