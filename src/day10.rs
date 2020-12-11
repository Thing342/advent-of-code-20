
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
  input.insert(0, 0);

  //eprintln!("{:?}", &input);

  let mut deltas = [0, 0, 1];

  for i in 1..input.len() {
    let n0 = input[i-1];
    let n1 = input[i];

    let delta = (n1 - n0) as usize;
    deltas[delta - 1] += 1;
  }

  //eprintln!("{:?}", deltas);

  println!("DAY 10, PART 1: {}", deltas[0] * deltas[2]);

  let p2: u64 =  input.windows(2)
    .collect::<Vec<_>>()
    .split(|a| a[1] - a[0] == 3) 
    .map(|x| match x.len() {  
        4 => 7,  
        3 => 4,  
        2 => 2,  
        _ => 1  
    })  
    .product();

  println!("DAY 10, PART 2: {}", p2);
}
