//! Command-line utilities

//-----------------------------------------------------------------------------
// Subcommand names
//-----------------------------------------------------------------------------

use crate::defs::{PaperSize, MAX_PAPER_LENGTH, MIN_PAPER_LENGTH};
use crate::errors::{err_invalid_paper_height, err_invalid_paper_width, err_invalid_value, err_read_file, Result};
use crate::paper::Paper;
use crate::units::to_inches;
use clap::{arg, command, crate_name, ArgAction, ArgGroup, ArgMatches};
use std::fmt::Debug;
use std::fs;
use std::str::FromStr;

pub const SUBCOMMAND_SINGLE: &str = "single";

pub const SUBCOMMAND_MULTIPLE: &str = "multiple";

pub const SUBCOMMAND_URL: &str = "url";

//-----------------------------------------------------------------------------
// Commands descriptions
//-----------------------------------------------------------------------------

pub const ABOUT_SINGLE: &str = r#"Convert single HTML file to PDF file"#;

pub const LONG_ABOUT_SINGLE: &str = r#"Convert single HTML file to PDF file."#;

pub const ABOUT_MULTIPLE: &str = r#"Convert multiple HTML files to multiple PDF files"#;

pub const LONG_ABOUT_MULTIPLE: &str = r#"Convert multiple HTML files to multiple PDF files."#;

pub const ABOUT_URL: &str = r#"Convert single HTML page to PDF file"#;

pub const LONG_ABOUT_URL: &str = r#"Convert single HTML page to PDF file."#;

//-----------------------------------------------------------------------------
// Options descriptions
//-----------------------------------------------------------------------------

pub const HELP_BACKGROUND: &str = r#"Print the page background"#;

pub const LONG_HELP_BACKGROUND: &str = r#"Prints the page background. When this option is present,
the output PDF file will contain the background filled
just like the input HTML file or website."#;

pub const HELP_HEADER_FOOTER: &str = r#"Print the header and footer"#;

pub const LONG_HELP_HEADER_FOOTER: &str = r#"Prints the header and footer. When this option is present,
the header and footer will be printed to PDF file.
When no custom header and/or footer is set, then the
default Chromium's header and/or footer is printed."#;

pub const HELP_HEADER: &str = r#"Set custom header"#;

pub const LONG_HELP_HEADER: &str = r#"Sets custom header. This option defines custom HTML template
for the print header. This template must be valid HTML markup
with the following classes used to inject printing values into them:
  date       - formatted date of printing,
  title      - document title,
  url        - document location,
  pageNumber - current page number,
  totalPages - total number of pages in the document.
The same rules are valid for --footer option."#;

pub const HELP_HEADER_FILE: &str = r#"Load custom header from file"#;

pub const LONG_HELP_HEADER_FILE: &str = r#"Loads custom header from file. This option defines custom HTML template
for the print header loaded from file.
See --header option for a detailed description of the HTML template."#;

pub const HELP_FOOTER: &str = r#"Set custom footer"#;

pub const LONG_HELP_FOOTER: &str = r#"Sets custom footer. The same rules as for --header option."#;

pub const HELP_FOOTER_FILE: &str = r#"Load custom footer from file"#;

pub const LONG_HELP_FOOTER_FILE: &str = r#"Loads custom footer from file. This option defines custom HTML template
for the print footer loaded from file.
See --header option for a detailed description of the HTML template."#;

pub const HELP_LANDSCAPE: &str = r#"Set the paper orientation to landscape"#;

pub const LONG_HELP_LANDSCAPE: &str = r#"Sets the paper orientation to landscape. In landscape mode
the longest paper edge is positioned in horizontal
direction. The default paper orientation is portrait."#;

pub const HELP_PAPER: &str = r#"Set the paper format"#;

pub const LONG_HELP_PAPER: &str = r#"Sets the paper format. Currently supported paper formats:
A0 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11 A12 A13 2A0 4A0 A0+ A1+ A3+
B0 B1 B2 B3 B4 B5 B6 B7 B8 B9 B10 B11 B12 B13 B0+ B1+ B2+
C0 C1 C2 C3 C4 C5 C6 C7 C8 C9 C10
Letter Legal Tabloid Ledger 'Junior Legal' 'Half Letter'
'Government Letter' 'Government Legal'
'Ansi A' 'Ansi B' 'Ansi C' 'Ansi D' 'Ansi E'
'Arch A' 'Arch B' 'Arch C' 'Arch D' 'Arch E'
'Arch E1' 'Arch E2' 'Arch E3'"#;

pub const HELP_PAPER_WIDTH: &str = r#"Set the custom paper width"#;

