use std::io::Read;
use std::str::FromStr;

const GOAL: usize = 2020;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let numbers: Vec<_> = input.split_whitespace().map(|number| usize::from_str(number).unwrap()).collect();

    let mut bitfield = vec![false; *numbers.iter().max().unwrap() + 1];
    for &n in &numbers {
        bitfield[n] = true;
    }
    for &n in &numbers {
        if bitfield[GOAL - n] {
            println!("{}", n * (GOAL - n));
            break;
        }
    }

    'out2: for i in 0..numbers.len() {
        for j in (i + 1)..numbers.len() {
            for k in (j + 1)..numbers.len() {
                if numbers[i] + numbers[j] + numbers[k] == GOAL {
                    println!("{}", numbers[i] * numbers[j] * numbers[k]);
                    break 'out2;
                }
            }
        }
    }
}
