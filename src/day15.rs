extern crate regex;

use std::collections::HashMap;

const INPUT: &str = "9,12,1,4,17,0,18";

fn main() {
    let starting_numbers: Vec<i32> = INPUT.split(",").map(|n| n.parse().unwrap()).collect();
    println!("{}", get_nth(&starting_numbers, 2020));
    println!("{}", get_nth(&starting_numbers, 30000000));
}

fn get_nth(starting_numbers: &[i32], count: i32) -> i32 {
    let mut age = HashMap::new(); 
    let mut last_number = 0;
    for turn in 0..count {
        let next_number = if (turn as usize) < starting_numbers.len() {
            starting_numbers[turn as usize]
        } else if let Some(prev_turn) = age.get(&last_number) {
            turn - prev_turn
        } else {
            0
        };
        age.insert(last_number, turn);
        last_number = next_number;
    }
    last_number 
}
