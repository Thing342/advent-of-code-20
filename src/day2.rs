use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct PasswordEntry {
    policy_lo: usize,
    policy_hi: usize,
    policy_ch: char,
    password: String,
}

fn input() -> String {
    std::fs::read_to_string("inputs/day2.txt").unwrap()
}

fn count_chars(s: &str) -> HashMap<char, usize> {
    let mut map = HashMap::with_capacity(s.len());
    for c in s.chars() {
        if let Some(cn) = map.get_mut(&c) {
            *cn += 1;
        } else {
            map.insert(c, 1);
        }
    }
    map
}

#[test]
fn test_count_chars() {
    let s = "jzpprcpcmmpb";
    let counts = count_chars(s);
    println!("{:#?}", counts);
}

fn part1() -> usize {
    let input = input();
    let re = Regex::new("([0-9]*)-([0-9]*) (.): (.*)\n").unwrap();

    re.captures_iter(&input)
        .filter_map(|c| Some(PasswordEntry {
            policy_lo: c.get(1)?.as_str().parse().expect("Failed parsing lo"),
            policy_hi: c.get(2)?.as_str().parse().expect("Failed parsing hi"),
            policy_ch: c.get(3)?.as_str().parse().expect("Failed parsing char"),
            password: c.get(4)?.as_str().to_string(),
        }))
        .filter(|pe| {
            let counts = count_chars(&pe.password);
            if let Some(cn) = counts.get(&pe.policy_ch) {
                let res = pe.policy_lo <= *cn && *cn <= pe.policy_hi;
                //eprintln!("{} {} {} {} {} {}", pe.policy_ch, &pe.password, pe.policy_lo, cn, pe.policy_hi, res);
                res
            } else {
                //eprintln!("{} {} {} {} {} {}", pe.policy_ch, &pe.password, pe.policy_lo, 0, pe.policy_hi, false);
                false
            }
        })
        .count() + 1
}

fn part2() -> usize {
    let input = input();
    let re = Regex::new("([0-9]*)-([0-9]*) (.): (.*)\n").unwrap();

    re.captures_iter(&input)
        .filter_map(|c| Some(PasswordEntry {
            policy_lo: c.get(1)?.as_str().parse().expect("Failed parsing lo"),
            policy_hi: c.get(2)?.as_str().parse().expect("Failed parsing hi"),
            policy_ch: c.get(3)?.as_str().parse().expect("Failed parsing char"),
            password: c.get(4)?.as_str().to_string(),
        }))
        .filter(|pe| {
            let mut chs = pe.password.chars();
            let lo = chs.nth(pe.policy_lo - 1);
            let hi = chs.nth(pe.policy_hi - pe.policy_lo - 1);

            //eprintln!("{} {} {}:{:?} {}:{:?}", pe.policy_ch, &pe.password, pe.policy_lo, lo, pe.policy_hi, hi);

            match (hi, lo) {
                (Some(hi), Some(lo)) => {
                    let lo_ok = lo == pe.policy_ch;
                    let hi_ok = hi == pe.policy_ch;
                    (lo_ok || hi_ok) && !(lo_ok && hi_ok)
                },
                _ => false
            }
        })
        .count()
}

fn main() {
    let p1 = part1();
    println!("DAY 2, PART 1: {}", p1);
    let p2 = part2();
    println!("DAY 2, PART 2: {}", p2);
}