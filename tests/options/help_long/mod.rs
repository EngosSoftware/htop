use super::*;
use assert_cmd::assert::OutputAssertExt;
use assert_cmd::cargo::CommandCargoExt;
use std::process::Command;

#[test]
fn _0001() {
  let tc = test_context!().set_up();
  let mut cmd = Command::cargo_bin("htop").unwrap();
  cmd.current_dir(tc.current_dir()).arg("--help").assert().stdout(
    r#"HTML to PDF converter

Usage: htop [OPTIONS] [COMMAND]

Commands:
  single    Convert single HTML file to PDF file
  multiple  Convert multiple HTML files to multiple PDF files
  url       Convert single HTML page to PDF file
  help      Print this message or the help of the given subcommand(s)

Options:
  -b, --print-background
          Prints the page background. When this option is present,
          the output PDF file will contain the background filled
          just like the input HTML file or website.

      --print-header-footer
          Prints the header and footer. When this option is present,
          the header and footer will be printed to PDF file.
          When no custom header and/or footer is set, then the
          default Chromium's header and/or footer is printed.

      --header <HEADER>
          Sets custom header. This option defines custom HTML template
          for the print header. This template must be valid HTML markup
          with the following classes used to inject printing values into them:
            date       - formatted date of printing,
            title      - document title,
            url        - document location,
            pageNumber - current page number,
            totalPages - total number of pages in the document.
          The same rules are valid for --footer option.

      --header-file <HEADER_FILE>
          Loads custom header from file. This option defines custom HTML template
          for the print header loaded from file.
          See --header option for a detailed description of the HTML template.

      --footer <FOOTER>
          Sets custom footer. The same rules as for --header option.

      --footer-file <FOOTER_FILE>
          Loads custom footer from file. This option defines custom HTML template
          for the print footer loaded from file.
          See --header option for a detailed description of the HTML template.

  -l, --landscape
          Sets the paper orientation to landscape. In landscape mode
          the longest paper edge is positioned in horizontal
          direction. The default paper orientation is portrait.

  -m, --margin <MARGIN>
          Sets the margin. The value of the margin
          should be specified like in CSS, e.g.:
            '0 12mm 3pt 4px' - sets the top margin to zero, right margin to 12mm,
                               bottom margin to 3pt and left margin to 4px,
            '1.5cm 2cm'      - sets the top and bottom margins to 1.5cm,
                               the left and right margins are set to 2cm,
            '15mm'           - sets all margins to 15mm.
          Allowed units are: cm, mm, Q, in, pc, pt, px.

  -p, --paper-format <FORMAT>
          Sets the paper format. Currently supported paper formats:
          A0 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11 A12 A13 2A0 4A0 A0+ A1+ A3+
          B0 B1 B2 B3 B4 B5 B6 B7 B8 B9 B10 B11 B12 B13 B0+ B1+ B2+
          C0 C1 C2 C3 C4 C5 C6 C7 C8 C9 C10
          Letter Legal Tabloid Ledger 'Junior Legal' 'Half Letter'
          'Government Letter' 'Government Legal'
          'Ansi A' 'Ansi B' 'Ansi C' 'Ansi D' 'Ansi E'
          'Arch A' 'Arch B' 'Arch C' 'Arch D' 'Arch E'
          'Arch E1' 'Arch E2' 'Arch E3'

      --paper-width <PAPER_WIDTH>
          Sets the custom paper width. Paper width should be specified
          using CSS units, e.g.: '100mm', '21cm', '800pt', etc.
          Allowed units are: cm, mm, Q, in, pc, pt, px.

      --paper-height <PAPER_HEIGHT>
          Sets the custom paper height. Paper height should be specified
          using CSS units, e.g.: '200mm', '29.7cm', '1200pt', etc.
          Allowed units are: cm, mm, Q, in, pc, pt, px.

  -z, --paper-size <PAPER_SIZE>
          Sets the custom paper size. Paper size should be specified
          as two values separated by comma using CSS units, e.g.: '210mm,297mm', '21cm,29.7cm', etc.
          Allowed units are: cm, mm, Q, in, pc, pt, px.

  -r, --ranges <RANGES>
          Sets the range of pages to print.
          Paper ranges start from 1, e.g.: '1-5, 8, 11-13'.
          Pages are printed in the document order.

  -s, --scale <SCALE>
          Sets the printing scale.
          Scale should be specified as a number or as a percentage.
          Accepted scales are in range from 0.1 to 2.0 (from 10% to 200%).
          The default scale is 1.0 (100%).

  -J, --jpeg
          Save a screenshot as a JPEG image.

  -P, --png
          Save a screenshot as a PNG image.

  -W, --webp
          Save a screenshot as a WebP image.

  -w, --window-size <WINDOW_SIZE>
          Set a window size for the screenshot.

  -v, --verbose
          Displays printing details. Prints to standard
          output informative messages about the printing process.

      --log-level <LEVEL>
          Sets the logging level. Allowed logging levels are:
            none,
            error,
            warn,
            info,
            debug,
            trace.
          The default logging level is 'none'.

      --no-crash-reports
          Disables saving crash reports. By default, Headless Chrome
          saves crash reports in user's home directory.
          This can be switched-off by specifying this option.
          When printing is run in multiuser environment or in the cloud,
          it may happen, that crash reports generated by one user
          clash with reports generated by other user. In such case
          the printing process hangs forever.

  -t, --timeout <TIMEOUT>
          Sets page load timeout in milliseconds. During printing
          webpages, when the page load timeout is reached, the printing
          process is cancelled and an error is reported.

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
"#,
  );
  tc.tear_down();
}
