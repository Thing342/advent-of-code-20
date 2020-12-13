use std::fs::File;
use std::io::{BufReader, BufRead};

const FILE: &str = "inputs/day13.txt";

type Int = i64;

pub fn main() {
    let file = File::open(FILE).expect("Failed to open file");
    let rdr = BufReader::new(file);
    let mut lines = rdr.lines();

    let curr_time: Int = lines.next().unwrap().unwrap().parse().unwrap();
    let buses = lines.next().unwrap().unwrap().split(",")
        .map(|s| match s.chars().next().unwrap() {
            'x' => None,
            _ => Some(s.parse::<Int>().unwrap())
        }).collect::<Vec<_>>();

    let bus = buses.iter().filter(|o| o.is_some())
        .map(|o| o.unwrap())
        .max_by_key(|bus_id| curr_time % bus_id)
        .unwrap();

    let wait_time = bus - (curr_time % bus);
    println!("DAY 13, PART 1: {}", wait_time * bus);

    let mut n_v = Vec::new();
    let mut a_v = Vec::new();

    for (i, b) in buses.iter().enumerate() {
        if let Some(bus) = b {
            n_v.push(*bus);
            a_v.push(*bus - (i as Int % *bus));
        }
    }

    let mut x = a_v[0];
    let mut step = n_v[0];

    for i in 1..n_v.len() {
        let target = a_v[i];
        let modder = n_v[i];
        //eprintln!("modder: {}, x: {}, step: {}", modder, x, step);
        x = (0..).map(|j| x + j * step).find(|s| s % modder == target).unwrap();
        step *= modder;
    }

    println!("DAY 13, PART 2: {}", x);

}