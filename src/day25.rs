extern crate regex;

use std::io::Read;

const MOD: usize = 20201227;
const SUBJECT_NO: usize = 7;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    
    let public_a: usize = input.lines().nth(0).unwrap().parse().unwrap();
    let public_b: usize = input.lines().nth(1).unwrap().parse().unwrap();

    let mut loop_size = 0;
    let mut result = 1;
    while result != public_a && result != public_b {
        result *= SUBJECT_NO;
        result %= MOD;
        loop_size += 1;
    }
    println!("{}", loop_size);

    let subject_no = if result == public_a { public_b } else { public_a };
    let mut result = 1;
    for _ in 0..loop_size {
        result *= subject_no;
        result %= MOD;
    }
    println!("{}", result);
}
