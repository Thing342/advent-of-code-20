use std::collections::{VecDeque, HashMap};
use itertools::Itertools;

const INPUT: &str = "253149867";
const ROUNDS: usize = 100;
//const INPUT: &str = "389125467";
//const ROUNDS: usize = 10;

#[derive(Debug)]
struct Cup(usize);

fn eval(rounds: usize, max_cup: usize, min_cup: usize, first_cup: usize, cups: &mut HashMap<usize, usize>) {
    let mut current_cup = first_cup;
    for round in 0..rounds {
        //eprintln!("--- move {} ---", round+1);
        //eprintln!("current cup: {:?}", &current_cup);

        let moved_cup_1 = cups[&current_cup];
        let moved_cup_2 = cups[&moved_cup_1];
        let moved_cup_3 = cups[&moved_cup_2];

        cups.insert(current_cup, cups[&moved_cup_3]);

        //eprintln!("pick up: {:?}", (moved_cup_1, moved_cup_2, moved_cup_3));

        let destination_cup = (min_cup..current_cup).rev()
            .find(|v|
                *v != moved_cup_1 && *v != moved_cup_2 && *v != moved_cup_3
            ).or_else(||
            (min_cup..=max_cup).rev().find(|v|
                *v != moved_cup_1 && *v != moved_cup_2 && *v != moved_cup_3
            )
        ).unwrap();

        //eprintln!("destination: {}", destination_cup);
        let destination_cup_next = cups[&destination_cup];

        cups.insert(destination_cup, moved_cup_1);
        cups.insert(moved_cup_3, destination_cup_next);

        current_cup = cups[&current_cup];
    }
}

pub fn main() {
    let input = INPUT.chars()
        .map(|c| (c as u8 - '0' as u8) as usize).collect::<Vec<usize>>();

    let max_cup = *input.iter().max().unwrap();
    let min_cup = *input.iter().min().unwrap();

    let mut cups = HashMap::with_capacity(input.len());
    for (n1,n2) in input.iter().tuple_windows::<(_,_)>() {
        cups.insert(*n1, *n2);
    }

    let first_cup = *input.first().unwrap();
    cups.insert(*input.last().unwrap(), first_cup);

    eval(100, max_cup, min_cup, first_cup, &mut cups);

    let mut p1 = String::new();
    let mut cup = cups[&1];
    while cup != 1 {
        p1.push((cup as u8 + '0' as u8) as char);
        cup = cups[&cup];
    }

    println!("DAY 23, PART 1: {}", p1);

    let mut cups = {
        let mut cups2 = HashMap::with_capacity(1_000_000);
        for (n1,n2) in input.iter().tuple_windows::<(_,_)>() {
            cups2.insert(*n1, *n2);
        }

        cups2.insert(*input.last().unwrap(), max_cup+1);

        for n in (max_cup+1)..=1_000_000 {
            cups2.insert(n,n+1);
        }

        cups2.insert(1_000_000, first_cup);

        cups2
    };

    eval(10_000_000, 1_000_000, min_cup, first_cup, &mut cups);

    let p2_1 = cups[&1];
    let p2_2 = cups[&p2_1];

    println!("DAY 23, PART 2: {}", p2_1 * p2_2);
}