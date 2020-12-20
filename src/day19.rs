extern crate regex;

use std::io::Read;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
enum Rule {
    Or(Vec<Rule>),
    Seq(Vec<Rule>),
    Literal(String),
    Ref(usize),
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut parsing_rules = true;
    let mut input_lines = Vec::new();
    let mut rules = HashMap::new();
    for line in input.lines() {
        if parsing_rules {
            if line.is_empty() {
                parsing_rules = false;
                continue;
            }
            let (id, rule) = parse_rule(line);
            rules.insert(id, rule);
        } else {
            input_lines.push(line);
        }
    }

    println!("{}", input_lines.iter().filter(|line| matches(&rules, line)).count());

    rules.insert(8, parse_rule("8: 42 | 42 8").1);
    rules.insert(11, parse_rule("11: 42 31 | 42 11 31").1);
    println!("{}", input_lines.iter().filter(|line| matches(&rules, line)).count());
}

fn parse_rule(line: &str) -> (usize, Rule) {
    let rule_regex = Regex::new(r"^(\d+): (.+)$").unwrap();
    let rule_caps = rule_regex.captures(line).unwrap();
    let id: usize = rule_caps.get(1).unwrap().as_str().parse().unwrap();
    let rule_atoms: Vec<_> = rule_caps.get(2).unwrap().as_str().split_whitespace().collect();
    let rule = Rule::Or(rule_atoms
        .split(|&atom| atom == "|")
        .map(|atoms| Rule::Seq(
            atoms.iter().map(|atom|
                if atom.chars().nth(0).unwrap() == '"' {
                    Rule::Literal(atom[1..(atom.len() - 1)].to_owned())
                } else {
                    Rule::Ref(atom.parse().unwrap())
                }
            )
            .collect()
        ))
        .collect()
    );
    (id, rule)
}

fn matches(rules: &HashMap<usize, Rule>, input: &str) -> bool {
    matches_rec(rules, rules.get(&0).unwrap(), input).iter().any(|s| s.is_empty())
}

fn matches_rec<'a>(rules: &HashMap<usize, Rule>, rule: &Rule, input: &'a str) -> Vec<&'a str> {
    match rule {
        Rule::Or(items) => {
            items.iter()
                .flat_map(|item| matches_rec(rules, item, input))
                .collect()
        }
        Rule::Seq(items) => {
            let mut current_inputs = vec![input];
            let mut next_inputs = vec![];
            for item in items {
                for input in &current_inputs {
                    next_inputs.append(&mut matches_rec(rules, item, input));
                }
                std::mem::swap(&mut current_inputs, &mut next_inputs);
                next_inputs.clear();
            }
            current_inputs
        }
        Rule::Literal(literal) => {
            if input.starts_with(literal) {
                vec![&input[literal.len()..]]
            } else {
                vec![]
            }
        }
        Rule::Ref(r) => {
            matches_rec(rules, rules.get(r).unwrap(), input)
        }
    }
}
