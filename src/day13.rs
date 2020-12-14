extern crate regex;

use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let lines: Vec<_> = input
        .lines()
        .collect();
    let start_time = lines[0].parse::<i64>().unwrap();
    let busses: Vec<Option<i64>> = lines[1].split(",").map(|n| if n == "x" { None } else { Some(n.parse().unwrap()) }).collect();

    let (next_bus, next_time) = busses.iter()
        .map(|&bus| {
            if let Some(bus) = bus {
                assert!(start_time % bus != 0);
                (bus, (start_time / bus + 1) * bus)
            } else {
                (0, std::i64::MAX)
            }
        })
        .min_by_key(|&(_bus, next_time)| next_time)
        .unwrap();
    println!("{}", next_bus * (next_time - start_time));

    let mut solution = 0;
    let mut modulus = 1;
    for (minute, &bus) in busses.iter().enumerate() {
        if let Some(bus) = bus {
            let goal_rem = if minute == 0 { 0 } else { bus - (minute as i64 % bus) };
            while solution % bus != goal_rem {
                solution += modulus;
            }
            modulus *= bus;
        }
    }
    println!("{}", solution);
}

