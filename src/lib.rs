#[macro_use]
extern crate lazy_static;

use std::io::{self, BufRead};
use std::str::FromStr;

pub mod bingo;
pub mod lines;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Point2Di32 {
  pub x: i32,
  pub y: i32,
}

impl Point2Di32 {
  pub const PATTERN: &'static str = r"\d+\s*,\s*\d+";
}

impl Point2Di32 {
  pub fn min_max(ps: &[Point2Di32]) -> Option<(Point2Di32, Point2Di32, )> {
    if ps.len() <= 0 {
      return None;
    }
    let mut min: Point2Di32 = ps[0];
    let mut max: Point2Di32 = ps[0];
    for p in ps {
      if p.x < min.x {
        min.x = p.x;
      } else if p.x > max.x {
        max.x = p.x;
      }
      if p.y < min.y {
        min.y = p.y;
      } else if p.y > max.y {
        max.y = p.y;
      }
    }
    Some((min, max, ))
  }
}

impl FromStr for Point2Di32 {
  type Err = String;

  fn from_str(value: &str) -> Result<Self, Self::Err> {
    let pair = value.trim().split(',').collect::<Vec<&str>>();

    if pair.len() != 2 {
      return Err(format!("'{}' did not contain two elements", value));
    }

    match pair.into_iter()
      .map(|s| s.parse().ok())
      .collect::<Vec<Option<i32>>>()
      .as_slice() {
      [Some(x), Some(y)] => Ok(Point2Di32 { x: *x, y: *y }),
      _ => Err(format!("Failed to parse '{}'", value))
    }
  }
}

pub fn stdin() -> Option<Vec<String>> {
  io::stdin().lock()
    .lines()
    .map(|l| l.ok())
    .collect()
}

pub fn stdin_collect<T>() -> Option<Vec<T>>
  where T: FromStr {
  stdin()?.into_iter()
    .map(|l| l.parse().ok())
    .collect::<Option<Vec<T>>>()
}

pub fn stdin_tok_ws() -> Option<Vec<Vec<String>>> {
  Some(stdin()?.into_iter()
    .map(|l| l.split_whitespace()
      .into_iter()
      .map(|sw| sw.to_string())
      .collect::<Vec<String>>())
    .collect::<Vec<Vec<String>>>())
}

pub fn bincharvec_to_unsigned(v: Vec<u8>) -> u64 {
  let mut n: u64 = 0;
  let len = v.len();
  for x in 0..len {
    n |= (v[x] as u64) << (len - x - 1);
  }
  n
}