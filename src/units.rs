//! # Utilities for parsing lengths

use crate::defs::str_to_f64;
use crate::errors::{err_invalid_length, Result};

/// CSS units.
enum CssUnit {
  /// Centimeters, 1`cm` = 37.8`px` = 25.2/64`in`
  CM,
  /// Millimeters, 1`mm` = 1/10`cm`
  MM,
  /// Quarter-millimeters, 1`Q` = 1/40`cm`
  Q,
  /// Inches, 1`in` = 2.54`cm` = 96`px`
  IN,
  /// Picas, 1`pc` = 1/6`in`
  PC,
  /// Points, 1`pt` = 1/72`in`
  PT,
  /// Pixels, 1`px` = 1/96`in`
  PX,
}

/// Converts multiple CSS absolute lengths into inches.
pub fn inches(lengths: Vec<String>) -> Result<Vec<f64>> {
  lengths.iter().map(|s| to_inches(s)).collect()
}

/// Converts CSS absolute length into inches.
pub fn to_inches(length: &str) -> Result<f64> {
  // process lengths with units
  if let Some(prefix) = length.strip_suffix("cm") {
    return unit_to_inches(prefix, CssUnit::CM);
  }
  if let Some(prefix) = length.strip_suffix("mm") {
    return unit_to_inches(prefix, CssUnit::MM);
  }
  if let Some(prefix) = length.strip_suffix('Q') {
    return unit_to_inches(prefix, CssUnit::Q);
  }
  if let Some(prefix) = length.strip_suffix("in") {
    return unit_to_inches(prefix, CssUnit::IN);
  }
  if let Some(prefix) = length.strip_suffix("pc") {
    return unit_to_inches(prefix, CssUnit::PC);
  }
  if let Some(prefix) = length.strip_suffix("pt") {
    return unit_to_inches(prefix, CssUnit::PT);
  }
  if let Some(prefix) = length.strip_suffix("px") {
    return unit_to_inches(prefix, CssUnit::PX);
  }
  // the only valid length without unit is zero
  if str_to_f64(length)? < f64::EPSILON {
    return Ok(0.0);
  }
  // report invalid length
  Err(err_invalid_length(length))
}

/// Converts inches into millimeters.
pub fn inches_to_millimeters(inches: f64) -> f64 {
  (inches * 25.4).round()
}

/// Converts inches into points.
pub fn inches_to_points(inches: f64) -> f64 {
  round2(round2(inches) * 72.0)
}

pub fn round1(value: f64) -> f64 {
  (value * 10.0).round() / 10.0
}

pub fn round2(value: f64) -> f64 {
  (value * 100.0).round() / 100.0
}

/// Converts a value expressed in specified units into inches.
fn unit_to_inches(s: &str, unit: CssUnit) -> Result<f64> {
  let value = str_to_f64(s)?;
  Ok(match unit {
    CssUnit::CM => value * 25.2 / 64.0,
    CssUnit::MM => value * 2.52 / 64.0,
    CssUnit::Q => value * 2.52 / 256.0,
    CssUnit::IN => value,
    CssUnit::PC => value / 6.0,
    CssUnit::PT => value / 72.0,
    CssUnit::PX => value / 96.0,
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  fn eq(expected: f64, actual: f64) {
    assert!((expected - (actual * 1000.0).round() / 1000.0).abs() < f64::EPSILON);
  }

  #[test]
  fn test_length_to_inches() {
    eq(0.0, to_inches("0").unwrap());
    eq(0.0, to_inches("0.0").unwrap());
    eq(1.0, to_inches("2.54cm").unwrap());
    eq(-1.0, to_inches("-2.54cm").unwrap());
    eq(1.0, to_inches("254e-2cm").unwrap());
    eq(-1.0, to_inches("-254e-2cm").unwrap());
    eq(1.0, to_inches("25.4mm").unwrap());
    eq(1.0, to_inches("101.6Q").unwrap());
    eq(1.0, to_inches("1in").unwrap());
    eq(1.0, to_inches("6pc").unwrap());
    eq(1.0, to_inches("72pt").unwrap());
    eq(1.0, to_inches("96px").unwrap());
    assert!(to_inches("9a6px").is_err());
    assert!(to_inches("20").is_err());
    assert!(to_inches("56.23").is_err());
  }
}
