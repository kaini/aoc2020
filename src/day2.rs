extern crate regex;

use std::io::Read;
use std::str::FromStr;
use regex::Regex;

#[derive(Debug)]
struct Line {
    min_occurs: usize,
    max_occurs: usize,
    letter: char,
    password: String,
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let regex = Regex::new(r"(?m)^(\d+)-(\d+) (.): (.*)$").unwrap();
    let input: Vec<_> = regex.captures_iter(&input).map(|cap| Line {
        min_occurs: usize::from_str(cap.get(1).unwrap().as_str()).unwrap(),
        max_occurs: usize::from_str(cap.get(2).unwrap().as_str()).unwrap(),
        letter: cap.get(3).unwrap().as_str().chars().next().unwrap(),
        password: cap.get(4).unwrap().as_str().to_string(),
    }).collect();

    println!("{}", input.iter().filter(|line| {
        let count = line.password.chars().filter(|&c| c == line.letter).count();
        line.min_occurs <= count && count <= line.max_occurs
    }).count());

    println!("{}", input.iter().filter(|line| {
        let a = line.password.chars().nth(line.min_occurs - 1).unwrap() == line.letter;
        let b = line.password.chars().nth(line.max_occurs - 1).unwrap() == line.letter;
        (a && !b) || (!a && b)
    }).count());
}
