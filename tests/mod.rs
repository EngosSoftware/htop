use std::path::{Path, PathBuf};

mod options;

struct TestContext {
  pub current_dir: PathBuf,
  pub expected_pdf: PathBuf,
  pub actual_pdf: PathBuf,
}

macro_rules! test_context {
  () => {{
    let file_name = file!();
    let current_dir = Path::new(file_name).parent().unwrap().to_path_buf();
    let expected_pdf = current_dir.join("expected.pdf");
    let actual_pdf = current_dir.join("actual.pdf");
    TestContext {
      current_dir,
      expected_pdf,
      actual_pdf,
    }
  }};
}

use test_context;
