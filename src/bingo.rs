use std::io::{self, BufRead};

use array2d::Array2D;

pub const BOARD_X: usize = 5;
pub const BOARD_Y: usize = 5;
pub const BOARD_SZ: usize = BOARD_X * BOARD_Y;

pub type BingoBoard = Array2D<BingoTile>;
pub type BingoValue = i32;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BingoTile {
  Unmarked(BingoValue),
  Marked(BingoValue),
}

impl BingoTile {
  fn marked(t: &BingoTile) -> bool {
    match t {
      BingoTile::Marked(_) => true,
      _ => false
    }
  }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BingoDirection {
  X(usize),
  Y(usize),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BingoResult {
  Bingo(BingoDirection),
  NoBingo,
}

pub trait BingoGame {
  fn mark(&mut self, value: BingoValue) -> BingoResult;
  fn check_bingo(&self, y: usize, x: usize) -> BingoResult;
  fn sum_unmarked(&self) -> BingoValue;
}

impl BingoGame for BingoBoard {
  fn mark(&mut self, value: BingoValue) -> BingoResult {
    for y in 0..BOARD_Y {
      for x in 0..BOARD_X {
        if self[(y, x, )] == BingoTile::Unmarked(value) {
          self[(y, x, )] = BingoTile::Marked(value);
          let bingo = self.check_bingo(y, x);
          match bingo {
            BingoResult::Bingo(_) => return bingo,
            _ => ()
          }
        }
      }
    }
    BingoResult::NoBingo
  }

  fn check_bingo(&self, y: usize, x: usize) -> BingoResult {
    if self.row_iter(y).all(BingoTile::marked) {
      BingoResult::Bingo(BingoDirection::X(y))
    } else if self.column_iter(x).all(BingoTile::marked) {
      BingoResult::Bingo(BingoDirection::Y(x))
    } else {
      BingoResult::NoBingo
    }
  }

  fn sum_unmarked(&self) -> BingoValue {
    self.elements_row_major_iter()
      .map(|t| match t {
        BingoTile::Unmarked(n) => *n,
        _ => 0
      }).sum()
  }
}

pub fn stdin_bingo_draw() -> Option<Vec<BingoValue>> {
  for line in io::stdin().lock().lines() {
    let line = line.ok()?;
    if !line.contains(',') {
      continue;
    }

    return line.split(',')
      .map(|n| n.parse().ok())
      .collect::<Option<Vec<BingoValue>>>();
  }

  None
}

pub fn stdin_bingo_boards() -> Option<Vec<BingoBoard>> {
  let mut accum: Vec<BingoTile> = Vec::new();
  for line in io::stdin().lock().lines() {
    match line {
      Ok(line) => {
        let line = line.trim();
        if line.len() <= 0 {
          continue;
        }
        let mut tiles = line.split_whitespace()
          .map(|n| n.parse().ok())
          .collect::<Option<Vec<BingoValue>>>()?
          .into_iter()
          .map(|n| BingoTile::Unmarked(n))
          .collect::<Vec<BingoTile>>();
        accum.append(&mut tiles);
      }
      Err(_) => return None
    }
  }

  Some(accum
    .chunks(BOARD_SZ)
    .map(|v| BingoBoard::from_row_major(v,
                                        BOARD_Y,
                                        BOARD_X))
    .collect::<Vec<BingoBoard>>())
}