pub const LONG_HELP_PAPER_WIDTH: &str = r#"Sets the custom paper width. Paper width should be specified
using CSS units, e.g.: '100mm', '21cm', '800pt', etc.
Allowed units are: cm, mm, Q, in, pc, pt, px."#;

pub const HELP_PAPER_HEIGHT: &str = r#"Set the custom paper height"#;

pub const LONG_HELP_PAPER_HEIGHT: &str = r#"Sets the custom paper height. Paper height should be specified
using CSS units, e.g.: '200mm', '29.7cm', '1200pt', etc.
Allowed units are: cm, mm, Q, in, pc, pt, px."#;

pub const HELP_PAPER_SIZE: &str = r#"Set the custom paper size"#;

pub const LONG_HELP_PAPER_SIZE: &str = r#"Sets the custom paper size. Paper size should be specified
as two values separated by comma using CSS units, e.g.: '210mm,297mm', '21cm,29.7cm', etc.
Allowed units are: cm, mm, Q, in, pc, pt, px."#;

pub const HELP_RANGES: &str = r#"Set the range of pages to print"#;

pub const LONG_HELP_RANGES: &str = r#"Sets the range of pages to print.
Paper ranges start from 1, e.g.: '1-5, 8, 11-13'.
Pages are printed in the document order."#;

pub const HELP_SCALE: &str = r#"Set the printing scale"#;

pub const LONG_HELP_SCALE: &str = r#"Sets the printing scale.
Scale should be specified as a number or as a percentage.
Accepted scales are in range from 0.1 to 2.0 (from 10% to 200%).
The default scale is 1.0 (100%)."#;

pub const HELP_MARGIN: &str = r#"Set the margin"#;

pub const LONG_HELP_MARGIN: &str = r#"Sets the margin. The value of the margin
should be specified like in CSS, e.g.:
  '0 12mm 3pt 4px' - sets the top margin to zero, right margin to 12mm,
                     bottom margin to 3pt and left margin to 4px,
  '1.5cm 2cm'      - sets the top and bottom margins to 1.5cm,
                     the left and right margins are set to 2cm,
  '15mm'           - sets all margins to 15mm.
Allowed units are: cm, mm, Q, in, pc, pt, px."#;

pub const HELP_VERBOSE: &str = r#"Display printing details"#;

pub const LONG_HELP_VERBOSE: &str = r#"Displays printing details. Prints to standard
output informative messages about the printing process."#;

pub const HELP_NO_CRASH_REPORTS: &str = r#"Disable saving crash reports"#;

pub const LONG_HELP_NO_CRASH_REPORTS: &str = r#"Disables saving crash reports. By default, Headless Chrome
saves crash reports in user's home directory.
This can be switched-off by specifying this option.
When printing is run in multiuser environment or in the cloud,
it may happen, that crash reports generated by one user
clash with reports generated by other user. In such case
the printing process hangs forever."#;

pub const HELP_PAGE_LOAD_TIMEOUT: &str = r#"Set page load timeout in milliseconds"#;

pub const LONG_HELP_PAGE_LOAD_TIMEOUT: &str = r#"Sets page load timeout in milliseconds. During printing
webpages, when the page load timeout is reached, the printing
process is cancelled and an error is reported."#;

pub const HELP_LOG_LEVEL: &str = r#"Set the logging level"#;

pub const LONG_HELP_LOG_LEVEL: &str = r#"Sets the logging level. Allowed logging levels are:
  none,
  error,
  warn,
  info,
  debug,
  trace.
The default logging level is 'none'."#;

pub const HELP_INPUT_FILE: &str = r#"Input HTML file (required)"#;

pub const HELP_OUTPUT_FILE: &str = r#"Output PDF file name (optional)"#;

pub const HELP_INPUT_DIR: &str = r#"Input directory name (required)"#;

pub const HELP_OUTPUT_DIR: &str = r#"Output directory name (optional)"#;

pub const HELP_INPUT_URL: &str = r#"Input page URL (required)"#;

//-----------------------------------------------------------------------------
// Utility functions
//-----------------------------------------------------------------------------

/// Returns matched parsable value from command-line arguments.
pub fn value<T>(matches: &ArgMatches, id: &str) -> Result<Option<T>>
where
  T: FromStr,
  <T as FromStr>::Err: Debug,
{
  if let Some(s) = matches.get_one::<String>(id).cloned() {
    let value = s.parse::<T>().map_err(|e| err_invalid_value(&s, &format!("{:?}", e)))?;
    Ok(Some(value))
  } else {
    Ok(None)
  }
}

/// Returns the value of the flag taken from command-line argument matches.
pub fn flag(matches: &ArgMatches, id: &str) -> bool {
  matches.get_flag(id)
}

