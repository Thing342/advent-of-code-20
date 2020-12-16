use std::collections::HashMap;

type Int = i64;
const INPUT: [Int; 6] = [12,1,16,3,11,0];
//const INPUT: [Int; 3] = [0,3,6];

pub fn main() {
    const ROUNDS1: usize = 2020;
    const ROUNDS2: usize = 30000000;
    let mut state = HashMap::new();
    let mut last = -1;
    for round in 1..=ROUNDS2 {
        let it = if round <= INPUT.len() {
            INPUT[round - 1]
        } else {
            if let Some(prev_round) = state.get(&last) {
                (round - 1 - prev_round) as i64
            } else {
                0
            }
        };

        if round == ROUNDS1 {
            println!("DAY 15, PART 1: {}", it);
        }
        //eprintln!("{}) {} {} {:?}", round, it, last, state);
        state.insert(last, round - 1);
        last = it;
    }

    println!("DAY 15, PART 2: {}", last);
}