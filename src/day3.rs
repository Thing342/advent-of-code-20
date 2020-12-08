use std::fs::File;
use std::io::{BufReader, BufRead};


fn input() -> Vec<String> {
    let file = File::open("inputs/day3.txt").unwrap();
    let rdr = BufReader::new(file);

    rdr.lines()
        .filter_map(|f| f.ok())
        .collect()
}

fn check_slope(input: &Vec<String>, slope_x: usize, slope_y: usize) -> usize {
    let mut trees_seen = 0;
    let ymax = input.len() - 1;

    let mut x = 0;
    let mut y = 0;

    while y <= ymax {
        let line = input.get(y).unwrap();

        let sled_x = x % line.len();
        let at_pos = line.chars().nth(sled_x).unwrap();

        //eprintln!("{}\t{}->{}", &line, sled_x, at_pos);

        if at_pos == '#' {
            trees_seen += 1;
        }

        x += slope_x;
        y += slope_y;
    }

    //eprintln!("({}, {}) -> {}", slope_x, slope_y, trees_seen);
    trees_seen
}

fn part1() -> usize {
    let input = input();
    check_slope(&input, 3, 1)
}

fn part2() -> usize {
    let input = input();
    check_slope(&input, 1, 1) * check_slope(&input, 3, 1) * check_slope(&input, 5, 1) * check_slope(&input, 7, 1) * check_slope(&input, 1, 2)
}

pub fn main() {
    let p1 = part1();
    println!("DAY 3, PART 1: {}", p1);
    let p2 = part2();
    println!("DAY 3, PART 2: {}", p2);
}