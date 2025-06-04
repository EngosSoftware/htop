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
  tc.tear_down();
}

#[test]
fn _0002() {
  assert_paper_format(
    "letter",
    "Letter",
    "[htop] Paper format: Letter\n[htop] Page size: 216 x 279 mm,  8.5 x 11 in, 612 x 790.56 pts",
    "Page size:       612.96 x 791.04 pts (letter)",
  );
}

#[test]
fn _0003() {
  assert_paper_format(
    "legal",
    "Legal",
    "[htop] Paper format: Legal\n[htop] Page size: 216 x 356 mm,  8.5 x 14 in, 612 x 1009.44 pts",
    "Page size:       612.96 x 1009.92 pts",
  );
}

#[test]
fn _0004() {
  assert_paper_format(
    "tabloid",
    "Tabloid",
    "[htop] Paper format: Tabloid\n[htop] Page size: 279 x 432 mm,  11 x 17 in, 790.56 x 1224.72 pts",
    "Page size:       791.04 x 1224.96 pts",
  );
}

#[test]
fn _0005() {
  assert_paper_format(
    "ledger",
    "Ledger",
    "[htop] Paper format: Ledger\n[htop] Page size: 432 x 279 mm,  17 x 11 in, 1224.72 x 790.56 pts",
    "Page size:       1224.96 x 791.04 pts",
  );
}

#[test]
fn _0006() {
  assert_paper_format(
    "junior-legal",
    "Junior Legal",
    "[htop] Paper format: Junior Legal\n[htop] Page size: 127 x 203 mm,  5 x 8 in, 360 x 575.28 pts",
    "Page size:       360 x 576 pts",
  );
}

#[test]
fn _0007() {
  assert_paper_format(
    "half-letter",
    "Half Letter",
    "[htop] Paper format: Half Letter\n[htop] Page size: 140 x 216 mm,  5.5 x 8.5 in, 396.72 x 612 pts",
    "Page size:       396.96 x 612.96 pts",
  );
}

#[test]
fn _0008() {
  assert_paper_format(
    "government-letter",
    "Government Letter",
    "[htop] Paper format: Government Letter\n[htop] Page size: 203 x 267 mm,  8 x 10.5 in, 575.28 x 756.72 pts",
    "Page size:       576 x 756.96 pts",
  );
}

#[test]
fn _0009() {
  assert_paper_format(
    "government-legal",
    "Government Legal",
    "[htop] Paper format: Government Legal\n[htop] Page size: 216 x 330 mm,  8.5 x 13 in, 612 x 935.28 pts",
    "Page size:       612.96 x 936 pts",
  );
}

#[test]
fn _0010() {
  assert_paper_format(
    "ansi-a",
    "Ansi A",
    "[htop] Paper format: Ansi A\n[htop] Page size: 216 x 279 mm,  8.5 x 11 in, 612 x 790.56 pts",
    "Page size:       612.96 x 791.04 pts (letter)",
  );
}

#[test]
fn _0011() {
  assert_paper_format(
    "ansi-b",
    "Ansi B",
    "[htop] Paper format: Ansi B\n[htop] Page size: 279 x 432 mm,  11 x 17 in, 790.56 x 1224.72 pts",
    "Page size:       791.04 x 1224.96 pts",
  );
}

#[test]
fn _0012() {
  assert_paper_format(
    "ansi-c",
    "Ansi C",
    "[htop] Paper format: Ansi C\n[htop] Page size: 432 x 559 mm,  17 x 22 in, 1224.72 x 1584.72 pts",
    "Page size:       1224.96 x 1584.96 pts",
  );
}

#[test]
fn _0013() {
  assert_paper_format(
    "ansi-d",
    "Ansi D",
    "[htop] Paper format: Ansi D\n[htop] Page size: 559 x 864 mm,  22 x 34 in, 1584.72 x 2449.44 pts",
    "Page size:       1584.96 x 2449.92 pts",
  );
}

#[test]
fn _0014() {
  assert_paper_format(
    "ansi-e",
    "Ansi E",
    "[htop] Paper format: Ansi E\n[htop] Page size: 864 x 1118 mm,  34 x 44 in, 2449.44 x 3169.44 pts",
    "Page size:       2449.92 x 3169.92 pts",
  );
}

#[test]
fn _0015() {
  assert_paper_format(
    "arch-a",
    "Arch A",
    "[htop] Paper format: Arch A\n[htop] Page size: 229 x 305 mm,  9 x 12 in, 649.44 x 864.72 pts",
    "Page size:       649.92 x 864.96 pts",
  );
}

#[test]
fn _0016() {
  assert_paper_format(
    "arch-b",
    "Arch B",
    "[htop] Paper format: Arch B\n[htop] Page size: 305 x 457 mm,  12 x 18 in, 864.72 x 1295.28 pts",
    "Page size:       864.96 x 1296 pts",
  );
}

#[test]
fn _0017() {
  assert_paper_format(
    "arch-c",
    "Arch C",
    "[htop] Paper format: Arch C\n[htop] Page size: 457 x 610 mm,  18 x 24 in, 1295.28 x 1729.44 pts",
    "Page size:       1296 x 1729.92 pts",
  );
}

#[test]
fn _0018() {
  assert_paper_format(
    "arch-d",
    "Arch D",
    "[htop] Paper format: Arch D\n[htop] Page size: 610 x 914 mm,  24 x 36 in, 1729.44 x 2590.56 pts",
    "Page size:       1729.92 x 2591.04 pts",
  );
}

#[test]
fn _0019() {
  assert_paper_format(
    "arch-e",
    "Arch E",
    "[htop] Paper format: Arch E\n[htop] Page size: 914 x 1219 mm,  36 x 48 in, 2590.56 x 3455.28 pts",
    "Page size:       2591.04 x 3456 pts",
  );
}

#[test]
fn _0020() {
  assert_paper_format(
    "arch-e1",
    "Arch E1",
    "[htop] Paper format: Arch E1\n[htop] Page size: 762 x 1067 mm,  30 x 42 in, 2160 x 3024.72 pts",
    "Page size:       2160 x 3024.96 pts",
  );
}

#[test]
fn _0021() {
  assert_paper_format(
    "arch-e2",
    "Arch E2",
    "[htop] Paper format: Arch E2\n[htop] Page size: 660 x 965 mm,  26 x 38 in, 1870.56 x 2735.28 pts",
    "Page size:       1871.04 x 2736 pts",
  );
}

#[test]
fn _0022() {
  assert_paper_format(
    "arch-e3",
    "Arch E3",
    "[htop] Paper format: Arch E3\n[htop] Page size: 686 x 991 mm,  27 x 39 in, 1944.72 x 2809.44 pts",
    "Page size:       1944.96 x 2809.92 pts",
  );
}
