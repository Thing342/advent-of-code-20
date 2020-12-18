use std::fs::File;
use std::io::{BufReader, BufRead};
use std::str::Chars;
use itertools::Itertools;
use regex::internal::Char;

const FILE: &str = "inputs/day18.txt";

type Int = i64;

#[derive(Debug)]
enum Token {
    Number(Int), Add, Mul
}

fn eval1(ts: &mut Chars) -> Int {
    let mut stack = vec![];
    while let Some(c) = ts.next() {
        //eprintln!("{} {:?}", c, &stack);
        match c {
            ws if ws.is_whitespace() => { },
            n if n.is_ascii_digit() => {
                let value = (n as Int) - ('0' as Int);
                let op = stack.pop();
                let v = stack.pop();

                match (op, v) {
                    (Some(Token::Mul), Some(Token::Number(i))) => stack.push(Token::Number(value * i)),
                    (Some(Token::Add), Some(Token::Number(i))) => stack.push(Token::Number(value + i)),
                    (None, None) => stack.push(Token::Number(value)),
                    (_, _) => panic!()
                }
            },
            '+' => {
                stack.push(Token::Add);
            },
            '*' => {
                stack.push(Token::Mul);
            },
            '(' => {
                let value = eval1(ts);
                let op = stack.pop();
                let v = stack.pop();

                match (op, v) {
                    (Some(Token::Mul), Some(Token::Number(i))) => stack.push(Token::Number(value * i)),
                    (Some(Token::Add), Some(Token::Number(i))) => stack.push(Token::Number(value + i)),
                    (None, None) => stack.push(Token::Number(value)),
                    (_, _) => panic!()
                }
            },
            ')' => {
                break;
            }
            _ => panic!("unrec token")
        }
    }

    match stack.pop() {
        Some(Token::Number(i)) => i,
        _ => panic!()
    }
}

fn eval2(ts: &mut Chars) -> Int {

    let mut stack = vec![];
    while let Some(c) = ts.next() {
        //eprintln!("{} {:?}", c, &stack);
        match c {
            ws if ws.is_whitespace() => { },
            n if n.is_ascii_digit() => {
                let value = (n as Int) - ('0' as Int);
                let op = stack.pop();
                //let v = stack.pop();

                match op {
                    Some(Token::Mul) => {
                        stack.push(Token::Mul);
                        stack.push(Token::Number(value))
                    },
                    Some(Token::Add) => {
                        if let Some(Token::Number(i)) = stack.pop() {
                            stack.push(Token::Number(value + i))
                        } else {
                            unreachable!()
                        }
                    },
                    None => {
                        stack.push(Token::Number(value))
                    },
                    _ => unreachable!()
                }
            },
            '+' => {
                stack.push(Token::Add);
            },
            '*' => {
                stack.push(Token::Mul);
            },
            '(' => {
                let value = eval2(ts);
                let op = stack.pop();
                //let v = stack.pop();

                match op {
                    Some(Token::Mul) => {
                        stack.push(Token::Mul);
                        stack.push(Token::Number(value))
                    },
                    Some(Token::Add) => {
                        if let Some(Token::Number(i)) = stack.pop() {
                            stack.push(Token::Number(value + i))
                        } else {
                            unreachable!()
                        }
                    },
                    None => {
                        stack.push(Token::Number(value))
                    },
                    _ => unreachable!()
                }
            },
            ')' => {
                break;
            }
            _ => panic!("unrec token")
        }
    }

    //eprintln!("{:?}", stack);
    while stack.len() > 1 {
        match (stack.pop(), stack.pop(), stack.pop()) {
            (Some(Token::Number(i)), Some(Token::Add), Some(Token::Number(j))) => stack.push(Token::Number(i + j)),
            (Some(Token::Number(i)), Some(Token::Mul), Some(Token::Number(j))) => stack.push(Token::Number(i * j)),
            _ => panic!()
        }
    }

    match stack.pop() {
        Some(Token::Number(i)) => i,
        _ => panic!()
    }
}

pub fn main() {
    let file = File::open(FILE).expect("Failed to open file");
    let rdr = BufReader::new(file);

    let mut sol: (Int, Int) = rdr.lines()
        .filter_map(|r| r.ok())
        .map(|s| {
            eprintln!("\t{}", &s);
            let i = eval1(&mut s.chars());
            eprintln!("\t = {}", i);

            let j = eval2(&mut s.chars());
            eprintln!("\t = {}", j);

            (i, j)
        })
        .fold((0,0), |(mut i, mut j), (is, js)| {
            i+=is;
            j+=js;
            (i,j)
        });

    println!("DAY 18, PART 1: {:?}", sol.0);
    println!("DAY 18, PART 2: {:?}", sol.1);

}

#[test]
fn parse_test() {
    let s = "1 + 2 * 3 + 4 * 5 + 6";
    eprintln!("{:#?}", eval1(&mut s.chars()));

    let s = "1 + (2 * 3) + (4 * (5 + 6))";
    eprintln!("{:#?}", eval1(&mut s.chars()));

    let s = "(4 * 6 + 7 + 7 * 4 + 7) * (5 + 6 * 6 * 6 * 8) * 8 * (5 * 4 * 9 + 4 + 3 * 9) * 3";
    eprintln!("{:#?}", eval1(&mut s.chars()));

    let s = "1 + 2 * 3 + 4 * 5 + 6";
    eprintln!("{:#?}", eval2(&mut s.chars()));

    let s = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
    eprintln!("{:#?}", eval2(&mut s.chars()));

    let s = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
    eprintln!("{:#?}", eval2(&mut s.chars()));
}