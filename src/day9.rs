extern crate regex;

use std::io::Read;

const PREFIX_LEN: usize = 25;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let numbers: Vec<_> = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect();
    
    let mut goal = 0;
    'outer: for i in PREFIX_LEN..numbers.len() {
        for a in &numbers[(i - PREFIX_LEN)..i] {
            for b in &numbers[(i - PREFIX_LEN)..i] {
                if a != b {
                    if a + b == numbers[i] {
                        continue 'outer;
                    }
                }
            }
        }
        goal = numbers[i];
        println!("{}", goal);
        break;
    }
    assert_ne!(goal, 0);

    let goal = goal;
    let mut left = 0;
    let mut right = 0;
    let mut sum = numbers[0];
    loop {
        if sum == goal {
            println!("{}", numbers[left..=right].iter().max().unwrap() + numbers[left..=right].iter().min().unwrap());
            break;
        } else if sum < goal {
            right += 1;
            sum += numbers[right];
        } else if sum > goal {
            sum -= numbers[left];
            left += 1;
        }
        assert!(left <= right);
        assert!(right < numbers.len());
    }
}

