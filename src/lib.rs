use std::io::{self, BufRead};
use std::str::FromStr;

pub mod bingo;

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