//! # HTML to PDF converter

use crate::defs::Files;
use crate::errors::{err_headless_chrome, err_write_file, Result};
use crate::options::{ImagePrintingOptions, PdfPrintingOptions};
use crate::units::{inches_to_millimeters, inches_to_points, round1};
use clap::crate_name;
use headless_chrome::protocol::cdp::Page;
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
  let options = LaunchOptionsBuilder::default()
    .headless(true)
    .devtools(false)
    .args(arguments)
    .build()
    .map_err(|e| err_headless_chrome(e.to_string()))?;
  let browser = Browser::new(options).map_err(|e| err_headless_chrome(e.to_string()))?;
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
pub fn html_to_image(files: Files, image_printing_options: ImagePrintingOptions) -> Result<()> {
  let verbose = image_printing_options.verbose;
  let no_crash_reports = image_printing_options.no_crash_reports;
  let mut arguments = vec![OsStr::new("--disable-search-engine-choice-screen")];
  if no_crash_reports {
    arguments.push(OsStr::new("--disable-crash-reporter"))
  }
  let options = LaunchOptionsBuilder::default()
    .headless(true)
    .devtools(false)
    .args(arguments)
    .window_size(Some((1200, 1200)))
    .build()
    .map_err(|e| err_headless_chrome(e.to_string()))?;
  let browser = Browser::new(options).map_err(|e| err_headless_chrome(e.to_string()))?;
  let tab = browser.new_tab().map_err(|e| err_headless_chrome(e.to_string()))?;
  if let Some(timeout) = image_printing_options.page_load_timout {
    tab.set_default_timeout(Duration::from_millis(timeout));
  }
  for (input_url, output_file_name) in &files {
    tab.navigate_to(input_url).map_err(|e| err_headless_chrome(e.to_string()))?;
    tab.wait_until_navigated().map_err(|e| err_headless_chrome(e.to_string()))?;
    let viewport = Page::Viewport {
      x: 0.0,
      y: 0.0,
      width: 1200.0,
      height: 1200.0,
      scale: 2.0,
    };
    let image = tab
      .capture_screenshot(CaptureScreenshotFormatOption::Jpeg, None, Some(viewport), true)
      .map_err(|e| err_headless_chrome(e.to_string()))?;
    fs::write(output_file_name, image).map_err(|e| err_write_file(output_file_name, e.to_string()))?;
    if verbose {
      println!("[{}] Imaging completed: {}\n", crate_name!(), output_file_name);
    }
  }
  Ok(())
}
