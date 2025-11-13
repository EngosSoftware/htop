use super::*;

mod single_jpeg;
mod single_png;
mod single_webp;

#[test]
fn _0001() {
  let mut cmd = Command::new(cargo_bin!());
  cmd
    .arg("-W")
    .arg("--window-size=1200,1200cm")
    .arg("url")
    .arg("https://engos.de")
    .assert()
    .code(1)
    .stdout("")
    .stderr("Error: invalid height: 1200cm\n");
}

#[test]
fn _0002() {
  let mut cmd = Command::new(cargo_bin!());
  cmd
    .arg("--png")
    .arg("--window-size=1200cm,1200")
    .arg("url")
    .arg("https://engos.de")
    .assert()
    .code(1)
    .stdout("")
    .stderr("Error: invalid width: 1200cm\n");
}

#[test]
fn _0003() {
  let mut cmd = Command::new(cargo_bin!());
  cmd
    .arg("--jpeg")
    .arg("--window-size=1200:1200")
    .arg("url")
    .arg("https://engos.de")
    .assert()
    .code(1)
    .stdout("")
    .stderr("Error: invalid window size: 1200:1200\n");
}
