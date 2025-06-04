//! # The application

mod cli;
mod converter;
mod defs;
mod errors;
mod options;
mod paper;
mod units;

use clap::{crate_description, crate_name, crate_version};
use cli::{file_content, flag, get_command, paper, string, timeout, SUBCOMMAND_MULTIPLE, SUBCOMMAND_SINGLE, SUBCOMMAND_URL};
use converter::{html_to_pdf, html_to_screenshot};
use defs::{file_name, file_url, file_url_unchecked, has_html_extension, init_logger, margin, replace_ext, scale, window_size, Files, ScreenshotFormat};
use errors::{err_read_dir, Result};
use options::{PdfPrintingOptions, ScreenshotTakingOptions};
use std::fs;
use std::path::Path;

/// Application entrypoint.
fn main() -> Result<()> {
  // Get command-line matches.
  let matches = get_command().get_matches();

  // Initialize the logger.
  init_logger(matches.get_one::<String>("log-level").cloned());

  let verbose = flag(&matches, "verbose");
  let no_crash_reports = flag(&matches, "no-crash-reports");
  let page_load_timout = timeout(&matches, "timeout")?;

  // prepare printing options
  let pdf_printing_options = PdfPrintingOptions {
    landscape: flag(&matches, "landscape"),
    print_header_footer: flag(&matches, "print-header-footer"),
    print_background: flag(&matches, "print-background"),
    scale: scale(string(&matches, "scale"))?,
    paper_size: paper(&matches, "paper-format", "paper-width", "paper-height", "paper-size", verbose)?,
    margins: margin(string(&matches, "margin"))?,
    page_ranges: string(&matches, "ranges"),
    header: file_content(&matches, "header", "header-file")?,
    footer: file_content(&matches, "footer", "footer-file")?,
    verbose,
    no_crash_reports,
    page_load_timout,
  };

  let output_format = if flag(&matches, "jpeg") {
    Some(ScreenshotFormat::Jpeg)
  } else if flag(&matches, "png") {
    Some(ScreenshotFormat::Png)
  } else if flag(&matches, "webp") {
    Some(ScreenshotFormat::Webp)
  } else {
    None
  };

  let screenshot_taking_options = ScreenshotTakingOptions {
    output_format,
    window_size: window_size(string(&matches, "window-size"))?,
    verbose,
    no_crash_reports,
    page_load_timeout: page_load_timout,
  };

  let process_files = |files: Files| -> Result<()> {
    if output_format.is_some() {
      // take screenshots
      html_to_screenshot(files, screenshot_taking_options)
    } else {
      // print PDF
      html_to_pdf(files, pdf_printing_options)
    }
  };

  // parse subcommands
  match matches.subcommand() {
    Some((SUBCOMMAND_SINGLE, m)) => {
      // Input file name is required.
      let input_file = m.get_one::<String>("INPUT_FILE").unwrap();
      let input_file_path = Path::new(input_file);
      let input_file_url = file_url(input_file_path)?;
      // Output file name is optional, when not provided, then
      // the output file name is derived from the input file name.
      let output_file_name = if let Some(output_file) = m.get_one::<String>("OUTPUT_FILE") {
        output_file.to_owned()
      } else {
        replace_ext(input_file_path)
      };
      process_files(vec![(input_file_url, output_file_name)])?;
    }
    Some((SUBCOMMAND_MULTIPLE, m)) => {
      let mut files: Files = vec![];
      // Input directory name is required.
      let input_dir = m.get_one::<String>("INPUT_DIR").unwrap();
      // Output directory is optional, when not provided, then
      // the input directory is used as output directory.
      if let Some(output_dir) = m.get_one::<String>("OUTPUT_DIR") {
        for path in fs::read_dir(input_dir).map_err(|e| err_read_dir(input_dir, e.to_string()))? {
          let entry = path.unwrap().path();
          if entry.is_file() && has_html_extension(entry.as_path()) {
            let input_file_url = file_url_unchecked(&entry); // Change to file_url when an erroneous case is found.
            let output_file_path = Path::new(output_dir).join(file_name(entry.as_path()));
            let output_file_name = output_file_path.to_string_lossy().to_string();
            files.push((input_file_url, output_file_name));
          }
        }
      } else {
        for path in fs::read_dir(input_dir).map_err(|e| err_read_dir(input_dir, e.to_string()))? {
          let entry = path.unwrap().path();
          if entry.is_file() && has_html_extension(entry.as_path()) {
            let input_file_url = file_url_unchecked(&entry); // Change to file_url when an erroneous case is found.
            let output_file_name = replace_ext(entry.as_path());
            files.push((input_file_url, output_file_name));
          }
        }
      }
      process_files(files)?;
    }
    Some((SUBCOMMAND_URL, m)) => {
      // input page URL is required
      let input_url = m.get_one::<String>("URL").cloned().unwrap();
      // output file name is optional, when not provided, then
      // the output file name is set to predefined value
      let output_file_name = if let Some(output_file) = m.get_one::<String>("OUTPUT_FILE") {
        output_file.to_owned()
      } else {
        "output.pdf".to_string()
      };
      process_files(vec![(input_url, output_file_name)])?;
    }
    _ => {
      println!("{} {}\n\n{}\n", crate_name!(), crate_version!(), crate_description!());
      println!("{}: missing subcommand", crate_name!());
      println!("Try '{} --help' for more information.", crate_name!());
    }
  }
  Ok(())
}
