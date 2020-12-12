use std::fs::File;
use std::io::{BufReader, BufRead};

fn findparents(x: u64, ns: &[u64]) -> Option<(u64, u64)> {
    for i in 0..ns.len() {
        for j in 0..ns.len() {
            let ni = ns[i];
            let nj = ns[j];

            if ni == nj { continue }
            if ni + nj == x {
                return Some((ni, nj))
            }
        }
    }

    None
}

pub fn main() {
    const PREAMBLE_SIZE: usize = 25;

    let file = File::open("inputs/day9.txt").expect("Failed to open file");
    let rdr = BufReader::new(file);
    let input = rdr.lines()
        .filter_map(|f| f.ok())
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u64>>();

    let mut weakness = (0, 0);

    for i in 0..input.len() {
        let idx = i + PREAMBLE_SIZE;
        let preamble = &input[i..idx];

        let x = input[idx];
        if findparents(x, preamble).is_none() {
            weakness = (idx, x);
            break;
        }
    }

    println!("DAY 9, PART 1: {}", weakness.1);

    for i in 0..weakness.0 {
        let mut acc = 0;
        let mut j = i;
        while acc < weakness.1 {
            acc += input[j];
            j += 1;
        }

        if acc == weakness.1 {
            //let bounds = (input[i], input[j]);
            let slice = &input[i..j];
            //eprintln!("{} = {:?}", slice.iter().sum::<u64>(), slice);

            let lo = slice.iter().min().unwrap();
            let hi = slice.iter().max().unwrap();

            //eprintln!("{} + {}", lo, hi);
            println!("DAY 9, PART 2: {}", lo + hi);
            return;
        }
    }

    panic!();
}