use advent_2021::lines::*;
use advent_2021::Point2Di32;

fn main() {
  let lines = advent_2021::stdin().unwrap().into_iter()
    .map(|l| l.parse().unwrap())
    .collect::<Vec<Linei32>>();
  let (_, Point2Di32 { x: bound_x, y: bound_y }) = bounds(&lines).unwrap();

  let mut canvas = Canvasi32::new(bound_x as usize + 1, bound_y as usize + 1);
  for line in lines.into_iter().filter(|l| l.is_straight()) {
    canvas.draw_line(&line);
  }

  let dangerous = canvas.pixels.elements_row_major_iter()
    .filter(|x| **x >= 2)
    .count();
  println!("Dangerous: {:?}", dangerous);
}