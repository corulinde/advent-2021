use advent_2021::bingo::*;

fn main() {
  let balls = stdin_bingo_draw().unwrap();
  let mut boards = stdin_bingo_boards().unwrap();

  for ball in balls {
    for board in &mut boards {
      let result = board.mark(ball);
      match result {
        BingoResult::Bingo(_) => {
          let sum = board.sum_unmarked();
          println!("Ball: {} Sum: {} Ball*Sum: {}", ball, sum, ball * sum);
          return;
        }
        _ => ()
      }
    }
  }
}