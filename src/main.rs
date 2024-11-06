//! # Application entry point

use crate::cli::*;
use crate::converter::{html_to_image, html_to_pdf};
use crate::defs::*;
use crate::errors::Result;
use crate::options::{ImagePrintingOptions, PdfPrintingOptions};
use clap::{crate_description, crate_name, crate_version};
use std::fs;
use std::path::Path;

mod cli;
mod converter;
mod defs;
mod errors;
mod options;
mod paper;
mod units;

/// Main entrypoint of the application.
fn main() -> Result<()> {
  // get command-line argument matches
  let matches = get_matches();

  // initialize the logger
  init_logger(matches.get_one::<String>("log-level").cloned());

  let verbose = flag(&matches, "verbose");
  let no_crash_reports = flag(&matches, "no-crash-reports");
  let page_load_timout = value(&matches, "timeout")?;

  // prepare printing options
  let pdf_printing_options = PdfPrintingOptions {
    landscape: flag(&matches, "landscape"),
    print_header_footer: flag(&matches, "print-header-footer"),
    print_background: flag(&matches, "print-background"),
    scale: scale(string(&matches, "scale"))?,
    paper_size: paper(&matches, "paper-format", "paper-width", "paper-height", verbose)?,
    margins: margin(string(&matches, "margin"))?,
    page_ranges: string(&matches, "ranges"),
    header: file_content(&matches, "header", "header-file")?,
    footer: file_content(&matches, "footer", "footer-file")?,
    verbose,
    no_crash_reports,
    page_load_timout,
  };

  let image_printing_options = ImagePrintingOptions {
    verbose,
    no_crash_reports,
    page_load_timout,
  };

  // parse subcommands
  match matches.subcommand() {
    Some((SUBCOMMAND_SINGLE, m)) => {
      // input file name is required
      let input_file = m.get_one::<String>("INPUT_FILE").unwrap();
      let input_file_path = Path::new(input_file);
      let input_file_url = file_url(input_file_path)?;
      // output file name is optional, when not provided, then
      // the output file name is derived from the input file name
      let output_file_name = if let Some(output_file) = m.get_one::<String>("OUTPUT_FILE") {
        output_file.to_owned()
      } else {
        replace_ext(input_file_path)
      };
      // convert input HTML file into output PDF file
      html_to_pdf(vec![(input_file_url, output_file_name)], pdf_printing_options)?;
    }
    Some((SUBCOMMAND_MULTIPLE, m)) => {
      let mut files: Files = vec![];
      // input directory name is required
      let input_dir = m.get_one::<String>("INPUT_DIR").unwrap();
      // output directory is optional, when not provided, then
      // the input directory is used as output directory
      if let Some(output_dir) = m.get_one::<String>("OUTPUT_DIR") {
        for path in fs::read_dir(input_dir).unwrap() {
          let entry = path.unwrap().path();
          if entry.is_file() && has_html_extension(entry.as_path()) {
            let input_file_url = file_url(&entry)?;
            let output_file_path = Path::new(output_dir).join(file_name(entry.as_path())?);
            let output_file_name = output_file_path.to_string_lossy().to_string();
            files.push((input_file_url, output_file_name));
          }
        }
      } else {
        for path in fs::read_dir(input_dir).unwrap() {
          let entry = path.unwrap().path();
          if entry.is_file() && has_html_extension(entry.as_path()) {
            let input_file_url = file_url(&entry)?;
            let output_file_name = replace_ext(entry.as_path());
            files.push((input_file_url, output_file_name));
          }
        }
      }
      // convert input HTML files into output PDF files
      html_to_pdf(files, pdf_printing_options)?;
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
      // convert input HTML page into output PDF file
      //html_to_pdf(vec![(input_url, output_file_name)], pdf_printing_options)?;
      html_to_image(vec![(input_url, output_file_name)], image_printing_options)?;
    }
    _ => {
      println!("{} {}\n\n{}\n", crate_name!(), crate_version!(), crate_description!());
      println!("{}: missing subcommand", crate_name!());
      println!("Try '{} --help' for more information.", crate_name!());
    }
  }
  Ok(())
}
