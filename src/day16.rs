use std::fs::File;
use std::io::{BufReader, BufRead};
use std::ops::{Range, RangeInclusive};
use std::collections::{HashSet, HashMap};

use regex::Regex;

const FILE: &str = "inputs/day16.txt";

type Int = i64;

#[derive(Debug)]
struct Rule {
    field: String,
    range_lo: RangeInclusive<Int>,
    range_hi: RangeInclusive<Int>,
}

impl Rule {
    fn is_valid(&self, v: &Int) -> bool {
        self.range_lo.contains(v) || self.range_hi.contains(v)
    }
}

#[derive(Debug)]
struct Ticket(Vec<Int>);

pub fn main() {
    let rules_re = Regex::new("([A-z ]+): ([0-9]+)-([0-9]+) or ([0-9]+)-([0-9]+)").unwrap();

    let file = std::fs::read_to_string(FILE).unwrap();
    let mut sections = file.split("\n\n");

    let rules = sections.next().map(|s|
        rules_re.captures_iter(s).filter_map(|c| Some(Rule {
            field: c.get(1)?.as_str().to_string(),
            range_lo: (c.get(2)?.as_str().parse().unwrap()..=c.get(3)?.as_str().parse().unwrap()),
            range_hi: (c.get(4)?.as_str().parse().unwrap()..=c.get(5)?.as_str().parse().unwrap()),
        })).collect::<Vec<_>>()
    ).unwrap();

    let your_ticket = sections.next().map(|s|
        s.split("\n").skip(1).map(|s2|
            Ticket(s2.split(",")
                .map(|s3| s3.parse().unwrap())
                .collect())
        ).next().unwrap()
    ).unwrap();

    let mut other_tickets = sections.next().map(|s|
        s.split("\n").skip(1).map(|s2|
            Ticket(s2.split(",")
                .map(|s3| s3.parse().unwrap())
                .collect())
        ).collect::<Vec<_>>()
    ).unwrap();

    //eprintln!("{:#?}", rules);
    //eprintln!("{:#?}", your_ticket);
    //eprintln!("{:#?}", other_tickets);

    let mut p1: (Int, Vec<&Ticket>) = other_tickets.iter()
        .fold((0, vec![]), |(mut sum, mut valid_tix), t| {
            let p1 = t.0.iter().map(|v|
                rules.iter()
                    .find(|r| r.is_valid(v))
                    .map(|_| 0)
                    .unwrap_or(*v)
            ).sum::<Int>();
            if p1 == 0 {
                valid_tix.push(t);
            } else {
                sum += p1
            }
            (sum, valid_tix)
        },
        );

    println!("DAY 16, PART 1: {}", p1.0);

    let mut valid_tix = p1.1;
    //eprintln!("{:?}", your_ticket);
    valid_tix.push(&your_ticket);

    let mut gs = (0..rules.len()).map(|i| {
        (0..rules.len()).filter(|j|
            valid_tix.iter().all(|t| rules[i].is_valid(&t.0[*j]))
        ).collect::<HashSet<_>>()
    }).collect::<Vec<_>>();

    //for (i, k) in gs.iter().enumerate() {
    //    eprintln!("{} {:?}", i, k)
    //}

    let mut mapping = HashMap::new();
    let mut remaining_rules = rules.iter().map(|r| r.field.as_str()).collect::<HashSet<_>>();
    let mut remaining_cols = (0..rules.len()).collect::<HashSet<_>>();

    while let Some(rule_id) = gs.iter().position(|g| g.len() == 1) {
        let rule = rules[rule_id].field.clone();
        let column = gs[rule_id].iter().next().unwrap().clone();
        //eprintln!("assign col {} == {} ({})", column, &rule, rule_id);
        mapping.insert(rule.clone(), column);
        remaining_rules.remove(rule.as_str());
        remaining_cols.remove(&column);
        for g in &mut gs {
            g.remove(&column);
            //eprintln!("{:?}", g);
        }

        //for (i, k) in gs.iter().enumerate() {
        //    eprintln!("{} {:?}", i, k)
        //}
    }

    // bug i am too lazy to properly fix, so here's this hack
    mapping.insert(remaining_rules.drain()
                       .next()
                       .unwrap().to_string()
                   , remaining_cols.drain().next().unwrap());

    //eprintln!("{:#?}", mapping);

    let p2 = your_ticket.0[mapping["departure location"] as usize] *
             your_ticket.0[mapping["departure station"] as usize] *
             your_ticket.0[mapping["departure platform"] as usize] *
             your_ticket.0[mapping["departure track"] as usize] *
             your_ticket.0[mapping["departure date"] as usize] *
             //your_ticket.0[16] *
             your_ticket.0[mapping["departure time"] as usize];

    println!("DAY 16, PART 2: {}", p2)
}