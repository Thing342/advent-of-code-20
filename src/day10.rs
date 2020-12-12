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

    let p1 = input.windows(2)
        .fold([0, 0, 1], |mut acc, w| {
            let n0 = w[0];
            let n1 = w[1];

            let delta = (n1 - n0) as usize;
            acc[delta - 1] += 1;
            acc
        });

    println!("DAY 10, PART 1: {}", p1[0] * p1[2]);

    let p2: u64 = input.windows(2)
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
