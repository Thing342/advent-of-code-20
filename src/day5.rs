use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let file = File::open("inputs/day5.txt").unwrap();
    let rdr = BufReader::new(file);

    let mut seats = rdr.lines()
        .filter_map(|f| f.ok())
        .map(|s|
            u32::from_str_radix(&s
                .replace("F", "0")
                .replace("B", "1")
                .replace("L", "0")
                .replace("R", "1"), 2)
                .expect("Couldn't parse")
        )
        .collect::<Vec<u32>>();

    seats.sort();
    let maxseat = seats.last().unwrap();
    println!("DAY 5, PART 1: {}", maxseat);

    //eprintln!("{:#?}", seats);
    for (n, seat) in seats.iter().enumerate() {
        if let Some(next) = seats.get(n + 1) {
            //eprintln!("({}, {})", seat, next);
            if *next != seat + 1 {
                let seatid = seat + 1;
                println!("DAY 5, PART 2: {}", seatid);
                return;
            }
        }
    }

    panic!("Seat not found!");
}