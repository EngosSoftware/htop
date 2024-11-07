//! # HTML to PDF converter

use crate::defs::{Files, ScreenshotFormat};
use crate::errors::{err_headless_chrome, err_write_file, Result};
use crate::options::{PdfPrintingOptions, ScreenshotTakingOptions};
use crate::units::{inches_to_millimeters, inches_to_points, round1};
use clap::crate_name;
use headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption;
use headless_chrome::{Browser, LaunchOptionsBuilder};
use std::ffi::OsStr;
use std::fs;
use std::time::Duration;

/// Converts `HTML` input files into `PDF` output files.
pub fn html_to_pdf(files: Files, pdf_printing_options: PdfPrintingOptions) -> Result<()> {
  let verbose = pdf_printing_options.verbose;
  let paper_size = pdf_printing_options.paper_size;
  let no_crash_reports = pdf_printing_options.no_crash_reports;
  let mut arguments = vec![OsStr::new("--disable-search-engine-choice-screen")];
  if no_crash_reports {
    arguments.push(OsStr::new("--disable-crash-reporter"))
  }
  let launch_options = LaunchOptionsBuilder::default()
    .headless(true)
    .devtools(false)
    .args(arguments)
    .build()
    .map_err(|e| err_headless_chrome(e.to_string()))?;
  let browser = Browser::new(launch_options).map_err(|e| err_headless_chrome(e.to_string()))?;
  let tab = browser.new_tab().map_err(|e| err_headless_chrome(e.to_string()))?;
  if let Some(timeout) = pdf_printing_options.page_load_timout {
    tab.set_default_timeout(Duration::from_millis(timeout));
  }
  for (input_url, output_file_name) in &files {
    if verbose {
      println!("[{}] Printing file {}", crate_name!(), input_url);
      if let Some((paper_width, paper_height)) = paper_size.0.zip(paper_size.1) {
        println!(
          "[{}] Page size: {} x {} mm,  {} x {} in, {} x {} pts",
          crate_name!(),
          inches_to_millimeters(paper_width),
          inches_to_millimeters(paper_height),
          round1(paper_width),
          round1(paper_height),
          inches_to_points(paper_width),
          inches_to_points(paper_height)
        );
      }
    }
    tab.navigate_to(input_url).map_err(|e| err_headless_chrome(e.to_string()))?;
    tab.wait_until_navigated().map_err(|e| err_headless_chrome(e.to_string()))?;
    let pdf = tab
      .print_to_pdf(Some(pdf_printing_options.clone().into()))
      .map_err(|e| err_headless_chrome(e.to_string()))?;
    fs::write(output_file_name, pdf).map_err(|e| err_write_file(output_file_name, e.to_string()))?;
    if verbose {
      println!("[{}] Printing completed: {}\n", crate_name!(), output_file_name);
    }
  }
  Ok(())
}

/// Converts `HTML` input files into image output files.
pub fn html_to_screenshot(files: Files, screenshot_taking_options: ScreenshotTakingOptions) -> Result<()> {
  let output_format = screenshot_taking_options.output_format;
  let windows_size: Option<(u32, u32)> = screenshot_taking_options.window_size.into();
  let verbose = screenshot_taking_options.verbose;
  let no_crash_reports = screenshot_taking_options.no_crash_reports;
  let mut arguments = vec![OsStr::new("--disable-search-engine-choice-screen")];
  if no_crash_reports {
    arguments.push(OsStr::new("--disable-crash-reporter"))
  }
  let launch_options = LaunchOptionsBuilder::default()
    .headless(true)
    .devtools(false)
    .args(arguments)
    .window_size(windows_size)
    .build()
    .map_err(|e| err_headless_chrome(e.to_string()))?;
  let browser = Browser::new(launch_options).map_err(|e| err_headless_chrome(e.to_string()))?;
  let tab = browser.new_tab().map_err(|e| err_headless_chrome(e.to_string()))?;
  if let Some(timeout) = screenshot_taking_options.page_load_timout {
    tab.set_default_timeout(Duration::from_millis(timeout));
  }
  for (input_url, output_file_name) in &files {
    tab.navigate_to(input_url).map_err(|e| err_headless_chrome(e.to_string()))?;
    tab.wait_until_navigated().map_err(|e| err_headless_chrome(e.to_string()))?;
    let image = tab
      .capture_screenshot(output_format.into(), None, None, true)
      .map_err(|e| err_headless_chrome(e.to_string()))?;
    fs::write(output_file_name, image).map_err(|e| err_write_file(output_file_name, e.to_string()))?;
    if verbose {
      println!("[{}] Imaging completed: {}\n", crate_name!(), output_file_name);
    }
  }
  Ok(())
}
