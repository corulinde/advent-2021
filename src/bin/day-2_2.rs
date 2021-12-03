fn main() {
  let cmds = advent_2021::stdin_tok_ws().unwrap();
  let mut x = 0i64;
  let mut y = 0i64;
  let mut aim = 0i64;

  for cmd in cmds {
    let dir = cmd[0].as_str();
    let val: i64 = cmd[1].parse().unwrap();

    match dir {
      "forward" => {
        x += val;
        y += aim * val;
      }
      "up" => aim -= val,
      "down" => aim += val,
      _ => eprintln!("Unrecognized command: {} {}", dir, val)
    }
  }

  println!("X: {} Y: {} X*Y: {}", x, y, x * y);
}