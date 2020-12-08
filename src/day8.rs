extern crate regex;

use std::io::Read;
use regex::Regex;
use std::convert::TryInto;

#[derive(Debug, Clone, Copy)]
enum Opcode {
    Nop, Acc, Jmp,
}

#[derive(Debug, Clone)]
struct Instruction {
    opcode: Opcode,
    value: i64,
}

#[derive(Debug)]
struct Machine {
    program_counter: i64,
    accumulator: i64,
}

impl Machine {
    fn new() -> Machine {
        Machine { program_counter: 0, accumulator: 0, }
    }

    fn execute_instruction(&mut self, instruction: &Instruction) {
        match instruction.opcode {
            Opcode::Nop => { self.program_counter += 1; }
            Opcode::Acc => { self.program_counter += 1; self.accumulator += instruction.value; }
            Opcode::Jmp => { self.program_counter += instruction.value; }
        }
    }

    fn execute_program(&mut self, program: &[Instruction]) {
        let program_len = program.len().try_into().unwrap();
        let mut executed = vec![false; program.len()];
        while self.program_counter >= 0 && self.program_counter < program_len && !executed[self.program_counter as usize] {
            executed[self.program_counter as usize] = true;
            self.execute_instruction(&program[self.program_counter as usize]);
        }
    }
}

fn main() {
    let operation_regex = Regex::new(r"^([a-z]+) ([+-]\d+)$").unwrap();

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let program: Vec<_> = input
        .lines()
        .map(|line| {
            let caps = operation_regex.captures(line).unwrap();
            Instruction {
                opcode: match caps.get(1).unwrap().as_str() {
                    "nop" => Opcode::Nop,
                    "acc" => Opcode::Acc,
                    "jmp" => Opcode::Jmp,
                    _ => panic!("unknown opcode"),
                },
                value: caps.get(2).unwrap().as_str().parse().unwrap(),
            }
        })
        .collect();
    
    let mut machine = Machine::new();
    machine.execute_program(&program);
    println!("{}", machine.accumulator);

    for i in 0..program.len() {
        let mut fixed_program = program.clone();
        fixed_program[i].opcode = match fixed_program[i].opcode {
            Opcode::Nop => Opcode::Jmp,
            Opcode::Jmp => Opcode::Nop,
            other => other,
        };
        let mut machine = Machine::new();
        machine.execute_program(&fixed_program);
        if let Ok(program_counter) = TryInto::<usize>::try_into(machine.program_counter) {
            if program_counter == fixed_program.len() {
                println!("{}", machine.accumulator);
                break;
            }
        }
    }
}

