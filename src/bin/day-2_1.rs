fn main() {
  let cmds = advent_2021::stdin_tok_ws().unwrap();
  let mut x = 0;
  let mut y = 0;

  for cmd in cmds {
    let dir = cmd[0].as_str();
    let val: i32 = cmd[1].parse().unwrap();

    match dir {
      "forward" => x += val,
      "up" => y -= val,
      "down" => y += val,
      _ => eprintln!("Unrecognized command: {} {}", dir, val)
    }
  }

  println!("X: {} Y: {} X*Y: {}", x, y, x * y);
}