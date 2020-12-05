extern crate regex;

use std::io::Read;
use std::collections::HashSet;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let boarding_passes: HashSet<_> = input
        .lines()
        .map(|line| usize::from_str_radix(&line.chars().map(|c| match c {
            'B' | 'R' => '1',
            'F' | 'L' => '0',
            _ => panic!(),
        }).collect::<String>(), 2).unwrap())
        .collect();
        
    println!("{}", boarding_passes.iter().max().unwrap());

    for seat in 1..0b1111111111 {
        if !boarding_passes.contains(&seat) && boarding_passes.contains(&(seat - 1)) && boarding_passes.contains(&(seat + 1)) {
            println!("{}", seat);
            break;
        }
    }
}
