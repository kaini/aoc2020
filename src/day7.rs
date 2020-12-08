extern crate regex;

use std::io::Read;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

#[derive(Debug)]
struct Rule<'a> {
    count: usize,
    color: &'a str,
}

const MY_BAG: &str = "shiny gold";

fn main() {
    let rule_regex = Regex::new(r"(?m)^([a-z ]+) bags contain(.*+)$").unwrap();
    let sub_rule_regex = Regex::new(r" (\d+) ([a-z ]+) bags?[,.]").unwrap();

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let outer_to_inners: HashMap<_, Vec<_>> = HashMap::from_iter(rule_regex
        .captures_iter(&input)
        .map(|caps| (
            caps.get(1).unwrap().as_str(),
            sub_rule_regex
                .captures_iter(caps.get(2).unwrap().as_str())
                .map(|caps| Rule {
                    count: caps.get(1).unwrap().as_str().parse().unwrap(),
                    color: caps.get(2).unwrap().as_str(),
                })
                .collect()
        )));
    
    let mut inner_to_outers = HashMap::new();
    for (outer, inners) in &outer_to_inners {
        for inner in inners {
            if !inner_to_outers.contains_key(inner.color) {
                inner_to_outers.insert(inner.color, Vec::new());
            }
            inner_to_outers.get_mut(inner.color).unwrap().push(Rule { color: outer, count: inner.count });
        }
    }
    
    let mut dfs_stack = vec![MY_BAG];
    let mut dfs_visited = HashSet::new();
    while let Some(current_node) = dfs_stack.pop() {
        if !dfs_visited.insert(current_node) {
            continue;
        }
        
        if let Some(outers) = inner_to_outers.get(current_node) {
            for outer in outers {
                dfs_stack.push(outer.color);
            }
        }
    }
    dfs_visited.remove(MY_BAG);
    println!("{}", dfs_visited.len());

    let mut dfs_stack = vec![(MY_BAG, 1)];
    let mut sum = 0;
    while let Some((current_node, current_cost)) = dfs_stack.pop() {
        sum += current_cost;
        if let Some(inners) = outer_to_inners.get(current_node) {
            for inner in inners {
                dfs_stack.push((inner.color, current_cost * inner.count));
            }
        }
    }
    sum -= 1;
    println!("{}", sum);
}

