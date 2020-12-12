use std::fs::File;
use std::io::{BufReader, Read};
use SeatState::*;
use std::cmp::min;

const LINE_LEN: usize = 95;
const FILE: &str = "inputs/day11.txt";

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum SeatState {
    EMPTY,
    FILLED,
    FLOOR,
}

fn p1() {
    let file = File::open(FILE).expect("Failed to open file");
    let rdr = BufReader::new(file);
    let mut arena = rdr.bytes()
        .filter_map(|b| b.ok())
        .filter_map(|b| match b {
            b'L' => Some(EMPTY),
            b'.' => Some(FLOOR),
            b'#' => Some(FILLED),
            _ => None
        }).collect::<Vec<_>>();

    let arena_size = arena.len();
    let mut minesweeper_arena = vec![0; arena_size];

    let mut filled = 0;
    for round in 1..100 {
        let mut n_filled = 0;
        let mut n_empty = 0;
        for i in 0..arena_size {
            let count = &mut minesweeper_arena[i];
            let top_edge = i < LINE_LEN;
            let bottom_edge = i >= arena_size - LINE_LEN;
            let left_edge = i % LINE_LEN == 0;
            let right_edge = (i+1) % LINE_LEN == 0;

            if !top_edge    && !left_edge  && arena[i - LINE_LEN - 1] == FILLED { *count += 1 }
            if !top_edge                   && arena[i - LINE_LEN    ] == FILLED { *count += 1 }
            if !top_edge    && !right_edge && arena[i - LINE_LEN + 1] == FILLED { *count += 1 }
            if                 !left_edge  && arena[i            - 1] == FILLED { *count += 1 }
            if                 !right_edge && arena[i            + 1] == FILLED { *count += 1 }
            if !bottom_edge && !left_edge  && arena[i + LINE_LEN - 1] == FILLED { *count += 1 }
            if !bottom_edge                && arena[i + LINE_LEN    ] == FILLED { *count += 1 }
            if !bottom_edge && !right_edge && arena[i + LINE_LEN + 1] == FILLED { *count += 1 }

            //eprint!("{} ", *count);
            //if (i+1)%LINE_LEN == 0 { eprintln!(); }
        }

        for i in 0..arena_size {
            arena[i] = match (arena[i], minesweeper_arena[i]) {
                (SeatState::FILLED, surr) if surr >= 4 => EMPTY,
                (SeatState::EMPTY, surr) if surr == 0 => FILLED,
                (st, _) => st
            };

            minesweeper_arena[i] = 0;

            match arena[i] {
                FILLED => {
                    //eprint!("# ");
                    n_filled +=1;
                },
                EMPTY => {
                    //eprint!("L ");
                    n_empty += 1
                },
                FLOOR => {
                    //eprint!(". ");
                }
            }

            //if (i+1)%LINE_LEN == 0 { eprintln!() }
        }

        //eprintln!("{} | {} filled, {} empty", round, n_filled, n_empty);
        if filled == n_filled { break; }
        else { filled = n_filled; }
    }

    println!("DAY 11, PART 1: {}", filled);
}

fn p2() {
    let file = File::open(FILE).expect("Failed to open file");
    let rdr = BufReader::new(file);
    let mut arena = rdr.bytes()
        .filter_map(|b| b.ok())
        .filter_map(|b| match b {
            b'L' => Some(EMPTY),
            b'.' => Some(FLOOR),
            b'#' => Some(FILLED),
            _ => None
        }).collect::<Vec<_>>();

    let arena_size = arena.len();
    let mut minesweeper_arena = vec![0; arena_size];

    let mut filled = 0;
    for round in 1..100 {
        let mut n_filled = 0;
        let mut n_empty = 0;
        for i in 0..arena_size {
            //eprintln!("{}", i);
            let n_range = i / LINE_LEN;
            let w_range = i % LINE_LEN;
            let s_range = (arena_size / LINE_LEN) - n_range - 1;
            let e_range = LINE_LEN - w_range - 1;
            //eprintln!("N{}, W{}, E{}, S{}", n_range, w_range, e_range, s_range);
            //look northwest
            // i, i-L-1, i-2L-2, i-3L-3...
            let nw = (1..).map(|n| i - n*LINE_LEN - n)
                .take(min(n_range, w_range))
                .find_map(|j| match arena[j] { EMPTY => Some(0), FILLED => Some(1), FLOOR => None })
                .unwrap_or(0);

            //look north
            // i, i-L, i-2L, i-3L...
            let n = (1..).map(|n| i - n*LINE_LEN)
                .take(n_range)
                .find_map(|j| match arena[j] { EMPTY => Some(0), FILLED => Some(1), FLOOR => None })
                .unwrap_or(0);
            //look northeast
            // i, i-L+1, i-2L+2, i-3L+3...
            let ne = (1..).map(|n| i - n*LINE_LEN + n)
                .take(min(n_range, e_range))
                .find_map(|j| match arena[j] { EMPTY => Some(0), FILLED => Some(1), FLOOR => None })
                .unwrap_or(0);
            //look west
            // i, i-1, i-2, i-3...
            let w = (1..).map(|n| i - n)
                .take(w_range)
                .find_map(|j| match arena[j] { EMPTY => Some(0), FILLED => Some(1), FLOOR => None })
                .unwrap_or(0);
            //look east
            // i, i+1, i+2, i+3...
            let e = (1..).map(|n| i + n)
                .take(e_range)
                .find_map(|j| match arena[j] { EMPTY => Some(0), FILLED => Some(1), FLOOR => None })
                .unwrap_or(0);
            //look southwest
            // i, i+L-1, i+2L-2, i-3L-3...
            let sw = (1..).map(|n| i + n*LINE_LEN - n)
                .take(min(s_range, w_range))
                .find_map(|j| match arena[j] { EMPTY => Some(0), FILLED => Some(1), FLOOR => None })
                .unwrap_or(0);
            //look south
            // i, i+L, i+2L, i+3L...
            let s = (1..).map(|n| i + n*LINE_LEN)
                .take(s_range)
                .find_map(|j| match arena[j] { EMPTY => Some(0), FILLED => Some(1), FLOOR => None })
                .unwrap_or(0);
            //look southeast
            // i, i+L+1, i+2L+2, i+3L+3...
            let se = (1..).map(|n| i + n*LINE_LEN + n)
                .take(min(s_range, e_range))
                .find_map(|j| match arena[j] { EMPTY => Some(0), FILLED => Some(1), FLOOR => None })
                .unwrap_or(0);

            minesweeper_arena[i] = ne + n + nw + e + w + se + s + sw;
            //eprint!("{} ", minesweeper_arena[i]);
            //if (i+1)%LINE_LEN == 0 { eprintln!(); }
        }

        for i in 0..arena_size {
            arena[i] = match (arena[i], minesweeper_arena[i]) {
                (SeatState::FILLED, surr) if surr >= 5 => EMPTY,
                (SeatState::EMPTY, surr) if surr == 0 => FILLED,
                (st, _) => st
            };

            minesweeper_arena[i] = 0;

            match arena[i] {
                FILLED => {
                  //  eprint!("# ");
                    n_filled +=1;
                },
                EMPTY => {
                //    eprint!("L ");
                    n_empty += 1
                },
                FLOOR => {
              //      eprint!(". ");
                }
            }

            //if (i+1)%LINE_LEN == 0 { eprintln!() }
        }

        //eprintln!("{} | {} filled, {} empty", round, n_filled, n_empty);
        if filled == n_filled { break; }
        else { filled = n_filled; }
    }

    println!("DAY 11, PART 2: {}", filled);
}

fn main() {
    p1();
    p2();
}