extern crate regex;

use std::io::Read;
use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let numbers: Vec<_> = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect();
    
    let mut chain = Vec::new();
    chain.push(0);
    chain.extend(&numbers);
    chain.sort();
    chain.push(chain.last().unwrap() + 3);
    let mut a = 0;
    let mut b = 0;
    for i in 1..chain.len() {
        match chain[i] - chain[i - 1] {
            1 => { a += 1; }
            2 => {}
            3 => { b += 1; }
            other => { panic!("Bad difference {}", other); }
        }
    }
    println!("{}", a * b);

    println!("{}", count_rec(&mut HashMap::new(), &chain, 0));
}

fn count_rec(cache: &mut HashMap<usize, u64>, numbers: &[u64], numbers_at: usize) -> u64 {
    if let Some(&count) = cache.get(&numbers_at) {
        return count;
    }

    if numbers_at == numbers.len() - 1 {
        return 1;
    }

    let mut count = 0;
    for next_choice in (numbers_at + 1)..numbers.len() {
        if numbers[next_choice] - numbers[numbers_at] > 3 {
            break;
        }
        count += count_rec(cache, numbers, next_choice);
    }
    cache.insert(numbers_at, count);
    count
}

