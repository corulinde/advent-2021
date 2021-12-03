fn main() {
  let line_bits = advent_2021::stdin().unwrap()
    .into_iter()
    .map(|l| l.chars()
      .map(|c| c.to_digit(2).unwrap() as u8)
      .collect::<Vec<u8>>())
    .collect::<Vec<Vec<u8>>>();
  let half_y = line_bits.len() as u32 / 2;
  let x_sz = line_bits[0].len();

  let mut accum: Vec<u32> = vec![0; x_sz];
  for row in line_bits {
    for x in 0..x_sz {
      accum[x] = accum[x] + row[x] as u32;
    }
  }

  for x in 0..x_sz {
    if accum[x] > half_y {
      accum[x] = 1u32 << (x_sz - x - 1);
    } else {
      accum[x] = 0;
    }
  }

  let gamma: u32 = accum.into_iter().sum();
  let mask = (1u32 << x_sz) - 1;
  let epsilon = mask ^ gamma;

  println!("Gamma: {} Epsilon: {} Gamma*Epsilon: {}", gamma, epsilon, gamma * epsilon);
}