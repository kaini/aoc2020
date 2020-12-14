extern crate regex;

use std::io::Read;
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug, Clone, Copy)]
enum Command {
    Mask(u64, u64, u64),
    Assign(u64, u64),
}

fn main() {
    let mask_regex = Regex::new(r"^mask = ([01X]{36})$").unwrap();
    let assign_regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let commands: Vec<Command> = input
        .lines()
        .map(|line| {
            if let Some(caps) = mask_regex.captures(line) {
                Command::Mask(
                    caps.get(1).unwrap().as_str().chars().rev().enumerate().fold(std::u64::MAX, |value, (idx, chr)| if chr == '0' { value & !(1 << idx) } else { value }),
                    caps.get(1).unwrap().as_str().chars().rev().enumerate().fold(0, |value, (idx, chr)| if chr == '1' { value | (1 << idx) } else { value }),
                    caps.get(1).unwrap().as_str().chars().rev().enumerate().fold(0, |value, (idx, chr)| if chr == 'X' { value | (1 << idx) } else { value }),
                )
            } else if let Some(caps) = assign_regex.captures(line) {
                Command::Assign(
                    caps.get(1).unwrap().as_str().parse().unwrap(),
                    caps.get(2).unwrap().as_str().parse().unwrap(),
                )
            } else {
                panic!("Unknown line: {}", line)
            }
        })
        .collect();

    {
        let mut memory = HashMap::new();
        let mut and_mask = std::u64::MAX;
        let mut or_mask = 0;
        for &command in &commands {
            match command {
                Command::Mask(new_and, new_or, _new_x) => {
                    and_mask = new_and;
                    or_mask = new_or;
                }
                Command::Assign(addr, value) => {
                    memory.insert(addr, (value & and_mask) | or_mask);
                }
            }
        }
        println!("{}", memory.values().sum::<u64>());
    }

    {
        let mut memory = HashMap::new();
        let mut or_mask = 0;
        let mut x_mask = 0;
        for &command in &commands {
            match command {
                Command::Mask(_new_and, new_or, new_x) => {
                    or_mask = new_or;
                    x_mask = new_x;
                }
                Command::Assign(addr, value) => {
                    let addr = (addr | or_mask) & !x_mask;
                    let mut current_x = !x_mask;
                    for _ in 0..2u64.pow(x_mask.count_ones()) {
                        let x_or = current_x & x_mask;
                        let x_and = current_x | !x_mask;
                        memory.insert((addr | x_or) & x_and, value);
                        current_x = current_x.wrapping_add(1) | !x_mask;
                    }
                }
            }
        }
        println!("{}", memory.values().sum::<u64>());
    }
}

