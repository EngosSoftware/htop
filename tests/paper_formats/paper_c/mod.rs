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
    "c0",
    "C0",
    "[htop] Paper format: C0\n[htop] Page size: 917 x 1297 mm,  36.1 x 51.1 in, 2599.2 x 3676.32 pts",
    "Page size:       2599.92 x 3677.04 pts",
  );
}

#[test]
fn _0002() {
  assert_paper_format(
    "c1",
    "C1",
    "[htop] Paper format: C1\n[htop] Page size: 648 x 917 mm,  25.5 x 36.1 in, 1836.72 x 2599.2 pts",
    "Page size:       1836.96 x 2599.92 pts",
  );
}

#[test]
fn _0003() {
  assert_paper_format(
    "c2",
    "C2",
    "[htop] Paper format: C2\n[htop] Page size: 458 x 648 mm,  18 x 25.5 in, 1298.16 x 1836.72 pts",
    "Page size:       1299.12 x 1836.96 pts",
  );
}

#[test]
fn _0004() {
  assert_paper_format(
    "c3",
    "C3",
    "[htop] Paper format: C3\n[htop] Page size: 324 x 458 mm,  12.8 x 18 in, 918.72 x 1298.16 pts",
    "Page size:       918.96 x 1299.12 pts",
  );
}

#[test]
fn _0005() {
  assert_paper_format(
    "c4",
    "C4",
    "[htop] Paper format: C4\n[htop] Page size: 229 x 324 mm,  9 x 12.8 in, 649.44 x 918.72 pts",
    "Page size:       649.92 x 918.96 pts",
  );
}

#[test]
fn _0006() {
  assert_paper_format(
    "c5",
    "C5",
    "[htop] Paper format: C5\n[htop] Page size: 162 x 229 mm,  6.4 x 9 in, 459.36 x 649.44 pts",
    "Page size:       460.08 x 649.92 pts",
  );
}

#[test]
fn _0007() {
  assert_paper_format(
    "c6",
    "C6",
    "[htop] Paper format: C6\n[htop] Page size: 114 x 162 mm,  4.5 x 6.4 in, 323.28 x 459.36 pts",
    "Page size:       324 x 460.08 pts",
  );
}

#[test]
fn _0008() {
  assert_paper_format(
    "c7",
    "C7",
    "[htop] Paper format: C7\n[htop] Page size: 81 x 114 mm,  3.2 x 4.5 in, 229.68 x 323.28 pts",
    "Page size:       229.92 x 324 pts",
  );
}

#[test]
fn _0009() {
  assert_paper_format(
    "c8",
    "C8",
    "[htop] Paper format: C8\n[htop] Page size: 57 x 81 mm,  2.2 x 3.2 in, 161.28 x 229.68 pts",
    "Page size:       162 x 229.92 pts",
  );
}

#[test]
fn _0010() {
  assert_paper_format(
    "c9",
    "C9",
    "[htop] Paper format: C9\n[htop] Page size: 40 x 57 mm,  1.6 x 2.2 in, 113.04 x 161.28 pts",
    "Page size:       114 x 162 pts",
  );
}

#[test]
fn _0011() {
  assert_paper_format(
    "c10",
    "C10",
    "[htop] Paper format: C10\n[htop] Page size: 28 x 40 mm,  1.1 x 1.6 in, 79.2 x 113.04 pts",
    "Page size:       79.92 x 114 pts",
  );
}
