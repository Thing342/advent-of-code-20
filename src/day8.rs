use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;
use self::Opcode::{JMP, NOP};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Opcode {
    NOP,
    ACC,
    JMP,
}

#[derive(Debug)]
struct Instruction {
    opcode: Opcode,
    amt: i32,
}

#[derive(Debug)]
struct State {
    acc: i32,
    ins: i32,
    visited: HashSet<i32>
}

fn run(instructions: &[Instruction], flip_ins: Option<i32>) -> Result<State, State> {
    let mut state = State {
        acc: 0, ins: 0, visited: HashSet::with_capacity(instructions.len())
    };

    loop {
        if state.ins == instructions.len() as i32 {
            return Ok(state)
        }

        let instruction = &instructions[state.ins as usize];
        state.visited.insert(state.ins);

        let opcode = match (instruction.opcode, flip_ins) {
            (JMP, Some(idx)) if idx == state.ins => NOP,
            (NOP, Some(idx)) if idx == state.ins => JMP,
            (o, _) => o,
        };

        match opcode {
            Opcode::NOP => {
                state.ins += 1;
            },
            Opcode::ACC => {
                state.ins += 1;
                state.acc += instruction.amt;
            },
            Opcode::JMP => {
                let new_ins = state.ins + instruction.amt;
                if state.visited.contains(&new_ins) {
                    return Err(state);
                } else {
                    state.ins = new_ins;
                }
            }
        }
    }
}

pub fn main() {
    let file = File::open("inputs/day8.txt").expect("Failed to open file");
    let rdr = BufReader::new(file);
    let instructions: Vec<Instruction> = rdr.lines()
        .filter_map(|f| f.ok())
        .map(|s|
            Instruction {
                opcode: match &s[0..3] {
                    "jmp" => Opcode::JMP,
                    "acc" => Opcode::ACC,
                    "nop" => Opcode::NOP,
                    _ => panic!()
                },

                amt: (&s[4..s.len()]).parse().unwrap(),
            }
        ).collect();

    let p1_state = run(&instructions, None).err().unwrap();

    println!("DAY 8, PART 1: {}", p1_state.acc);

    for (i, instr) in instructions.iter().rev().enumerate() {
        let ins = instructions.len() - i - 1;
        if instr.opcode == JMP || instr.opcode == NOP {
            if let Ok(p2_state) = run(&instructions, Some(ins as i32)) {
                //eprintln!("{} {:?}", ins, instr);
                println!("DAY 8, PART 2: {}", p2_state.acc);
            }
        }
    }
}