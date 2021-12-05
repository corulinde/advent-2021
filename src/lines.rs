use std::str::FromStr;

use array2d::Array2D;
use regex::Regex;

use super::Point2Di32;

lazy_static! {
  static ref LINE_PAT: String = format!(r"^({})\s*->\s*({})$", Point2Di32::PATTERN, Point2Di32::PATTERN);
  static ref LINE_RE: Regex = Regex::new(LINE_PAT.as_str()).unwrap();
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Linei32 {
  pub p0: Point2Di32,
  pub p1: Point2Di32,
}

impl Linei32 {
  pub fn is_straight(&self) -> bool {
    self.p0.x == self.p1.x || self.p0.y == self.p1.y
  }
}

#[derive(Debug)]
pub struct Canvasi32 {
  pub bound_x: usize,
  pub bound_y: usize,
  pub pixels: Array2D<i32>,
}

impl FromStr for Linei32 {
  type Err = String;

  fn from_str(value: &str) -> Result<Self, Self::Err> {
    match LINE_RE.captures(value) {
      Some(c) => Ok(Self { p0: c[1].parse()?, p1: c[2].parse()? }),
      _ => Err(format!("Unable to parse line: '{}'", value))
    }
  }
}

pub fn bounds(lines: &Vec<Linei32>) -> Option<(Point2Di32, Point2Di32, )> {
  let n_lines = lines.len();
  if n_lines <= 0 {
    return None;
  }
  let mut points: Vec<Point2Di32> = Vec::with_capacity(n_lines * 2);
  for line in lines {
    points.push(line.p0);
    points.push(line.p1);
  }
  Point2Di32::min_max(points.as_slice())
}

impl Canvasi32 {
  pub fn print(&self) {
    for y in 0..self.bound_y {
      println!("{}", self.pixels.row_iter(y)
        .map(|n| n.to_string())
        .collect::<Vec<String>>().join(" "));
    }
  }

  pub fn new(bound_x: usize, bound_y: usize) -> Self {
    Self {
      bound_x,
      bound_y,
      pixels: Array2D::filled_with(0, bound_y, bound_x),
    }
  }

  pub fn draw_line(&mut self, l: &Linei32) {
    let Linei32 { p0: Point2Di32 { x: mut x0, y: mut y0 }, p1: Point2Di32 { x: x1, y: y1 } } = l;

    let dx = (x1 - x0).abs();
    let sx = if x0 < *x1 { 1 } else { -1 };
    let dy = -((y1 - y0).abs());
    let sy = if y0 < *y1 { 1 } else { -1 };
    let mut err = dx + dy;
    let mut last_x = None;
    let mut last_y = None;
    loop {
      if !(last_x == Some(x0) && last_y == Some(y0)) {
        self.pixels[(y0 as usize, x0 as usize, )] += 1;
        last_x = Some(x0);
        last_y = Some(y0);
      }
      if x0 == *x1 && y0 == *y1 { break; }
      let e2 = err * 2;
      if e2 > dy {
        err += dy;
        x0 += sx;
      }
      if e2 < dx {
        err += dx;
        y0 += sy;
      }
    }
  }
}