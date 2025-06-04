use super::*;

#[test]
fn _0001() {
  let tc = test_context!().set_up();
  tc.command().current_dir(tc.current_dir()).arg("url").arg("https://engos.de").assert().success();
  tc.compare_similar_files(&tc.path("expected.pdf"), &tc.path("output.pdf"));
  tc.delete("output.pdf");
  tc.tear_down();
}
