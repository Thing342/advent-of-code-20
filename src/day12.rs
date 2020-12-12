use std::fs::File;
use std::io::{BufReader, BufRead};

const FILE: &str = "inputs/day12.txt";

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Heading {
    North,
    South,
    East,
    West,
    Forward,
    Left,
    Right,
}

#[derive(Copy, Clone, Debug)]
struct Direction {
    heading: Heading,
    distance: i32,
}

#[derive(Copy, Clone, Debug)]
struct BoatState {
    heading: Heading,
    lat: i32,
    lon: i32,
}

#[derive(Copy, Clone, Debug)]
struct P2BoatState {
    lat: i32,
    lon: i32,
    vec_lat: i32,
    vec_lon: i32,
}

pub fn main() {
    use Heading::*;

    let file = File::open(FILE).expect("Failed to open file");
    let rdr = BufReader::new(file);
    let dirs = rdr.lines()
        .map(|l| {
            let l = l.unwrap();
            Direction {
                heading: match l.as_bytes()[0] {
                    b'N' => North,
                    b'S' => South,
                    b'E' => East,
                    b'W' => West,
                    b'L' => Left,
                    b'R' => Right,
                    b'F' => Forward,
                    _ => panic!()
                },
                distance: (&l[1..]).parse().unwrap(),
            }
        }).collect::<Vec<_>>();

    let p1 = dirs.iter()
        .fold(BoatState { heading: Heading::East, lat: 0, lon: 0 }, |mut boat, dir| {
            match (boat.heading, dir.heading, dir.distance) {
                (_, North, d) | (North, Forward, d) => boat.lat += d,
                (_, South, d) | (South, Forward, d) => boat.lat -= d,
                (_, East, d) | (East, Forward, d) => boat.lon += d,
                (_, West, d) | (West, Forward, d) => boat.lon -= d,
                (North, Right, 90) | (South, Left, 90) | (South, Right, 270) | (North, Left, 270) | (West, _, 180) => boat.heading = East,
                (South, Right, 90) | (North, Left, 90) | (North, Right, 270) | (South, Left, 270) | (East, _, 180) => boat.heading = West,
                (East, Right, 90) | (West, Left, 90) | (West, Right, 270) | (East, Left, 270) | (North, _, 180) => boat.heading = South,
                (West, Right, 90) | (East, Left, 90) | (East, Right, 270) | (West, Left, 270) | (South, _, 180) => boat.heading = North,
                (_, _, _) => panic!("invalid direction")
            }
            boat
        });

    println!("DAY 12, PART 1: {}", p1.lat.abs() + p1.lon.abs());

    let p2 = dirs.iter()
        .fold(P2BoatState { lat: 0, lon: 0, vec_lat: 1, vec_lon: 10 }, |mut boat, dir| {
            match (dir.heading, dir.distance) {
                (North, d) => boat.vec_lat += d,
                (South, d) => boat.vec_lat -= d,
                (East, d) => boat.vec_lon += d,
                (West, d) => boat.vec_lon -= d,
                (Forward, d) => {
                    boat.lat += d * boat.vec_lat;
                    boat.lon += d * boat.vec_lon;
                }
                (Right, 90) | (Left, 270) => {
                    let (lat, lon) = (-boat.vec_lon, boat.vec_lat);
                    boat.vec_lat = lat;
                    boat.vec_lon = lon;
                },
                (Left, 90) | (Right, 270) => {
                    let (lat, lon) = (boat.vec_lon, -boat.vec_lat);
                    boat.vec_lat = lat;
                    boat.vec_lon = lon;
                },
                (Left, 180) | (Right, 180) => {
                    let (lat, lon) = (-boat.vec_lat, -boat.vec_lon);
                    boat.vec_lat = lat;
                    boat.vec_lon = lon;
                },
                (_, _) => panic!("invalid direction!")
            }
            boat
        });

    println!("DAY 12, PART 2: {}", p2.lat.abs() + p2.lon.abs());
}