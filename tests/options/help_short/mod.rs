use super::*;
use assert_cmd::assert::OutputAssertExt;
use assert_cmd::cargo::CommandCargoExt;
use std::process::Command;

#[test]
fn _0001() {
  let tc = test_context!().set_up();
  let mut cmd = Command::cargo_bin("htop").unwrap();
  cmd.current_dir(tc.current_dir()).arg("-h").assert().stdout(
    r#"HTML to PDF converter

Usage: htop [OPTIONS] [COMMAND]

Commands:
  single    Convert single HTML file to PDF file
  multiple  Convert multiple HTML files to multiple PDF files
  url       Convert single HTML page to PDF file
  help      Print this message or the help of the given subcommand(s)

Options:
  -b, --print-background             Print the page background
      --print-header-footer          Print the header and footer
      --header <HEADER>              Set custom header
      --header-file <HEADER_FILE>    Load custom header from file
      --footer <FOOTER>              Set custom footer
      --footer-file <FOOTER_FILE>    Load custom footer from file
  -l, --landscape                    Set the paper orientation to landscape
  -m, --margin <MARGIN>              Set the margin
  -p, --paper-format <FORMAT>        Set the paper format
      --paper-width <PAPER_WIDTH>    Set the custom paper width
      --paper-height <PAPER_HEIGHT>  Set the custom paper height
  -z, --paper-size <PAPER_SIZE>      Set the custom paper size
  -r, --ranges <RANGES>              Set the range of pages to print
  -s, --scale <SCALE>                Set the printing scale
  -J, --jpeg                         Save a screenshot as a JPEG image.
  -P, --png                          Save a screenshot as a PNG image.
  -W, --webp                         Save a screenshot as a WebP image.
  -w, --window-size <WINDOW_SIZE>    Set a window size for the screenshot.
  -v, --verbose                      Display printing details
      --log-level <LEVEL>            Set the logging level
      --no-crash-reports             Disable saving crash reports
  -t, --timeout <TIMEOUT>            Set page load timeout in milliseconds
  -h, --help                         Print help (see more with '--help')
  -V, --version                      Print version
"#,
  );
  tc.tear_down();
}
