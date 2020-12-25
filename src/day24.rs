use std::ops::{Add, AddAssign, Neg, Mul};
use std::fs::File;
use std::io::{BufReader, BufRead};
use itertools::{Itertools,iproduct};
use std::cmp::Ordering;
use std::collections::HashSet;

const FILE: &str = "inputs/day24.txt";

type PsuedoFloat = i64;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Complex {
    real: PsuedoFloat,
    imaginary: PsuedoFloat
}

impl Complex {
    fn zero() -> Self {
        Complex {real: 0_000000, imaginary: 0_000000}
    }

    fn scaled(&self, scalar: i64) -> Complex {
        Complex {
            real: self.real * scalar,
            imaginary: self.imaginary * scalar
        }
    }
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary
        }
    }
}

impl Add for &Complex {
    type Output = Complex;

    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary
        }
    }
}

impl AddAssign for Complex {
    fn add_assign(&mut self, rhs: Self) {
        self.real += rhs.real;
        self.imaginary += rhs.imaginary
    }
}

impl Neg for Complex {
    type Output = Complex;

    fn neg(self) -> Self::Output {
        Complex {
            real: -self.real,
            imaginary: -self.imaginary
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Northeast, East, Southeast, Southwest, West, Northwest
}

const COS_60: PsuedoFloat = 0_866025;

const UNIT_NE: Complex = Complex {
    real: 0_500000,
    imaginary: COS_60
};

const UNIT_E: Complex = Complex {
    real: 1_000000,
    imaginary: 0_000000
};

const UNIT_SE: Complex = Complex {
    real: 0_500000,
    imaginary: -COS_60
};

pub fn main() {
    let DIRECTIONS = [UNIT_NE, UNIT_E, UNIT_SE, -UNIT_NE, -UNIT_E, -UNIT_SE];

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

    let mut span = 0;

    for dir_set in directions {
        eprintln!("{:?}", dir_set);
        let (e,ne,se) = dir_set.iter()
            .fold((0,0,0), |pos, dir| {
                match dir {
                    Direction::East => (pos.0 + 1, pos.1, pos.2),
                    Direction::West => (pos.0 - 1, pos.1, pos.2),
                    Direction::Northeast => (pos.0, pos.1 + 1, pos.2),
                    Direction::Southwest => (pos.0, pos.1 - 1, pos.2),
                    Direction::Southeast => (pos.0, pos.1, pos.2 + 1),
                    Direction::Northwest => (pos.0, pos.1, pos.2 - 1)
                }
            });

        let tile = UNIT_E.scaled(e) + UNIT_NE.scaled(ne) + UNIT_SE.scaled(se);

        if !black_tiles.insert(tile.clone()) {
            //eprintln!("Flipping tile ({}+{}i) to white", &tile.real, &tile.imaginary);
            black_tiles.remove(&tile);
        } else {
            //eprintln!("Flipping tile ({}+{}i) to black", &tile.real, &tile.imaginary);
        }

        span = span.max(e.max(ne.max(se)))
    }

    println!("DAY 24, PART 1: {}", black_tiles.len());

    let mut next_gen: HashSet<Complex>;
    for generation in 0..100 {
        next_gen = black_tiles.clone();
        //let span_r = black_tiles.iter().map(|c| c.real).minmax().into_option().unwrap();
        //let span_i = black_tiles.iter().map(|c| c.imaginary).minmax().into_option().unwrap();

        next_gen.retain(|c| {
            let count = DIRECTIONS.iter()
                .filter(|d| black_tiles.contains(&(c+*d)))
                .count();
            0 < count && count < 2
        });

        next_gen.extend(
            iproduct!(-span..=span, -span..=span, -span..=span)
                .map(|(ne, e, se)| UNIT_E.scaled(e) + UNIT_NE.scaled(ne) + UNIT_SE.scaled(se))
                //.filter(|c| span_r.0 <= c.real && c.real <= span_r.1)
                //.filter(|c| span_i.0 <= c.imaginary && c.imaginary <= span_i.1)
                //.filter(|c| !black_tiles.contains(c))
                .filter(|c| {
                    let count = DIRECTIONS.iter()
                        .filter(|d| black_tiles.contains(&(c+*d)))
                        .count();
                    count == 2
                })
        );

        black_tiles = next_gen;
        span += 1;
        eprintln!("Day {}: {}", generation + 1, black_tiles.len())
    }

    println!("DAY 24, PART 2: {}", black_tiles.len());

}