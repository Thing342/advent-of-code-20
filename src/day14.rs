use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

const FILE: &str = "inputs/day14.txt";

type Int = u64;

#[derive(Debug, Clone)]
struct Bitmask {
    //masks: Vec<Int>
    m0: Int,
    m1: Int,
    mask: String,
}

impl Bitmask {
    fn apply(&self, a: Int) -> Int {
        (!a & self.m0) | (a & self.m1)
    }

    fn apply_mem(&self, addr: Int) -> Vec<Int> {
        let g = self.mask.chars().rev().enumerate()
            .fold(vec![addr], |mut addrs, (i, c)| {
            match c {
                '0' => {},
                '1' => addrs.iter_mut().for_each(|a| *a |= 1 << i),
                'X' => {
                    addrs.extend(addrs.clone().iter().map(|a| match (a >> i) & 1 {
                        0 => a | (1 << i),
                        1 => a & !(1 << i),
                        _ => unreachable!(),
                    }))
                }
                _ => panic!()
            }
            addrs
        });

        //g.iter().for_each(|n| eprintln!("{:064b}", *n));

        g
    }
}

#[derive(Debug)]
enum Instruction {
    SetBitmask(Bitmask),
    SetMemory { addr: Int, val: Int },
}

#[derive(Debug)]
struct Machine {
    bitmask: Bitmask,
    data: HashMap<Int, Int>,
}

impl Machine {
    fn new() -> Self {
        Machine {
            bitmask: Bitmask { m0: 0, m1: 0, mask: "".to_string() },
            data: HashMap::new(),
        }
    }

    fn sum(&self) -> Int {
        self.data.iter().map(|(_, v)| *v).sum()
    }
}

pub fn main() {
    let file = File::open(FILE).expect("Failed to open file");
    let rdr = BufReader::new(file);
    let instr = rdr.lines()
        .filter_map(|r| r.ok())
        .map(|s| {
            let mut tokens = s.split(" ");
            match tokens.next().unwrap() {
                "mask" => {
                    tokens.next();
                    let mask = tokens.next().unwrap();

                    let (m0, m1) = mask.chars().fold((0, 0), |mut acc, next| {
                        match next {
                            '0' => { acc.0 |= 0; acc.1 |= 0 },
                            '1' => { acc.0 |= 1; acc.1 |= 1 },
                            'X' => { acc.0 |= 0; acc.1 |= 1 },
                            _ => panic!()
                        }
                        acc.0 <<= 1;
                        acc.1 <<= 1;
                        acc
                    });

                    Instruction::SetBitmask(Bitmask {m0, m1, mask: mask.to_string()})
                }
                mem if mem.starts_with("mem") => {
                    let lbracket = mem.find('[').unwrap();
                    let rbracket = mem.rfind(']').unwrap();
                    let addr = mem[(lbracket + 1)..rbracket].parse().unwrap();
                    tokens.next();
                    let val = tokens.next().unwrap().parse().unwrap();
                    Instruction::SetMemory { addr, val }
                }
                _ => panic!("Couldn't parse instruction!")
            }
        })
        .collect::<Vec<_>>();

    //eprintln!("{:#?}", instr);

    let p1_st = instr.iter().fold(Machine::new(), |mut machine, inst| match inst {
        Instruction::SetBitmask(bitmask) => {
            machine.bitmask = bitmask.clone();
            machine
        }
        Instruction::SetMemory { addr, val } => {
            machine.data.insert(*addr, machine.bitmask.apply(*val));
            machine
        }
    });

    //eprintln!("m0: {:#064b} \nm1: {:#064b}", p1_st.bitmask.m0, p1_st.bitmask.m1);
    //eprintln!("{:#?}", &p1_st);
    println!("DAY 14, PART 1: {}", p1_st.sum());

    let p2_st = instr.iter().fold(Machine::new(), |mut machine, inst| match inst {
        Instruction::SetBitmask(bitmask) => {
            machine.bitmask = bitmask.clone();
            machine
        }
        Instruction::SetMemory { addr, val } => {
            for addr in machine.bitmask.apply_mem(*addr) {
                machine.data.insert(addr, *val);
            }
            machine
        }
    });

    //eprintln!("{:#?}", &p2_st);
    println!("DAY 14, PART 2: {}", p2_st.sum());
}