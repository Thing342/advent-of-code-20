use std::ops::{Add, AddAssign, Neg, Mul};
use std::fs::File;
use std::io::{BufReader, BufRead};
use itertools::{Itertools,iproduct};
use std::cmp::Ordering;
use std::collections::HashSet;

type Int = i32;

const FILE: &str = "inputs/day24.txt";

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Hex {
    q: Int,
    r: Int
}

impl Hex {

}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Northeast, East, Southeast, Southwest, West, Northwest
}

pub fn main() {
    let file = File::open(FILE).expect("Failed to open file");
    let rdr = BufReader::new(file);
    let directions = rdr.lines()
        .filter_map(|r| {
            let line = r.ok()?;
            let mut chars = line.chars();
            let mut dirs = vec![];

            while let Some(c) = chars.next() {
                match c {
                    'n' => match chars.next().unwrap() {
                        'e' => dirs.push(Direction::Northeast),
                        'w' => dirs.push(Direction::Northwest),
                        _ => panic!()
                    },
                    's' => match chars.next().unwrap() {
                        'e' => dirs.push(Direction::Southeast),
                        'w' => dirs.push(Direction::Southwest),
                        _ => panic!()
                    },
                    'e' => dirs.push(Direction::East),
                    'w' => dirs.push(Direction::West),
                    _ => panic!()
                }
            }

            Some(dirs)
        }).collect::<Vec<_>>();

    let mut black_tiles = HashSet::new();
    for dir_set in directions {
        let hex = dir_set.iter()
            .fold(Hex {r: 0, q: 0}, |mut hex, dir| {
                match dir {
                    Direction::East => hex.q += 1,
                    Direction::West => hex.q -= 1,
                    Direction::Northwest => hex.r -= 1,
                    Direction::Southeast => hex.r += 1,
                    Direction::Northeast => {
                        hex.q += 1;
                        hex.r -= 1;
                    },
                    Direction::Southwest => {
                        hex.q -= 1;
                        hex.r += 1;
                    }
                }
                hex
            });

        if !black_tiles.insert(hex.clone()) {
            black_tiles.remove(&hex);
        } else {
            // no-op
        }
    }

    println!("DAY 24, PART 1: {}", black_tiles.len());

    const NEIGHBORS: [(Int,Int); 6] = [(1,0), (-1, 0), (0, 1), (0, -1), (1, -1), (-1, 1)];

    let mut next_gen: HashSet<Hex>;
    for generation in 0..100 {
        let span_q = black_tiles.iter().map(|hex| hex.q).minmax().into_option().unwrap();
        let span_r = black_tiles.iter().map(|hex| hex.r).minmax().into_option().unwrap();

        next_gen = black_tiles.clone();
        next_gen.retain(|hex| {
            let count = NEIGHBORS.iter()
                .map(|(q,r)| Hex { q: hex.q + q, r: hex.r + r })
                .filter(|hex| black_tiles.contains(hex))
                .count();
            0 < count && count < 2
        });

        next_gen.extend(
            iproduct!((span_q.0 - 1)..=(span_q.1 + 1), (span_r.0 - 1)..=(span_r.1 + 1))
                .map(|(q,r)| Hex { q, r })
                .filter(|hex| {
                    let count = NEIGHBORS.iter()
                        .map(|(q,r)| Hex { q: hex.q + q, r: hex.r + r })
                        .filter(|hex| black_tiles.contains(hex))
                        .count();
                    count == 2
                }));

        black_tiles = next_gen;
        //eprintln!("Day {}: {}", generation + 1, black_tiles.len())
    }

    println!("DAY 24, PART 2: {}", black_tiles.len());
}