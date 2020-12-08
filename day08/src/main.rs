use anyhow::{Error, Result};

use std::collections::HashSet;
use std::io::Read;
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Operation {
    Accumulate,
    Jump,
    NoOp,
}

#[derive(Debug, Clone)]
struct Instruction {
    operation: Operation,
    argument: i32,
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Instruction> {
        let spl: Vec<&str> = s.split(' ').collect();
        Ok(Instruction {
            operation: match spl[0] {
                "nop" => Operation::NoOp,
                "jmp" => Operation::Jump,
                "acc" => Operation::Accumulate,
                _ => panic!(),
            },
            argument: spl[1].parse()?,
        })
    }
}

#[derive(Debug)]
struct GameConsole {
    program: Vec<Instruction>,
    accumulator: i32,
}

impl GameConsole {
    fn run(&mut self) -> Result<i32, i32> {
        let mut counter: i32 = 0;
        let mut seen_counters: HashSet<i32> = HashSet::new();
        let ninstructions = self.program.len() as i32;
        while !(counter == ninstructions) {
            if seen_counters.contains(&counter) {
                return Err(self.accumulator);
            } else {
                seen_counters.insert(counter);
            }
            if counter < 0 || counter > ninstructions {
                panic!();
            }
            let instruction = &self.program[counter as usize];
            match instruction.operation {
                Operation::Accumulate => self.accumulator += instruction.argument,
                Operation::Jump => {
                    counter += instruction.argument;
                    continue;
                }
                Operation::NoOp => (),
            }
            counter += 1;
        }
        Ok(self.accumulator)
    }

    fn new(program: &str) -> GameConsole {
        GameConsole {
            program: program.lines().map(|l| l.parse().unwrap()).collect(),
            accumulator: 0,
        }
    }
}

fn main() -> Result<()> {
    let contents = get_contents("input")?;
    let contents = contents.trim();

    // part 1
    let mut console = GameConsole::new(contents);
    match console.run() {
        Ok(v) | Err(v) => {
            dbg!(v);
        }
    }

    // part 2
    let program = console.program.clone();
    for (i, instruction) in program.iter().enumerate() {
        let mut console = GameConsole::new(contents);
        match instruction.operation {
            Operation::Accumulate => (),
            Operation::Jump => {
                console.program[i] = Instruction {
                    operation: Operation::NoOp,
                    argument: instruction.argument,
                }
            }
            Operation::NoOp => {
                console.program[i] = Instruction {
                    operation: Operation::Jump,
                    argument: instruction.argument,
                }
            }
        }
        match console.run() {
            Ok(v) => {
                dbg!(v);
                break;
            }
            Err(_) => continue,
        }
    }

    Ok(())
}

fn get_contents(filename: &str) -> Result<String> {
    let mut f = std::fs::File::open(filename)?;

    let mut contents = String::new();

    f.read_to_string(&mut contents)?;

    Ok(contents)
}
