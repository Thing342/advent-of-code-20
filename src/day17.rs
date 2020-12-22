use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

use itertools::{iproduct, Itertools};

const FILE: &str = "inputs/day17.txt";

type Int = i64;
type Cube = (Int, Int, Int);
type Hypercube = (Int, Int, Int, Int);

fn count_active_neighbors_3d(i: Int, j: Int, k: Int, active: &HashSet<Cube>) -> usize {
    iproduct!((i-1)..=(i+1), (j-1)..=(j+1), (k-1)..=(k+1))
        .filter(|cube2| {
            //eprintln!("({},{},{}) -> {:?}", i,j,k, cube2);
            cube2 != &(i,j,k) && active.contains(cube2)
        })
        .count()
}

fn count_active_neighbors_4d(hcube: &Hypercube, active: &HashSet<Hypercube>) -> usize {
    iproduct!(
        (hcube.0-1)..=(hcube.0+1),
        (hcube.1-1)..=(hcube.1+1), 
        (hcube.2-1)..=(hcube.2+1),
        (hcube.3-1)..=(hcube.3+1)
    )
        .filter(|hcube2| {
            //eprintln!("({},{},{}) -> {:?}", i,j,k, cube2);
            *hcube2 != *hcube && active.contains(hcube2)
        })
        .count()
}

pub fn main() {
    let file = File::open(FILE).expect("Failed to open file");
    let rdr = BufReader::new(file);
    
    let mut active = HashSet::new();
    for (i, r) in rdr.lines().enumerate() {
        for (j, c) in r.unwrap().chars().enumerate() {
            if c == '#' {
                active.insert((i as Int, j as Int, 0 as Int));
            }
        }
    }

    let mut active_4d: HashSet<Hypercube> = active.iter().map(|cube| (cube.0, cube.1, cube.2, 0)).collect();

    //eprintln!("{:?} \n", active);

    let mut next_gen: HashSet<Cube>;
    for generation in 0..6 {
        let span_i = active.iter().map(|(i,_,_)| *i).minmax().into_option().unwrap();
        let span_j = active.iter().map(|(_,j,_)| *j).minmax().into_option().unwrap();
        let span_k = active.iter().map(|(_,_,k)| *k).minmax().into_option().unwrap();


        next_gen = active.clone();
        next_gen.retain(|(i,j,k)| {
            let n = count_active_neighbors_3d(*i, *j, *k, &active);
            //eprintln!("Active cube ({},{},{}) has {} active neighbors.", i, j, k, n);
            n == 2 || n == 3
        });
        
        next_gen.extend(
            iproduct!((span_i.0 - 1)..=(span_i.1 + 1), (span_j.0 -1)..=(span_j.1 + 1), (span_k.0 - 1)..=(span_k.1 + 1))
                .filter(|cube| !active.contains(cube))
                .filter(|(i,j,k)| {
                    let n = count_active_neighbors_3d(*i, *j, *k, &active);
                    //eprintln!("Inactive cube ({},{},{}) has {} active neighbors.", i, j, k, n);
                    n == 3
                }));

        active = next_gen;
    }

    println!("DAY 17, PART 1: {}", active.len());

    let mut next_gen_4d: HashSet<Hypercube>;
    for generation in 0..6 {
        let span_i = active_4d.iter().map(|(i,_,_,_)| *i).minmax().into_option().unwrap();
        let span_j = active_4d.iter().map(|(_,j,_,_)| *j).minmax().into_option().unwrap();
        let span_k = active_4d.iter().map(|(_,_,k,_)| *k).minmax().into_option().unwrap();
        let span_h = active_4d.iter().map(|(_,_,_,h)| *h).minmax().into_option().unwrap();


        next_gen_4d = active_4d.clone();
        next_gen_4d.retain(|hcube| {
            let n = count_active_neighbors_4d(hcube, &active_4d);
            //eprintln!("Active cube ({},{},{}) has {} active neighbors.", i, j, k, n);
            n == 2 || n == 3
        });
        
        next_gen_4d.extend(
            iproduct!((span_i.0 - 1)..=(span_i.1 + 1), (span_j.0 -1)..=(span_j.1 + 1), (span_k.0 - 1)..=(span_k.1 + 1), (span_h.0 - 1)..=(span_h.1 + 1))
                .filter(|hcube| !active_4d.contains(hcube))
                .filter(|hcube| {
                    let n = count_active_neighbors_4d(hcube, &active_4d);
                    //eprintln!("Inactive cube ({},{},{}) has {} active neighbors.", i, j, k, n);
                    n == 3
                }));

                active_4d = next_gen_4d;
    }


    println!("DAY 17, PART 2: {}", active_4d.len());

}