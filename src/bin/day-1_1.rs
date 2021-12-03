fn main() {
  let depths = advent_2021::stdin_collect::<i32>().unwrap();
  let (_, increases, ) = depths.iter()
    .fold((depths[0], 0, ), |(prev, i, ), depth| {
      (*depth, if *depth > prev {
        i + 1
      } else {
        i
      }, )
    });
  println!("Increases: {}", increases);
}