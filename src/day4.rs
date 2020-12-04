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
    MissingByr, MissingIyr, MissingEyr, MissingHgt, MissingHcl, MissingEcl, MissingPid
}

fn parse_passport(input: &str) -> Result<Passport, PassportError> {
    let mut bindings: HashMap<&str, &str> = HashMap::with_capacity(8);
    for record in input.split_ascii_whitespace() {
        let mut fields = record.split(':');
        let key = fields.next().unwrap();
        let value = fields.next().unwrap();
        bindings.insert(key, value);
    }

    eprintln!("{:#?}", bindings);

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

fn part1() -> usize {
    std::fs::read_to_string("inputs/day4.txt").unwrap()
        .split("\n\n")
        .map(parse_passport)
        .filter_map(|passport| passport.ok())
        .count()
}

fn part2() -> usize {
    0
}

fn main() {
    let p1 = part1();
    println!("DAY 4, PART 1: {}", p1);
    //let p2 = part2();
    //println!("DAY 3, PART 2: {}", p2);
}