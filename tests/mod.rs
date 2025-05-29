use std::path::{Path, PathBuf};

mod options;

/// Utility function that returns the directory of the file.
fn current_dir(file_name: &str) -> &Path {
  Path::new(file_name).parent().unwrap()
}

/// Utility function that returns the name of the file in current directory.
fn current_file(file_name: &str, append_file_name: &str) -> PathBuf {
  Path::new(file_name).parent().unwrap().join(append_file_name)
}