/// Returns the value of the string taken from command-line argument matches.
pub fn string(matches: &ArgMatches, id: &str) -> Option<String> {
  matches.get_one::<String>(id).cloned()
}

pub fn file_content(matches: &ArgMatches, id: &str, file_id: &str) -> Result<Option<String>> {
  let content = string(matches, id);
  if content.is_some() {
    return Ok(content);
  }
  if let Some(file_name) = string(matches, file_id) {
    let file_content = fs::read_to_string(&file_name).map_err(|e| err_read_file(&file_name, e.to_string()))?;
    return Ok(Some(file_content));
  }
  Ok(None)
}

/// Converts paper format definitions into a tuple of values in inches.
pub fn paper(matches: &ArgMatches, paper_format_id: &str, paper_width_id: &str, paper_height_id: &str, verbose: bool) -> Result<PaperSize> {
  let opt_paper_format = string(matches, paper_format_id);
  let opt_paper_width = string(matches, paper_width_id);
  let opt_paper_height = string(matches, paper_height_id);
  if let Some(paper_format) = opt_paper_format {
    let paper = Paper::new(paper_format.as_str().try_into()?);
    if verbose {
      println!("[{}] Paper format: {}", crate_name!(), paper_format);
    }
    Ok((Some(paper.width()), Some(paper.height())))
  } else if let Some((width_str, height_str)) = opt_paper_width.zip(opt_paper_height) {
    let width = to_inches(&width_str)?;
    if !(MIN_PAPER_LENGTH..=MAX_PAPER_LENGTH).contains(&width) {
      return Err(err_invalid_paper_width(width));
    }
    let height = to_inches(&height_str)?;
    if !(MIN_PAPER_LENGTH..=MAX_PAPER_LENGTH).contains(&height) {
      return Err(err_invalid_paper_height(height));
    }
    Ok((Some(width), Some(height)))
  } else {
    Ok((None, None))
  }
}

