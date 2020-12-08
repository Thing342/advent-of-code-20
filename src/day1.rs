use std::fs::File;
use std::io::{BufReader, BufRead};

const TARGET: i64 = 2020;

fn input() -> Vec<i64> {
    let file = File::open("inputs/day1.txt").unwrap();
    let rdr = BufReader::new(file);

    rdr.lines()
        .map(|l| l.unwrap().parse::<i64>().unwrap())
        .collect()
}

fn find_addends(sorted_nums: &Vec<i64>, target: i64) -> Option<(i64, i64)> {
    for (lo_idx, lo) in sorted_nums.iter().enumerate() {
        for hi_idx in (lo_idx+1 ..= sorted_nums.len()-1).rev() {
            let hi = sorted_nums[hi_idx];
            //eprintln!("{}, {}, {}, {}", lo, hi, lo+hi, lo*hi);
            if lo + hi == target {
                return Some((*lo, hi))
            }
        }
    }

    None
}

fn part1() -> i64 {
    let mut input = input();
    input.sort();

    let (lo, hi) = find_addends(&input, TARGET).unwrap();
    //eprintln!("{} x {} = {}", lo, hi, lo * hi);

    lo * hi
}

fn part2() -> i64 {
    let mut input = input();
    input.sort();

    for it in &input {
        let delta = TARGET - it;
        if let Some((lo, hi)) = find_addends(&input, delta) {
            //eprintln!("{}, {}, {}", lo, it, hi);
            return lo * it * hi
        }
    }

    panic!("Answer not found")
}

pub fn main() {
    let p1 = part1();
    println!("DAY 1, PART 1: {}", p1);
    let p2 = part2();
    println!("DAY 1, PART 2: {}", p2);
}