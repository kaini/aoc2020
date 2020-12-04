extern crate regex;

use std::io::Read;
use std::str::FromStr;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use regex::Regex;

fn main() {
    let field_validations: HashMap<_, _> = HashMap::from_iter([
        ("byr", Regex::from_str(r"^((19[2-9][0-9])|(200[012]))$").unwrap()),
        ("iyr", Regex::from_str(r"^((201[0-9])|2020)$").unwrap()),
        ("eyr", Regex::from_str(r"^((202[0-9])|2030)$").unwrap()),
        ("hgt", Regex::from_str(r"^((((1[5678][0-9])|(19[0123]))cm)|(((59)|(6[0-9])|(7[0123456]))in))$").unwrap()),
        ("hcl", Regex::from_str(r"^#[0-9a-f]{6}$").unwrap()),
        ("ecl", Regex::from_str(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap()),
        ("pid", Regex::from_str(r"^[0-9]{9}$").unwrap()),
    ].iter().cloned());
    let required_fields: HashSet<_> = HashSet::from_iter(field_validations.keys());
    let passport_separator_regex = Regex::from_str(r"(\n\n)|(\r\r)|(\r\n\r\n)|(\n\r\n\r)").unwrap();
    let field_regex = Regex::from_str(r"([a-z]+):(.+?)(?:\s+|$)").unwrap();

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let passports: Vec<HashMap<_, _>> = passport_separator_regex
        .split(&input)
        .map(|passport| HashMap::from_iter(field_regex
            .captures_iter(passport)
            .map(|caps| (caps.get(1).unwrap().as_str(), caps.get(2).unwrap().as_str()))))
        .collect();

    println!("{}", passports.iter().filter(|passport| required_fields.is_subset(&HashSet::from_iter(passport.keys()))).count());

    println!("{}", passports.iter().filter(|passport|
        field_validations.iter().all(|(key, regex)| regex.is_match(passport.get(key).unwrap_or(&"")))
    ).count());
}
