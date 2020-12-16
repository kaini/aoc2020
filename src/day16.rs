extern crate regex;

use std::io::Read;
use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;
use regex::Regex;
use std::iter::FromIterator;

fn main() {
    let constraint_regex = Regex::new(r"^(.+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut constraints: HashMap<&str, Vec<RangeInclusive<i64>>> = HashMap::new();
    let mut my_ticket: Vec<i64> = vec![];
    let mut other_tickets: Vec<Vec<i64>> = vec![];
    let mut section = 0;
    for line in input.lines() {
        if line.is_empty() {
            section += 1;
        } else {
            match section {
                0 => {
                    let caps = constraint_regex.captures(line).unwrap();
                    let key = caps.get(1).unwrap().as_str();
                    let range1 = caps.get(2).unwrap().as_str().parse().unwrap()..=caps.get(3).unwrap().as_str().parse().unwrap();
                    let range2 = caps.get(4).unwrap().as_str().parse().unwrap()..=caps.get(5).unwrap().as_str().parse().unwrap();
                    constraints.insert(key, vec![range1, range2]);
                }
                1 => {
                    assert_eq!(line, "your ticket:");
                    section += 1;
                }
                2 => {
                    my_ticket = line.split(",").map(|n| n.parse().unwrap()).collect();
                }
                3 => {
                    assert_eq!(line, "nearby tickets:");
                    section += 1;
                }
                4 => {
                    other_tickets.push(line.split(",").map(|n| n.parse().unwrap()).collect());
                }
                _ => {
                    panic!();
                }
            }
        }
    }

    let mut sum = 0;
    let other_tickets: Vec<Vec<i64>> = other_tickets.drain(..).filter(|ticket| {
        let mut keep = true;
        for n in ticket {
            if !constraints.values().any(|cs| cs.iter().any(|c| c.contains(n))) {
                sum += n;
                keep = false;
            }
        }
        keep
    }).collect();
    println!("{}", sum);

    let mut possibilities: HashMap<&str, HashSet<usize>> = HashMap::from_iter(constraints.keys().map(|&k| (k, HashSet::from_iter(0..my_ticket.len()))));
    let mut uniques_found = vec![];
    let mut new_uniques_found = vec![];
    for ticket in &other_tickets {
        for (i, n) in ticket.iter().enumerate() {
            for (&field, possible_indices) in &mut possibilities {
                if possible_indices.contains(&i) && !constraints[field].iter().any(|c| c.contains(&n)) {
                    possible_indices.remove(&i);
                    assert!(possible_indices.len() > 0);
                    if possible_indices.len() == 1 {
                        uniques_found.push(field);
                    }
                }
            }
            while !uniques_found.is_empty() {
                for unique_field in uniques_found.drain(..) {
                    let position = *possibilities[unique_field].iter().next().unwrap();
                    for (&field, possible_indices) in &mut possibilities {
                        if field != unique_field && possible_indices.contains(&position) {
                            possible_indices.remove(&position);
                            assert!(possible_indices.len() > 0);
                            if possible_indices.len() == 1 {
                                new_uniques_found.push(field);
                            }
                        }
                    }
                }
                std::mem::swap(&mut uniques_found, &mut new_uniques_found);
            }
        }
    }
    println!(
        "{}",
        constraints.keys()
            .filter_map(|field| {
                if field.starts_with("departure") {
                    Some(my_ticket[*possibilities[field].iter().next().unwrap()])
                } else {
                    None
                }
            })
            .product::<i64>()
    );
}