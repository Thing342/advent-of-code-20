use std::collections::{HashMap};

#[derive(Debug)]
struct Passport {
    byr: usize,
    iyr: usize,
    eyr: usize,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>
}

#[derive(Debug)]
enum PassportError {
    MissingByr,
    MissingIyr,
    MissingEyr,
    MissingHgt,
    MissingHcl,
    MissingEcl,
    MissingPid,
}

fn parse_passport(input: &str) -> Result<Passport, PassportError> {
    let mut bindings: HashMap<&str, &str> = HashMap::with_capacity(8);
    for record in input.split_ascii_whitespace() {
        let mut fields = record.split(':');
        let key = fields.next().unwrap();
        let value = fields.next().unwrap();
        bindings.insert(key, value);
    }

    //eprintln!("{:#?}", bindings);

    let passport = Passport {
        byr: if let Some(val) = bindings.get("byr") { val.parse().unwrap() } else { return Err(PassportError::MissingByr) },
        iyr: if let Some(val) = bindings.get("iyr") { val.parse().unwrap() } else { return Err(PassportError::MissingIyr) },
        eyr: if let Some(val) = bindings.get("eyr") { val.parse().unwrap() } else { return Err(PassportError::MissingEyr) },
        pid: if let Some(val) = bindings.get("pid") { val.to_string() } else { return Err(PassportError::MissingPid) },
        hgt: if let Some(val) = bindings.get("hgt") { val.to_string() } else { return Err(PassportError::MissingHgt) },
        hcl: if let Some(val) = bindings.get("hcl") { val.to_string() } else { return Err(PassportError::MissingHcl) },
        ecl: if let Some(val) = bindings.get("ecl") { val.to_string() } else { return Err(PassportError::MissingEcl) },
        cid: bindings.get("cid").map(|s| s.to_string())
    };

    Ok(passport)
}

fn valid_height(height: &str) -> bool {
    if height.ends_with("in") {
        if let Ok(height) = height[0..height.len()-2].parse() {
            59 <= height && height <= 76
        } else {
            //eprintln!("Unreadable height (in)");
            false
        }
    } else if height.ends_with("cm") {
        if let Ok(height) = height[0..height.len()-2].parse() {
            150 <= height && height <= 193
        } else {
            //eprintln!("Unreadable height (cm)");
            false
        }
    } else {
        //eprintln!("Unreadable height unit");
        false
    }
}

fn valid_hcl(hcl: &str) -> bool {
    if !(hcl.starts_with("#")) { return false }
    for (i, c) in hcl.chars().enumerate() {
        if i == 0 { continue }
        if !('0' <= c && c <= '9') && !('a' <= c && c <= 'f') {
            return false
        }
    }
    true
}

fn valid_ecl(ecl: &str) -> bool {
    match ecl {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
        _ => {
            //eprintln!("invalid ecl {}", x);
            false
        }
    }
}

fn valid_pid(pid: &str) -> bool {
    if pid.len() != 9 { return false }
    pid.parse::<u64>().is_ok()
}


fn part1() -> usize {
    std::fs::read_to_string("inputs/day4.txt").unwrap()
        .split("\n\n")
        .filter_map(|s| parse_passport(s).ok())
        .count()
}

fn part2() -> usize {
    std::fs::read_to_string("inputs/day4.txt").unwrap()
        .split("\n\n")
        .filter_map(|s| parse_passport(s).ok())

        .filter(|passport| 1920 <= passport.byr && passport.byr <= 2002 )
        .filter(|passport| 2010 <= passport.iyr && passport.iyr <= 2020)
        .filter(|passport| 2020 <= passport.eyr && passport.eyr <= 2030)
        .filter(|passport| valid_height(passport.hgt.as_str()))
        .filter(|passport| valid_hcl(passport.hcl.as_str()))
        .filter(|passport| valid_ecl(passport.ecl.as_str()))
        .filter(|passport| valid_pid(passport.pid.as_str()))

        .count()
}

pub fn main() {
    let p1 = part1();
    println!("DAY 4, PART 1: {}", p1);
    let p2 = part2();
    println!("DAY 4, PART 2: {}", p2);
}