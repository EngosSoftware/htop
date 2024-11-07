//! # Common definitions

use crate::errors::*;
use crate::units::inches;
use std::env;
use std::path::Path;

/// Type alias for a collection of file tuples.
pub type Files = Vec<(String, String)>;

/// Type alias for multiple margins.
pub type Margins = (Option<f64>, Option<f64>, Option<f64>, Option<f64>);

/// Type alias for the paper size.
pub type PaperSize = (Option<f64>, Option<f64>);

/// Type alias for the window size.
pub type WindowSize = Option<(u32, u32)>;

/// Type alias for scale.
pub type Scale = Option<f64>;

/// Type alias for timeout.
pub type Timeout = Option<u64>;

/// Extension of PDF files.
pub const PDF_EXTENSION: &str = "pdf";

/// Extension of HTML files.
pub const HTML_EXTENSION: &str = "html";

/// Minimum paper length in inches.
pub const MIN_PAPER_LENGTH: f64 = 0.19;

/// Maximum paper length in inches.
pub const MAX_PAPER_LENGTH: f64 = 100.0;

/// Type of the screenshot output format.
#[derive(Copy, Clone)]
pub enum ScreenshotFormat {
  Jpeg,
  Png,
  Webp,
}

/// Converts margin definition into a tuple of values in inches.
pub fn margin(opt_margin: Option<String>) -> Result<Margins> {
  if let Some(margin) = opt_margin {
    let parts = margin.split(' ').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect::<Vec<String>>();
    let lengths = inches(parts)?;
    match lengths.len() {
      1 => Ok((Some(lengths[0]), Some(lengths[0]), Some(lengths[0]), Some(lengths[0]))),
      2 => Ok((Some(lengths[0]), Some(lengths[1]), Some(lengths[0]), Some(lengths[1]))),
      4 => Ok((Some(lengths[0]), Some(lengths[1]), Some(lengths[2]), Some(lengths[3]))),
      _ => Err(err_invalid_margin(&margin)),
    }
  } else {
    Ok((None, None, None, None))
  }
}

/// Converts scale definition into a value.
pub fn scale(opt_scale: Option<String>) -> Result<Scale> {
  if let Some(scale) = opt_scale {
    if let Some(prefix) = scale.strip_suffix('%') {
      Ok(Some(str_to_f64(prefix)? / 100.0))
    } else {
      Ok(Some(str_to_f64(&scale)?))
    }
  } else {
    Ok(None)
  }
}

/// Converts window size definition into a tuple of unsigned integers.
pub fn window_size(opt_window_size: Option<String>) -> Result<WindowSize> {
  if let Some(window_size) = opt_window_size {
    let parts = window_size.split(',').map(|s| s.trim().to_string()).collect::<Vec<String>>();
    if parts.len() == 2 {
      let width = parts[0].parse::<u32>().map_err(|_| err_invalid_width(&parts[0]))?;
      let height = parts[1].parse::<u32>().map_err(|_| err_invalid_height(&parts[1]))?;
      Ok(Some((width, height)))
    } else {
      Err(err_invalid_window_size(&window_size))
    }
  } else {
    Ok(None)
  }
}

/// Converts file path into file URL string.
pub fn file_url(file_path: &Path) -> Result<String> {
  Ok(format!(
    "file://{}",
    file_path.canonicalize().map_err(|e| err_canonicalize(file_path, e.to_string()))?.to_string_lossy()
  ))
}

/// Replaces the extension to `.pdf`.
pub fn replace_ext(path: &Path) -> String {
  path.with_extension(PDF_EXTENSION).to_string_lossy().to_string()
}

/// Replaces the extension to `.pdf` and returns the file name.
pub fn file_name(path: &Path) -> Result<String> {
  Ok(path.with_extension(PDF_EXTENSION).file_name().ok_or(err_file_name(path))?.to_string_lossy().to_string())
}

/// Returns `true` when specified path has `HTML` file extension.
pub fn has_html_extension(path: &Path) -> bool {
  if let Some(extension) = path.extension() {
    extension == HTML_EXTENSION
  } else {
    false
  }
}

/// Initializes the logger.
pub fn init_logger(opt_log_level: Option<String>) {
  match env::var("RUST_LOG").unwrap_or("off".to_string()).as_str() {
    "error" | "warn" | "info" | "debug" | "trace" => {}
    _ => env::set_var("RUST_LOG", "off"),
  }
  if let Some(log_level) = opt_log_level {
    env::set_var("RUST_LOG", log_level);
  }
  env_logger::init();
}

/// Converts string to [f64] value.
pub fn str_to_f64(s: &str) -> Result<f64> {
  s.parse::<f64>().map_err(|_| err_invalid_number(s))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_margin() {
    assert_eq!((None, None, None, None), margin(None).unwrap());
    assert_eq!(
      (Some(0.39375), Some(0.39375), Some(0.39375), Some(0.39375)),
      margin(Some("1cm 1cm 1cm 1cm".to_string())).unwrap()
    );
    assert_eq!(
      (Some(0.39375), Some(0.39375), Some(0.39375), Some(0.39375)),
      margin(Some("1cm    1cm   1cm     1cm".to_string())).unwrap()
    );
    assert_eq!((Some(0.39375), Some(0.39375), Some(0.39375), Some(0.39375)), margin(Some("1cm".to_string())).unwrap());
    assert_eq!((Some(0.39375), Some(0.4725), Some(0.39375), Some(0.4725)), margin(Some("1cm 1.2cm".to_string())).unwrap());
    assert_eq!(
      (Some(0.39375), Some(0.7875), Some(1.18125), Some(1.575)),
      margin(Some("1cm 2cm 3cm 4cm".to_string())).unwrap()
    );
    assert!(margin(Some("1cm 2cm 3cm".to_string())).is_err());
  }
}
