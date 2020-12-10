
use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn main() { 
  let file = File::open("inputs/day10.txt").expect("Failed to open file");
  let rdr = BufReader::new(file);
  let mut input = rdr.lines()
    .filter_map(|f| f.ok())
    .map(|s| s.parse().unwrap())
    .collect::<Vec<u64>>();

  input.sort();

  let mut deltas = [0, 0, 1];

  let delta = input[0] as usize;
  deltas[delta - 1] += 1;

  for i in 1..input.len() {
    let n0 = input[i-1];
    let n1 = input[i];

    let delta = (n1 - n0) as usize;
    deltas[delta - 1] += 1;
  }

  eprintln!("{:?}", deltas);

  println!("DAY 10, PART 1: {}", deltas[0] * deltas[2]);
}
