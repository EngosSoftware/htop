use super::*;

fn assert_paper_format(suffix: &str, paper_format: &str, htop: &str, pdfinfo: &str) {
  let tc = test_context!().set_up().with_suffix(suffix);
  let mut cmd = Command::cargo_bin("htop").unwrap();
  cmd
    .current_dir(tc.current_dir())
    .arg("-v")
    .arg("--margin=0")
    .arg("--paper-format")
    .arg(paper_format)
    .arg("single")
    .arg("H_000010.html")
    .arg(tc.actual_name())
    .assert()
    .success();
  assert_eq!(
    htop,
    String::from_utf8(cmd.output().unwrap().stdout)
      .unwrap()
      .lines()
      .filter(|line| line.contains("[htop] Paper format: ") || line.contains("[htop] Page size:"))
      .collect::<Vec<_>>()
      .join("\n")
  );
  assert_eq!(pdfinfo, tc.pdf_info_page_size());
  tc.tear_down(true);
}

#[test]
fn _0001() {
  assert_paper_format(
    "b0",
    "B0",
    "[htop] Paper format: B0\n[htop] Page size: 1000 x 1414 mm,  39.4 x 55.7 in, 2834.64 x 4008.24 pts",
    "Page size:       2835.12 x 4008.96 pts",
  );
}

#[test]
fn _0002() {
  assert_paper_format(
    "b1",
    "B1",
    "[htop] Paper format: B1\n[htop] Page size: 707 x 1000 mm,  27.8 x 39.4 in, 2003.76 x 2834.64 pts",
    "Page size:       2004.96 x 2835.12 pts",
  );
}

#[test]
fn _0003() {
  assert_paper_format(
    "b2",
    "B2",
    "[htop] Paper format: B2\n[htop] Page size: 500 x 707 mm,  19.7 x 27.8 in, 1417.68 x 2003.76 pts",
    "Page size:       1417.92 x 2004.96 pts",
  );
}

#[test]
fn _0004() {
  assert_paper_format(
    "b3",
    "B3",
    "[htop] Paper format: B3\n[htop] Page size: 353 x 500 mm,  13.9 x 19.7 in, 1000.8 x 1417.68 pts",
    "Page size:       1001.04 x 1417.92 pts",
  );
}

#[test]
fn _0005() {
  assert_paper_format(
    "b4",
    "B4",
    "[htop] Paper format: B4\n[htop] Page size: 250 x 353 mm,  9.8 x 13.9 in, 708.48 x 1000.8 pts",
    "Page size:       708.96 x 1001.04 pts",
  );
}

#[test]
fn _0006() {
  assert_paper_format(
    "b5",
    "B5",
    "[htop] Paper format: B5\n[htop] Page size: 176 x 250 mm,  6.9 x 9.8 in, 498.96 x 708.48 pts",
    "Page size:       498.96 x 708.96 pts",
  );
}

#[test]
fn _0007() {
  assert_paper_format(
    "b6",
    "B6",
    "[htop] Paper format: B6\n[htop] Page size: 125 x 176 mm,  4.9 x 6.9 in, 354.24 x 498.96 pts",
    "Page size:       354.96 x 498.96 pts",
  );
}

#[test]
fn _0008() {
  assert_paper_format(
    "b7",
    "B7",
    "[htop] Paper format: B7\n[htop] Page size: 88 x 125 mm,  3.5 x 4.9 in, 249.12 x 354.24 pts",
    "Page size:       250.08 x 354.96 pts",
  );
}

#[test]
fn _0009() {
  assert_paper_format(
    "b8",
    "B8",
    "[htop] Paper format: B8\n[htop] Page size: 62 x 88 mm,  2.4 x 3.5 in, 175.68 x 249.12 pts",
    "Page size:       175.92 x 250.08 pts",
  );
}

#[test]
fn _0010() {
  assert_paper_format(
    "b9",
    "B9",
    "[htop] Paper format: B9\n[htop] Page size: 44 x 62 mm,  1.7 x 2.4 in, 124.56 x 175.68 pts",
    "Page size:       125.04 x 175.92 pts",
  );
}

#[test]
fn _0011() {
  assert_paper_format(
    "b10",
    "B10",
    "[htop] Paper format: B10\n[htop] Page size: 31 x 44 mm,  1.2 x 1.7 in, 87.84 x 124.56 pts",
    "Page size:       88.08 x 125.04 pts",
  );
}

#[test]
fn _0012() {
  assert_paper_format(
    "b11",
    "B11",
    "[htop] Paper format: B11\n[htop] Page size: 22 x 31 mm,  0.9 x 1.2 in, 62.64 x 87.84 pts",
    "Page size:       63.12 x 88.08 pts",
  );
}

#[test]
fn _0013() {
  assert_paper_format(
    "b12",
    "B12",
    "[htop] Paper format: B12\n[htop] Page size: 15 x 22 mm,  0.6 x 0.9 in, 42.48 x 62.64 pts",
    "Page size:       42.96 x 63.12 pts",
  );
}

#[test]
fn _0014() {
  assert_paper_format(
    "b13",
    "B13",
    "[htop] Paper format: B13\n[htop] Page size: 11 x 15 mm,  0.4 x 0.6 in, 30.96 x 42.48 pts",
    "Page size:       31.92 x 42.96 pts",
  );
}

#[test]
fn _0015() {
  assert_paper_format(
    "b0plus",
    "B0+",
    "[htop] Paper format: B0+\n[htop] Page size: 1118 x 1580 mm,  44 x 62.2 in, 3169.44 x 4478.4 pts",
    "Page size:       3169.92 x 4479.12 pts",
  );
}

#[test]
fn _0016() {
  assert_paper_format(
    "b1plus",
    "B1+",
    "[htop] Paper format: B1+\n[htop] Page size: 720 x 1020 mm,  28.3 x 40.2 in, 2041.2 x 2891.52 pts",
    "Page size:       2040.96 x 2892 pts",
  );
}

#[test]
fn _0017() {
  assert_paper_format(
    "b2plus",
    "B2+",
    "[htop] Paper format: B2+\n[htop] Page size: 520 x 720 mm,  20.5 x 28.3 in, 1473.84 x 2041.2 pts",
    "Page size:       1475.04 x 2040.96 pts",
  );
}
