use anyhow::anyhow;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug)]
struct Instruction {
    operation: String,
    argument: Option<i32>,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" ");

        match (split.next(), split.next(), split.next()) {
            (Some("addx"), Some(argument), None) => Ok(Instruction {
                operation: "addx".to_string(),
                argument: Some(argument.parse::<i32>()?),
            }),
            (Some("noop"), None, None) => Ok(Instruction {
                operation: "noop".to_string(),
                argument: None,
            }),
            _ => Err(anyhow!("Invalid instruction")),
        }
    }
}

fn load_instructions(path: &str) -> anyhow::Result<Vec<Instruction>> {
    BufReader::new(File::open(path)?)
        .lines()
        .map(|line| line?.parse::<Instruction>())
        .collect()
}

type Register = i32;
type Cycle = i32;
type SignalStrength = i32;

struct Cpu {
    register: Register,
    register_changes: HashMap<Cycle, Register>,
}

impl Cpu {
    fn run_program(instructions: &[Instruction]) -> Self {
        let mut register_changes: HashMap<Cycle, Register> = HashMap::new();
        let mut register: Register = 1;
        let mut cycles_count: Cycle = 1;

        for instruction in instructions {
            cycles_count += 1;
            register_changes.insert(cycles_count as Cycle, register);
            match instruction.operation.as_str() {
                "addx" => {
                    register += instruction.argument.unwrap();
                    cycles_count += 1;
                    register_changes.insert(cycles_count as Cycle, register);
                }
                "noop" => {}
                _ => {}
            }
        }

        Self {
            register,
            register_changes,
        }
    }

    fn signal_of_cycle(&self, cycle: Cycle) -> SignalStrength {
        match self.register_changes.get(&cycle) {
            Some(signal) => signal.to_owned() * cycle,
            None => 0,
        }
    }

    fn sum_of_signal(&self) -> SignalStrength {
        [20, 60, 100, 140, 180, 220]
            .iter()
            .map(|cycle| self.signal_of_cycle(*cycle))
            .sum()
    }
}

pub fn day10() -> anyhow::Result<()> {
    let instructions = load_instructions("data/day10.txt")?;

    let cpu = Cpu::run_program(&instructions);
    println!("Day 10 part 1: {}", cpu.sum_of_signal());

    Ok(())
}
