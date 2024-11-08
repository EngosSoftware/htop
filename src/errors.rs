//! # Error definitions

use crate::defs::{MAX_PAPER_LENGTH, MIN_PAPER_LENGTH};
use std::fmt;
use std::path::Path;

/// Common result type.
pub type Result<T, E = HtopError> = std::result::Result<T, E>;

/// Common error definition.
pub struct HtopError(String);

impl fmt::Debug for HtopError {
  /// Implements [Debug](fmt::Debug) trait for [HtopError].
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl HtopError {
  /// Creates a new [HtopError] with specified message.
  pub fn new(message: String) -> Self {
    Self(message)
  }
}

/// Creates invalid paper format error.
pub fn err_invalid_paper_format(format_name: &str) -> HtopError {
  HtopError::new(format!("invalid paper format '{format_name}'"))
}

/// Creates invalid paper width error.
pub fn err_invalid_paper_width(paper_width: f64) -> HtopError {
  HtopError::new(format!("paper width is out of range ({MIN_PAPER_LENGTH}..{MAX_PAPER_LENGTH} in): {paper_width} in"))
}

/// Creates invalid paper height error.
pub fn err_invalid_paper_height(paper_height: f64) -> HtopError {
  HtopError::new(format!("paper height is out of range ({MIN_PAPER_LENGTH}..{MAX_PAPER_LENGTH} in): {paper_height} in"))
}

/// Creates an error with failure reason message from headless chrome.
pub fn err_headless_chrome(reason: String) -> HtopError {
  HtopError::new(format!("headless chrome failed with reason: {reason}"))
}

/// Creates an error with file writing failure reason.
pub fn err_write_file(file_name: &str, reason: String) -> HtopError {
  HtopError::new(format!("writing file {file_name} failed with reason: {reason}"))
}

/// Creates an error with file reading failure reason.
pub fn err_read_file(file_name: &str, reason: String) -> HtopError {
  HtopError::new(format!("reading file {file_name} failed with reason: {reason}"))
}

/// Creates an error when canonicalizing a path fails.
pub fn err_canonicalize(path: &Path, reason: String) -> HtopError {
  let path = path.to_string_lossy();
  HtopError::new(format!("canonicalizing failed for path {path} with reason: {reason}"))
}

/// Creates an error when retrieving file name fails.
pub fn err_file_name(path: &Path) -> HtopError {
  let path = path.to_string_lossy();
  HtopError::new(format!("retrieving file name for path {path} failed"))
}

/// Creates an error when invalid length was encountered.
pub fn err_invalid_length(length: &str) -> HtopError {
  HtopError::new(format!("invalid length: {length}"))
}

/// Creates an error when invalid number was encountered.
pub fn err_invalid_number(number: &str) -> HtopError {
  HtopError::new(format!("invalid number: {number}"))
}

/// Creates an error when invalid timeout was encountered.
pub fn err_invalid_timeout(timeout: &str) -> HtopError {
  HtopError::new(format!("invalid timeout: {timeout}"))
}

/// Creates an error when invalid width was encountered.
pub fn err_invalid_width(width: &str) -> HtopError {
  HtopError::new(format!("invalid width: {width}"))
}

/// Creates an error when invalid height was encountered.
pub fn err_invalid_height(height: &str) -> HtopError {
  HtopError::new(format!("invalid height: {height}"))
}

/// Creates an error when invalid margin was encountered.
pub fn err_invalid_margin(margin: &str) -> HtopError {
  HtopError::new(format!("invalid margin: {margin}"))
}

/// Creates an error when invalid window size was encountered.
pub fn err_invalid_window_size(window_size: &str) -> HtopError {
  HtopError::new(format!("invalid window size: {window_size}"))
}
