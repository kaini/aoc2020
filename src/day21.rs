extern crate regex;

use std::io::Read;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

#[derive(Debug)]
struct Line<'a> {
    ingredients: HashSet<&'a str>,
    allergens: HashSet<&'a str>,
}

fn main() {
    let line_regex = Regex::new(r"^(.*) \(contains (.*)\)$").unwrap();

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let lines: Vec<Line> = input.lines().map(|line| {
        let caps = line_regex.captures(line).unwrap();
        Line {
            ingredients: caps.get(1).unwrap().as_str().split_ascii_whitespace().collect(),
            allergens: caps.get(2).unwrap().as_str().split(", ").collect(),
        }
    }).collect();

    let mut alg_to_ing = HashMap::new();
    let mut ings = HashSet::new();
    for line in &lines {
        for &ing in &line.ingredients {
            ings.insert(ing);
            for &alg in &line.allergens {
                alg_to_ing.entry(alg).or_insert(HashSet::new()).insert(ing);
            }
        }
    }

    let assignment = find_solution_rec(&alg_to_ing, &lines).unwrap();
    let unsafe_ings: HashSet<_> = assignment.values().map(|&v| v).collect();
    let safe_ings: HashSet<_> = ings.difference(&unsafe_ings).collect();
    println!("assignments = {:?}, safe ings = {:?}", assignment, safe_ings);
    println!("{}", lines.iter().map(|line| line.ingredients.iter().filter(|ing| safe_ings.contains(ing)).count()).sum::<usize>());

    let rev_assignment: HashMap<_, _> = HashMap::from_iter(assignment.iter().map(|(k, v)| (v, k)));
    let mut dangerous: Vec<_> = unsafe_ings.iter().map(|&v| v).collect();
    dangerous.sort_by_key(|ing| rev_assignment.get(ing).unwrap());
    println!("{}", dangerous.join(","));
}

fn find_solution_rec<'a>(alg_to_ing: &HashMap<&'a str, HashSet<&'a str>>, lines: &[Line<'a>]) -> Option<HashMap<&'a str, &'a str>> {
    if let Some((alg, ings)) = alg_to_ing.iter().next() {
        if ings.len() == 0 {
            None
        } else {
            for ing in ings {
                if lines.iter().all(|line| !line.allergens.contains(alg) || line.ingredients.contains(ing)) {
                    let mut alg_to_ing = alg_to_ing.clone();
                    alg_to_ing.remove(alg);
                    for (_, ings) in &mut alg_to_ing {
                        ings.remove(ing);
                    }

                    if let Some(mut solution) = find_solution_rec(&alg_to_ing, lines) {
                        solution.insert(alg, ing);
                        return Some(solution)
                    }
                }
            }
            None
        }
    } else {
        Some(HashMap::new())
    }
}
