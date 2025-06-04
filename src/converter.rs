//! # HTML to PDF converter with screenshots

use crate::defs::{Files, ScreenshotFormat, Timeout, WindowSize};
use crate::errors::{err_headless_chrome, err_write_file, Result};
use crate::options::{PdfPrintingOptions, ScreenshotTakingOptions};
use crate::units::{inches_to_millimeters, inches_to_points, round1};
use clap::crate_name;
use headless_chrome::{Browser, LaunchOptionsBuilder, Tab};
use std::ffi::OsStr;
use std::fs;
use std::sync::Arc;
use std::time::Duration;

/// Converts `HTML` files into `PDF` files.
pub fn html_to_pdf(files: Files, pdf_printing_options: PdfPrintingOptions) -> Result<()> {
  let verbose = pdf_printing_options.verbose;
  let paper_size = pdf_printing_options.paper_size;
  let browser = get_browser(pdf_printing_options.no_crash_reports, None)?;
  let tab = get_tab(&browser, pdf_printing_options.page_load_timeout)?;
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
    save_file(output_file_name, &pdf, verbose, "Printing")?;
  }
  Ok(())
}

/// Takes screenshot of `HTML` files.
pub fn html_to_screenshot(files: Files, screenshot_taking_options: ScreenshotTakingOptions) -> Result<()> {
  let output_format = screenshot_taking_options.screenshot_format.unwrap_or(ScreenshotFormat::Png);
  let verbose = screenshot_taking_options.verbose;
  let browser = get_browser(screenshot_taking_options.no_crash_reports, screenshot_taking_options.window_size)?;
  let tab = get_tab(&browser, screenshot_taking_options.page_load_timeout)?;
  for (input_url, output_file_name) in &files {
    tab.navigate_to(input_url).map_err(|e| err_headless_chrome(e.to_string()))?;
    tab.wait_until_navigated().map_err(|e| err_headless_chrome(e.to_string()))?;
    let image = tab
      .capture_screenshot(output_format.into(), None, None, true)
      .map_err(|e| err_headless_chrome(e.to_string()))?;
    save_file(output_file_name, &image, verbose, "Imaging")?;
  }
  Ok(())
}

/// Returns a new browser.
fn get_browser(no_crash_reports: bool, window_size: WindowSize) -> Result<Browser> {
  let mut arguments = vec![OsStr::new("--disable-search-engine-choice-screen")];
  if no_crash_reports {
    arguments.push(OsStr::new("--disable-crash-reporter"))
  }
  let launch_options = LaunchOptionsBuilder::default()
    .headless(true)
    .devtools(false)
    .args(arguments)
    .window_size(window_size)
    .build()
    .map_err(|e| err_headless_chrome(e.to_string()))?;
  Browser::new(launch_options).map_err(|e| err_headless_chrome(e.to_string()))
}

/// Returns a new browser tab.
fn get_tab(browser: &Browser, timeout: Timeout) -> Result<Arc<Tab>> {
  let tab = browser.new_tab().map_err(|e| err_headless_chrome(e.to_string()))?;
  if let Some(timeout) = timeout {
    tab.set_default_timeout(Duration::from_millis(timeout));
  }
  Ok(tab)
}

/// Saves the content to file.
fn save_file(file_name: &str, content: &[u8], verbose: bool, action: &str) -> Result<()> {
  fs::write(file_name, content).map_err(|e| err_write_file(file_name, e.to_string()))?;
  if verbose {
    println!("[{}] {action} completed: {file_name}\n", crate_name!());
  }
  Ok(())
}
