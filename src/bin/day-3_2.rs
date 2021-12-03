fn main() {
  let line_bits = advent_2021::stdin().unwrap()
    .into_iter()
    .map(|l| l.chars()
      .map(|c| c.to_digit(2).unwrap() as u8)
      .collect::<Vec<u8>>())
    .collect::<Vec<Vec<u8>>>();

  let o2 = filter(line_bits.clone(),
                  |a, b| if a >= b { 1u8 } else { 0u8 }).unwrap();
  let co2 = filter(line_bits.clone(),
                   |a, b| if a >= b { 0u8 } else { 1u8 }).unwrap();

  println!("O2: {} CO2: {} O2*CO2: {}", o2, co2, o2 * co2);
}

fn bit_count(v: &Vec<Vec<u8>>) -> Vec<u32> {
  let x_sz = v[0].len();
  let mut accum: Vec<u32> = vec![0; x_sz];
  for row in v {
    for x in 0..x_sz {
      accum[x] = accum[x] + row[x] as u32;
    }
  }
  accum
}

fn filter<F>(mut v: Vec<Vec<u8>>, f: F) -> Option<u64>
  where F: Fn(u32, u32) -> u8 {
  let x_sz = v[0].len();
  let mut x = 0;
  let mut result = None;
  loop {
    if x >= x_sz {
      break;
    }

    let accum = bit_count(&v);
    let y_sz = v.len() as u32;
    let b = f(accum[x], y_sz - accum[x]);

    v = v.into_iter()
      .filter(|l| l[x] == b)
      .collect::<Vec<Vec<u8>>>();
    if v.len() == 1 {
      result = Some(v.pop().unwrap().clone());
      break;
    }

    x += 1;
  }

  match result {
    Some(v) => Some(advent_2021::bincharvec_to_unsigned(v)),
    None => None
  }
}