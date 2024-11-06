//! # PDF printing options

use crate::defs::{Margins, PaperSize};
use headless_chrome::types::PrintToPdfOptions;

/// PDF printing options.
#[derive(Clone)]
pub struct PdfPrintingOptions {
  /// Paper mode, `true` = landscape, `false` = portrait.
  pub landscape: bool,
  /// Flag indicating if header and footer should be printed, `true` = print header and footer.
  pub print_header_footer: bool,
  /// Flag indicating if background should be printed, `true` = print background.
  pub print_background: bool,
  /// Scale.
  pub scale: Option<f64>,
  /// Paper size.
  pub paper_size: PaperSize,
  /// Margins.
  pub margins: Margins,
  /// Page ranges.
  pub page_ranges: Option<String>,
  /// Header HTML template.
  pub header: Option<String>,
  /// Footer HTML template.
  pub footer: Option<String>,
  /// Flag indicating if printing process should be more talkative, `true` = more talkative.
  pub verbose: bool,
  /// Flag indicating if crash reporter should be disabled, `true` = disabled.
  pub no_crash_reports: bool,
  /// Page load timeout in milliseconds.
  pub page_load_timout: Option<u64>,
}

impl From<PdfPrintingOptions> for PrintToPdfOptions {
  /// Converts [PdfPrintingOptions] into [PrintToPdfOptions].
  fn from(value: PdfPrintingOptions) -> Self {
    Self {
      landscape: Some(value.landscape),
      display_header_footer: Some(value.print_header_footer),
      print_background: Some(value.print_background),
      scale: value.scale,
      paper_width: value.paper_size.0,
      paper_height: value.paper_size.1,
      margin_top: value.margins.0,
      margin_bottom: value.margins.2,
      margin_left: value.margins.3,
      margin_right: value.margins.1,
      page_ranges: value.page_ranges,
      ignore_invalid_page_ranges: None,
      header_template: value.header,
      footer_template: value.footer,
      prefer_css_page_size: None,
      transfer_mode: None,
    }
  }
}

/// Image printing options.
#[derive(Clone)]
pub struct ImagePrintingOptions {
  /// Flag indicating if printing process should be more talkative, `true` = more talkative.
  pub verbose: bool,
  /// Flag indicating if crash reporter should be disabled, `true` = disabled.
  pub no_crash_reports: bool,
  /// Page load timeout in milliseconds.
  pub page_load_timout: Option<u64>,
}
