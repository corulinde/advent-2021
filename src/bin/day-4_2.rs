use advent_2021::bingo::*;

struct BingoBoardTracker {
  bingo: bool,
  board: BingoBoard,
}

fn main() {
  let balls = stdin_bingo_draw().unwrap();
  let mut boards = stdin_bingo_boards().unwrap().into_iter()
    .map(|b| BingoBoardTracker { bingo: false, board: b })
    .collect::<Vec<BingoBoardTracker>>();
  let n_boards = boards.len();

  let mut last_won: Option<(usize, BingoValue, )> = None;
  for ball in balls {
    for board_idx in 0..n_boards {
      let tracker = &mut boards[board_idx];
      if tracker.bingo {
        continue;
      }
      let board = &mut tracker.board;
      let result = board.mark(ball);
      match result {
        BingoResult::Bingo(_) => {
          tracker.bingo = true;
          last_won = Some((board_idx, ball, ));
        }
        _ => ()
      }
    }
  }

  let (board_idx, ball, ) = last_won.unwrap();
  let sum = boards[board_idx].board.sum_unmarked();
  println!("Board: {} Ball: {} Sum: {} Ball*Sum: {}", board_idx + 1, ball, sum, ball * sum);
}