use std::env;

const FISH_SPAWN: usize = 0;
const FISH_AFTER_SPAWN: usize = 6;
const FISH_NEW: usize = 8;
const FISH_LIFE_MAX: usize = FISH_NEW + 1;

type Aquarium = [u64; FISH_LIFE_MAX];

fn main() {
  let days: usize = env::args().skip(1).next().unwrap().parse().unwrap();
  let fish_starting = advent_2021::stdin().unwrap()
    .into_iter()
    .map(|l| l.split(',').into_iter()
      .map(|n| n.parse().unwrap())
      .collect::<Vec<usize>>())
    .flatten()
    .collect::<Vec<usize>>();

  let mut fish_live: Aquarium = [0u64; FISH_LIFE_MAX];
  for fish in &fish_starting {
    fish_live[*fish] += 1;
  }

  for day in 0..days {
    aquarium_tick(&mut fish_live);
  }

  let fish = aquarium_total(&fish_live);
  println!("Fish: {}", fish);
}

fn aquarium_tick(aq: &mut Aquarium) {
  let spawned = aq[FISH_SPAWN];
  aq[FISH_SPAWN] = 0;

  for i in 1..FISH_LIFE_MAX {
    aq[i - 1] += aq[i];
    aq[i] = 0;
  }

  aq[FISH_AFTER_SPAWN] += spawned;
  aq[FISH_NEW] = spawned;
}

fn aquarium_total(aq: &Aquarium) -> u64 {
  aq.iter().sum()
}
