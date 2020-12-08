extern crate regex;

use std::io::Read;
use regex::Regex;
use std::u32;

fn main() {
    let group_seperator_regex = Regex::new(r"(\n\n)|(\r\r)|(\r\n\r\n)").unwrap();

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let forms: Vec<_> = group_seperator_regex
        .split(&input)
        .map(|group| group
            .lines()
            .fold((0, u32::MAX), |(value_or, value_and), line| {
                let value = line
                    .chars()
                    .fold(0, |value, chr| value | (1 << (chr as u32 - 'a' as u32)));
                (value_or | value, value_and & value)
            })
        )
        .collect();
    
    println!("{}", forms.iter().map(|form| form.0.count_ones()).sum::<u32>());

    println!("{}", forms.iter().map(|form| form.1.count_ones()).sum::<u32>());
}
