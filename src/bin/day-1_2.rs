fn main() {
  let depth_win = advent_2021::stdin_collect::<i32>().unwrap()
    .windows(3)
    .map(|w| w.iter().sum())
    .collect::<Vec<i32>>();
  let (_, increases, ) = depth_win.iter()
    .fold((depth_win[0], 0, ), |(prev, i, ), depth| {
      (*depth, if *depth > prev {
        i + 1
      } else {
        i
      }, )
    });
  println!("Increases: {}", increases);
}