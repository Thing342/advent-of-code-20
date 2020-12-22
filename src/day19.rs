use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use regex::Regex;

use itertools::Itertools;

const FILE: &str = "inputs/day19.txt";

type Int = usize;

#[derive(Debug)]
enum Expansion {
    Unexpanded(String),
    Expanded(String)
}

impl Expansion {
    fn as_str(&self) -> &str {
        match self {
            Self::Unexpanded(s) => s.as_str(),
            Self::Expanded(s) => s.as_str(),
        }
    }

    fn to_regex(&self, line: bool) -> Regex {
        if line {
            Regex::new(&format!("^{}$", self.as_str())).unwrap()
        } else {
            Regex::new(self.as_str()).unwrap()
        }
    }
}

fn create_regexes(rules: &HashMap<Int, String>) -> HashMap<Int, Expansion> {
    use Expansion::{*};

    let mut stack = vec![0];
    let mut exps = HashMap::new();

    let mut x = 0;
    while let Some(rule_id) = stack.last() {
        x += 1;

        let rule_id = *rule_id;
        let mut expansion = String::new();
        let rule = if let Some(rule) = exps.remove(&rule_id) {
            rule
        } else {
            Unexpanded(rules.get(&rule_id).unwrap().to_string())
        };

        let mut expanded = true;
        for token in rule.as_str().split(" ") {
            match token {
                "\"a\"" => expansion.push('a'),
                "\"b\"" => expansion.push('b'),
                "|" => {
                    expansion += "|";
                },
                token if token.chars().any(|c| c.is_ascii_digit()) => {
                    expanded = false;
                    let ref_rule_id = token.parse::<Int>().unwrap();
                    match exps.get(&ref_rule_id) {
                        Some(Expanded(s)) => {
                            expansion.push('(');
                            expansion += s.as_str();
                            expansion.push(')')
                        },
                        Some(Unexpanded(_)) | None => {
                            stack.push(ref_rule_id);
                            expansion.push(' ');
                            expansion += token;
                            expansion.push(' ');
                        }
                    }
                },
                token => {
                    expansion += token;
                }
            }
        }

        if expanded {
            exps.insert(rule_id, Expanded(expansion));
            stack.pop();
        } else {
            exps.insert(rule_id, Unexpanded(expansion));
        }
    }

    exps
}

pub fn main() {
    let file = File::open(FILE).expect("Failed to open file");
    let rdr = BufReader::new(file);
    let mut lines = rdr.lines().filter_map(|r| r.ok()).collect::<Vec<_>>();

    let rules = lines.iter()
        .take_while(|s| s.len() > 1)
        .map(|s| {
            //eprintln!("{}", &s);
            let mut spl = s.split(":");
            let rule_no = spl.next().expect("bad rule parse").parse::<Int>().expect("bad rule number");
            let rule_def = spl.next().expect("bad rule parse").trim().to_string();
            (rule_no, rule_def)
        }).collect::<HashMap<_, _>>();

    let mut regexes = create_regexes(&rules);

    let regex_0 = regexes.get(&0).unwrap().to_regex(true);
    //eprintln!("0: {}", regex_0);

    let p1 = lines.iter()
        .skip(rules.len() + 1)
        .filter(|s| regex_0.is_match(s.as_str()))
        .count();

    println!("DAY 19, PART 1: {}", p1);

    let regex31 = regexes.get(&31).unwrap().as_str();
    let regex42 = regexes.get(&42).unwrap().as_str();

    let fake11 = (1..=5)
        .rev()
        .map(|i| format!("({}{{{}}})({}{{{}}})", regex42, i, regex31, i))
        .join("|");

    let regex_p2 = Regex::new(&format!("({})+({})?$", regex42, fake11)).unwrap();

    let p2 = lines.iter()
        .skip(rules.len() + 1)
        .filter(|s| {
            let z = regex_p2.is_match(s.as_str());
            //eprintln!("{} {}", s, z);
            z
        })
        .count();

    println!("DAY 19, PART 2: {}", p2);

}

#[test]
fn test_regexes() {

}