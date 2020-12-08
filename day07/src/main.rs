#![crate_name = "day07"]

use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    part1(&contents);
}

fn part1(input: &str) -> i32 {
    let rules = input.lines().map(|line| parse_rule(line));

    let has_golden: HashSet<String> = rules
        .filter(|(_, inside_bags)| inside_bags.contains("shiny gold"))
        .map(|(bag, _)| bag)
        .collect();

    println!("{:?}", has_golden);

    0
}

/// parses a line into a rule. returns the bag and a set of bags that the main
/// bag contains
fn parse_rule(input: &str) -> (String, HashSet<String>) {
    let mut split = input.split("contain");

    // let the key be the first element in the split
    let bag = split
        .next()
        .unwrap()
        .strip_suffix("bags ")
        .unwrap()
        .to_string();

    // parse all of the bags _inside_ of the key
    let inside_bags: HashSet<String> = split
        .next()
        .unwrap()
        .split(',')
        .map(|bag| bag.strip_prefix(' ').unwrap())
        .map(|bag| {
            let mut words = bag.split_whitespace();
            words.next();
            words
                .take(2)
                .map(|word| word.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        })
        .collect();

    (bag, inside_bags)
}
