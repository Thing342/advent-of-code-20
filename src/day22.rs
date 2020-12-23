use std::collections::{VecDeque, HashSet};
use itertools::Itertools;

const FILE: &str = "inputs/day22.txt";

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
struct Card(usize);

fn hand_score(hand: &VecDeque<Card>) -> usize {
    hand.iter().rev().enumerate()
        .map(|(i, card)| {
            ////eprintln!("+ {} * {}", i + 1, card.0);
            (i + 1) * card.0
        })
        .sum()
}

fn recursive_combat(mut player1: VecDeque<Card>, mut player2: VecDeque<Card>, count: &mut usize) -> (bool, usize) {
    let mut prev_configs = HashSet::new();
    *count += 1;
    let game = *count;
    //eprintln!("--- Starting new game {}...", game);
    let mut round = 0usize;
    while !player1.is_empty() && !player2.is_empty() {
        round += 1;
        //eprintln!("-- Game {} Round {}", game, round);
        let decks = format!("{:?}", (&player1, &player2));
        ////eprintln!("{}", prev_configs.len());
        //eprintln!("{}", decks);
        if prev_configs.contains(&decks) {
            //eprintln!("ending game early");
            return (true, hand_score(&player1));
        } else {
            prev_configs.insert(decks);
        }

        let card1 = player1.pop_front().unwrap();
        let card2 = player2.pop_front().unwrap();

        //eprintln!("Player 1 plays {}\nPlayer 2 plays {}", card1.0, card2.0);

        let winner_p1 = if player1.len() >= card1.0 && player2.len() >= card2.0 {
            let player1: VecDeque<Card> = player1.iter().take(card1.0).cloned().collect();
            let player2: VecDeque<Card> = player2.iter().take(card2.0).cloned().collect();
            if player1.iter().max() > player2.iter().max() {
                true
            } else {
                recursive_combat(player1, player2, count).0
            }
        } else {
            card1 > card2
        };

        if winner_p1 {
            player1.push_back(card1);
            player1.push_back(card2);
        } else {
            player2.push_back(card2);
            player2.push_back(card1);
        }

        //eprintln!("{:?}\n{:?}\n", &player1, &player2);

    }

    let winner = if !player1.is_empty() {
        //eprintln!("--- Player 1 wins game {}", count);
        &player1
    } else {
        //eprintln!("--- Player 2 wins game {}", count);
        &player2
    };
    return (!player1.is_empty(), hand_score(winner));
}

pub fn main() {
    let file = std::fs::read_to_string(FILE).unwrap();

    // the front of the queue is logically the top of the deck
    let (mut player1, mut player2) = file.split("\n\n").map(|block|
        block.split("\n")
            .skip(1)
            .map(|line| Card(line.parse().unwrap()))
            .collect::<VecDeque<_>>()
    ).collect_tuple::<(_, _)>().unwrap();

    //eprintln!("{:?}", (&player1, &player2));

    let (player1p2, player2p2) = (player1.clone(), player2.clone());
    
    while !player1.is_empty() && !player2.is_empty() {
        let card1 = player1.pop_front().unwrap();
        let card2 = player2.pop_front().unwrap();

        //eprintln!("{:?}", (&player1, &player2));

        if card1 > card2 {
            player1.push_back(card1);
            player1.push_back(card2);
        } else {
            player2.push_back(card2);
            player2.push_back(card1);
        }
    }

    ////eprintln!("{:?}", (&player1, &player2));

    let winner = if !player1.is_empty() { &player1 } else { &player2 };
    let p1 = hand_score(winner);

    println!("DAY 22, PART 1: {}", p1);

    let (player_winner, score) = recursive_combat(player1p2, player2p2, &mut 0);

    println!("DAY 22, PART 2: {}", score);
}