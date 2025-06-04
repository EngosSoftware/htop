use assert_cmd::assert::OutputAssertExt;
use assert_cmd::cargo::CommandCargoExt;
use biff::{compare, ComparisonOptions, ComparisonResult};
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Command;

mod options;
mod paper_formats;
mod screenshots;
mod subcommands;

const DEFAULT_FORMAT: &str = "pdf";
const EXPECTED: &str = "expected";
const ACTUAL: &str = "actual";

struct TestContext {
  file_format: String,
  file_suffix: String,
  current_dir: PathBuf,
  expected: PathBuf,
  actual_name: String,
  actual: PathBuf,
  percentage_limit: f64,
}

impl TestContext {
  /// Returns the relative path to current directory.
  pub fn current_dir(&self) -> &PathBuf {
    &self.current_dir
  }

  /// Returns the relative path to expected file.
  pub fn expected(&self) -> &PathBuf {
    &self.expected
  }

  /// Returns the name of the actual file.
  pub fn actual_name(&self) -> &str {
    &self.actual_name
  }

  /// Returns the relative path to actual file.
  pub fn actual(&self) -> &PathBuf {
    &self.actual
  }

  /// Returns percentage limit for binary file comparison.
  pub fn percentage_limit(&self) -> f64 {
    self.percentage_limit
  }

  /// Initializes the testing context.
  pub fn set_up(self) -> Self {
    self
  }

  /// Sets the name suffix for the actual output file.
  pub fn with_suffix(mut self, suffix: &str) -> Self {
    self.file_suffix = if suffix.is_empty() { "".to_string() } else { format!("-{}", suffix) };
    self.actual_name = format!("{ACTUAL}{}.{}", self.file_suffix, self.file_format);
    self.actual = self.current_dir.join(&self.actual_name);
    self
  }

  /// Sets the name suffix for the actual output file.
  pub fn with_format(mut self, format: &str) -> Self {
    self.file_format = format.to_string();
    let expected_name = format!("{EXPECTED}{}.{}", self.file_suffix, self.file_format);
    self.expected = self.current_dir.join(expected_name);
    self.actual = self.current_dir.join(&self.actual_name);
    self.actual_name = format!("{ACTUAL}{}.{}", self.file_suffix, self.file_format);
    self.actual = self.current_dir.join(&self.actual_name);
    self
  }

  /// Finalizes the testing context.
  pub fn tear_down(&self) {
    if fs::exists(&self.actual).unwrap() {
      fs::remove_file(&self.actual).unwrap()
    }
  }

  /// Compares default files for similarity.
  pub fn compare_similar(&self) -> (bool, ComparisonResult) {
    self.compare_similar_files(self.expected(), self.actual())
  }

  /// Compares expected and actual files for similarity.
  pub fn compare_similar_files(&self, expected: &PathBuf, actual: &PathBuf) -> (bool, ComparisonResult) {
    let mut comparison_options = ComparisonOptions::default();
    comparison_options.percentage_limit = Some(self.percentage_limit);
    let file_1 = File::open(expected).unwrap();
    let file_2 = File::open(actual).unwrap();
    let comparison_result = compare(file_1, file_2, &comparison_options);
    let test_result = matches!(comparison_result, ComparisonResult::SimilarPercentage(_, _));
    (test_result, comparison_result)
  }

  /// Asserts that expected an actual PDF files are similar.
  pub fn assert_similar(&self) {
    let (test_result, comparison_result) = self.compare_similar();
    assert!(test_result, "{comparison_result:?}");
    self.tear_down();
  }

  /// Asserts that expected an actual PDF files are similar.
  pub fn assert_similar_files(&self, expected: &PathBuf, actual: &PathBuf) {
    let (test_result, comparison_result) = self.compare_similar_files(expected, actual);
    assert!(test_result, "{comparison_result:?}");
    self.tear_down();
    if fs::exists(actual).unwrap() {
      fs::remove_file(actual).unwrap()
    }
  }

  /// Returns path to specified file.
  pub fn path(&self, file_name: &str) -> PathBuf {
    self.current_dir.join(file_name)
  }

  /// Deletes specified file.
  pub fn delete(&self, file_name: &str) {
    let _ = fs::remove_file(self.current_dir.join(file_name));
  }

  /// Returns the default command.
  pub fn command(&self) -> Command {
    Command::cargo_bin("htop").unwrap()
  }

  /// Returns the page size retrieved using `pdfinfo` tool.
  pub fn pdf_info_page_size(&self) -> String {
    let mut cmd = Command::new("pdfinfo");
    cmd.current_dir(self.current_dir()).arg(self.actual_name()).assert().success();
    String::from_utf8(cmd.output().unwrap().stdout)
      .unwrap()
      .lines()
      .filter(|line| line.contains("Page size:"))
      .collect::<Vec<_>>()
      .join("\n")
  }

  /// Returns the lines from `stdout`.
  pub fn stdout_lines(&self, command: &mut Command) -> Vec<String> {
    String::from_utf8(command.output().unwrap().stdout)
      .unwrap()
      .lines()
      .map(|line| line.to_string())
      .collect::<Vec<_>>()
  }
}

macro_rules! test_context {
  () => {{
    let file_name = file!();
    let current_dir = Path::new(file_name).parent().unwrap().to_path_buf();
    let expected_name = format!("{EXPECTED}.{DEFAULT_FORMAT}");
    let expected = current_dir.join(&expected_name);
    let actual_name = format!("{ACTUAL}.{DEFAULT_FORMAT}");
    let actual = current_dir.join(&actual_name);
    TestContext {
      file_format: DEFAULT_FORMAT.to_string(),
      file_suffix: "".to_string(),
      current_dir,
      expected,
      actual_name,
      actual,
      percentage_limit: 0.2,
    }
  }};
}

use test_context;
