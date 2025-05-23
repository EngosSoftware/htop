//! # The `htop` application

use htop::cli::get_command;
use htop::errors::Result;

/// Application entrypoint.
fn main() -> Result<()> {
  htop::run(get_command().get_matches())
}
