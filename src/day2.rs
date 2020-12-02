use std::io::Read;
use std::str::FromStr;

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
    let input: Vec<_> = input.lines().filter_map(|line| {
        let mut parts = line.split_whitespace();
        let occurs = parts.next().unwrap();
        let mut occurs_parts = occurs.split('-');
        let min_occurs = usize::from_str(occurs_parts.next().unwrap()).unwrap();
        let max_occurs = usize::from_str(occurs_parts.next().unwrap()).unwrap();
        let letter = parts.next().unwrap().chars().nth(0).unwrap();
        let password = parts.next().unwrap().to_string();
        Some(Line { min_occurs, max_occurs, letter, password })
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
