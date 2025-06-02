use std::fs;
use std::path::{Path, PathBuf};

mod options;

struct TestContext {
  current_dir: PathBuf,
  expected_pdf: PathBuf,
  actual_pdf: PathBuf,
  percentage_limit: f64,
}

impl TestContext {
  pub fn current_dir(&self) -> &PathBuf {
    &self.current_dir
  }

  pub fn expected_pdf(&self) -> &PathBuf {
    &self.expected_pdf
  }

  pub fn actual_pdf(&self) -> &PathBuf {
    &self.actual_pdf
  }

  pub fn percentage_limit(&self) -> f64 {
    self.percentage_limit
  }

  pub fn set_up(self) -> Self {
    self
  }

  pub fn tear_down(self, test_result: bool) -> Self {
    if test_result {
      fs::remove_file(&self.actual_pdf).unwrap()
    }
    self
  }
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
      percentage_limit: 0.06,
    }
  }};
}

use test_context;
