use super::*;

fn assert_paper_format(suffix: &str, paper_format: &str, htop: &str, pdfinfo: &str) {
  let tc = test_context!().set_up().with_suffix(suffix);
  let mut cmd = tc.command();
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
  tc.tear_down();
}

#[test]
fn _0001() {
  assert_paper_format(
    "a0",
    "A0",
    "[htop] Paper format: A0\n[htop] Page size: 841 x 1189 mm,  33.1 x 46.8 in, 2383.92 x 3370.32 pts",
    "Page size:       2383.92 x 3371.04 pts (A0)",
  );
}

#[test]
fn _0002() {
  assert_paper_format(
    "a1",
    "A1",
    "[htop] Paper format: A1\n[htop] Page size: 594 x 841 mm,  23.4 x 33.1 in, 1684.08 x 2383.92 pts",
    "Page size:       1684.08 x 2383.92 pts (A1)",
  );
}

#[test]
fn _0003() {
  assert_paper_format(
    "a2",
    "A2",
    "[htop] Paper format: A2\n[htop] Page size: 420 x 594 mm,  16.5 x 23.4 in, 1190.88 x 1684.08 pts",
    "Page size:       1191.12 x 1684.08 pts (A2)",
  );
}

#[test]
fn _0004() {
  assert_paper_format(
    "a3",
    "A3",
    "[htop] Paper format: A3\n[htop] Page size: 297 x 420 mm,  11.7 x 16.5 in, 841.68 x 1190.88 pts",
    "Page size:       841.92 x 1191.12 pts (A3)",
  );
}

#[test]
fn _0005() {
  assert_paper_format(
    "a4",
    "A4",
    "[htop] Paper format: A4\n[htop] Page size: 210 x 297 mm,  8.3 x 11.7 in, 595.44 x 841.68 pts",
    "Page size:       595.92 x 841.92 pts (A4)",
  );
}
#[test]
fn _0006() {
  assert_paper_format(
    "a5",
    "A5",
    "[htop] Paper format: A5\n[htop] Page size: 148 x 210 mm,  5.8 x 8.3 in, 419.76 x 595.44 pts",
    "Page size:       420 x 595.92 pts (A5)",
  );
}
#[test]
fn _0007() {
  assert_paper_format(
    "a6",
    "A6",
    "[htop] Paper format: A6\n[htop] Page size: 105 x 148 mm,  4.1 x 5.8 in, 297.36 x 419.76 pts",
    "Page size:       298.08 x 420 pts",
  );
}
#[test]
fn _0008() {
  assert_paper_format(
    "a7",
    "A7",
    "[htop] Paper format: A7\n[htop] Page size: 74 x 105 mm,  2.9 x 4.1 in, 209.52 x 297.36 pts",
    "Page size:       210 x 298.08 pts",
  );
}
#[test]
fn _0009() {
  assert_paper_format(
    "a8",
    "A8",
    "[htop] Paper format: A8\n[htop] Page size: 52 x 74 mm,  2 x 2.9 in, 147.6 x 209.52 pts",
    "Page size:       148.08 x 210 pts",
  );
}
#[test]
fn _0010() {
  assert_paper_format(
    "a9",
    "A9",
    "[htop] Paper format: A9\n[htop] Page size: 37 x 52 mm,  1.5 x 2 in, 105.12 x 147.6 pts",
    "Page size:       104.88 x 148.08 pts",
  );
}
#[test]
fn _0011() {
  assert_paper_format(
    "a10",
    "A10",
    "[htop] Paper format: A10\n[htop] Page size: 26 x 37 mm,  1 x 1.5 in, 73.44 x 105.12 pts",
    "Page size:       73.92 x 104.88 pts",
  );
}
#[test]
fn _0012() {
  assert_paper_format(
    "a11",
    "A11",
    "[htop] Paper format: A11\n[htop] Page size: 18 x 26 mm,  0.7 x 1 in, 51.12 x 73.44 pts",
    "Page size:       52.08 x 73.92 pts",
  );
}
#[test]
fn _0013() {
  assert_paper_format(
    "a12",
    "A12",
    "[htop] Paper format: A12\n[htop] Page size: 13 x 18 mm,  0.5 x 0.7 in, 36.72 x 51.12 pts",
    "Page size:       36.96 x 52.08 pts",
  );
}
#[test]
fn _0014() {
  assert_paper_format(
    "a13",
    "A13",
    "[htop] Paper format: A13\n[htop] Page size: 9 x 13 mm,  0.4 x 0.5 in, 25.2 x 36.72 pts",
    "Page size:       25.92 x 36.96 pts",
  );
}
#[test]
fn _0015() {
  assert_paper_format(
    "2a0",
    "2A0",
    "[htop] Paper format: 2A0\n[htop] Page size: 1189 x 1682 mm,  46.8 x 66.2 in, 3370.32 x 4767.84 pts",
    "Page size:       3371.04 x 4768.08 pts",
  );
}
#[test]
fn _0016() {
  assert_paper_format(
    "4a0",
    "4A0",
    "[htop] Paper format: 4A0\n[htop] Page size: 1682 x 2378 mm,  66.2 x 93.6 in, 4767.84 x 6740.64 pts",
    "Page size:       4768.08 x 6740.88 pts",
  );
}
#[test]
fn _0017() {
  assert_paper_format(
    "a0plus",
    "A0+",
    "[htop] Paper format: A0+\n[htop] Page size: 914 x 1292 mm,  36 x 50.9 in, 2590.56 x 3662.64 pts",
    "Page size:       2591.04 x 3662.88 pts",
  );
}
#[test]
fn _0018() {
  assert_paper_format(
    "a1plus",
    "A1+",
    "[htop] Paper format: A1+\n[htop] Page size: 609 x 914 mm,  24 x 36 in, 1726.56 x 2590.56 pts",
    "Page size:       1727.04 x 2591.04 pts",
  );
}
#[test]
fn _0019() {
  assert_paper_format(
    "a3plus",
    "A3+",
    "[htop] Paper format: A3+\n[htop] Page size: 329 x 483 mm,  13 x 19 in, 932.4 x 1369.44 pts",
    "Page size:       932.88 x 1369.92 pts",
  );
}