/// Returns command-line arguments matches.
pub fn get_matches() -> ArgMatches {
  command!()
    .arg(
      arg!(--"print-background")
        .short('b')
        .help(HELP_BACKGROUND)
        .long_help(LONG_HELP_BACKGROUND)
        .action(ArgAction::SetTrue)
        .display_order(1),
    )
    .arg(
      arg!(--"print-header-footer")
        .help(HELP_HEADER_FOOTER)
        .long_help(LONG_HELP_HEADER_FOOTER)
        .action(ArgAction::SetTrue)
        .display_order(2),
    )
    .arg(
      arg!(--"header" <HEADER>)
        .help(HELP_HEADER)
        .long_help(LONG_HELP_HEADER)
        .action(ArgAction::Set)
        .display_order(3),
    )
    .arg(
      arg!(--"header-file" <HEADER_FILE>)
        .help(HELP_HEADER_FILE)
        .long_help(LONG_HELP_HEADER_FILE)
        .action(ArgAction::Set)
        .display_order(4),
    )
    .arg(
      arg!(--"footer" <FOOTER>)
        .help(HELP_FOOTER)
        .long_help(LONG_HELP_FOOTER)
        .action(ArgAction::Set)
        .display_order(5),
    )
    .arg(
      arg!(--"footer-file" <FOOTER_FILE>)
        .help(HELP_FOOTER_FILE)
        .long_help(LONG_HELP_FOOTER_FILE)
        .action(ArgAction::Set)
        .display_order(6),
    )
    .arg(
      arg!(--"landscape")
        .short('l')
        .help(HELP_LANDSCAPE)
        .long_help(LONG_HELP_LANDSCAPE)
        .action(ArgAction::SetTrue)
        .display_order(7),
    )
    .arg(
      arg!(--"margin" <MARGIN>)
        .short('m')
        .help(HELP_MARGIN)
        .long_help(LONG_HELP_MARGIN)
        .action(ArgAction::Set)
        .display_order(8),
    )
    .arg(
      arg!(--"paper-format" <FORMAT>)
        .short('p')
        .help(HELP_PAPER)
        .long_help(LONG_HELP_PAPER)
        .action(ArgAction::Set)
        .display_order(9),
    )
    .arg(
      arg!(--"paper-width" <PAPER_WIDTH>)
        .help(HELP_PAPER_WIDTH)
        .long_help(LONG_HELP_PAPER_WIDTH)
        .action(ArgAction::Set)
        .display_order(10)
        .requires("paper-height"),
    )
    .arg(
      arg!(--"paper-height" <PAPER_HEIGHT>)
        .help(HELP_PAPER_HEIGHT)
        .long_help(LONG_HELP_PAPER_HEIGHT)
        .action(ArgAction::Set)
        .display_order(11)
        .requires("paper-width"),
    )
    .arg(
      arg!(--"paper-size" <PAPER_SIZE>)
        .help(HELP_PAPER_SIZE)
        .long_help(LONG_HELP_PAPER_SIZE)
        .action(ArgAction::Set)
        .display_order(12),
    )
    .arg(
      arg!(--"ranges" <RANGES>)
        .short('r')
        .help(HELP_RANGES)
        .long_help(LONG_HELP_RANGES)
        .action(ArgAction::Set)
        .display_order(13),
    )
    .arg(
      arg!(--"scale" <SCALE>)
        .short('s')
        .help(HELP_SCALE)
        .long_help(LONG_HELP_SCALE)
        .action(ArgAction::Set)
        .display_order(14),
    )
    .arg(arg!(--"jpg").help("a").long_help("b").action(ArgAction::SetTrue).display_order(15))
    .arg(arg!(--"png").help("c").long_help("d").action(ArgAction::SetTrue).display_order(16))
    .arg(arg!(--"webp").help("e").long_help("f").action(ArgAction::SetTrue).display_order(17))
    .arg(arg!(--"window-size" <WINDOW_SIZE>).help("g").long_help("h").action(ArgAction::Set).display_order(18))
    .arg(
      arg!(--"verbose")
        .short('v')
        .help(HELP_VERBOSE)
        .long_help(LONG_HELP_VERBOSE)
        .action(ArgAction::SetTrue)
        .display_order(20),
    )
    .arg(
      arg!(--"log-level" <LEVEL>)
        .help(HELP_LOG_LEVEL)
        .long_help(LONG_HELP_LOG_LEVEL)
        .action(ArgAction::Set)
        .display_order(21),
    )
    .arg(
      arg!(--"no-crash-reports")
        .help(HELP_NO_CRASH_REPORTS)
        .long_help(LONG_HELP_NO_CRASH_REPORTS)
        .action(ArgAction::SetTrue)
        .display_order(22),
    )
    .arg(
      arg!(--"timeout" <TIMEOUT>)
        .short('t')
        .help(HELP_PAGE_LOAD_TIMEOUT)
        .long_help(LONG_HELP_PAGE_LOAD_TIMEOUT)
        .action(ArgAction::Set)
        .display_order(23),
    )
    .group(ArgGroup::new("headers").arg("header").arg("header-file"))
    .group(ArgGroup::new("footers").arg("footer").arg("footer-file"))
    .group(ArgGroup::new("paper-widths").arg("paper-format").arg("paper-width").arg("paper-size"))
    .group(ArgGroup::new("paper-heights").arg("paper-format").arg("paper-height").arg("paper-size"))
    .group(ArgGroup::new("image-formats").arg("jpg").arg("png").arg("webp"))
    .group(
      ArgGroup::new("pdf-printing-only")
        .arg("header")
        .arg("header-file")
        .arg("footer")
        .arg("footer-file")
        .arg("paper-format")
        .arg("paper-width")
        .arg("paper-height")
        .arg("print-background")
        .arg("scale")
        .multiple(true)
        .conflicts_with("image-printing-only"),
    )
    .group(
      ArgGroup::new("image-printing-only")
        .arg("jpg")
        .arg("png")
        .arg("webp")
        .arg("window-size")
        .multiple(true)
        .conflicts_with("pdf-printing-only"),
    )
    .subcommand(
      command!()
        .name(SUBCOMMAND_SINGLE)
        .about(ABOUT_SINGLE)
        .long_about(LONG_ABOUT_SINGLE)
        .arg(arg!(<INPUT_FILE>).help(HELP_INPUT_FILE).required(true).index(1))
        .arg(arg!([OUTPUT_FILE]).help(HELP_OUTPUT_FILE).required(false).index(2)),
    )
    .subcommand(
      command!()
        .name(SUBCOMMAND_MULTIPLE)
        .about(ABOUT_MULTIPLE)
        .long_about(LONG_ABOUT_MULTIPLE)
        .arg(arg!(<INPUT_DIR>).help(HELP_INPUT_DIR).required(true).index(1))
        .arg(arg!([OUTPUT_DIR]).help(HELP_OUTPUT_DIR).required(false).index(2)),
    )
    .subcommand(
      command!()
        .name(SUBCOMMAND_URL)
        .about(ABOUT_URL)
        .long_about(LONG_ABOUT_URL)
        .arg(arg!(<URL>).help(HELP_INPUT_URL).required(true).index(1))
        .arg(arg!([OUTPUT_FILE]).help(HELP_OUTPUT_FILE).required(false).index(2)),
    )
    .get_matches()
}
