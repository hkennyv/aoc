#![crate_name = "day07"]

//! ## --- Day 7: Handy Haversacks ---
//!
//! You land at the regional airport in time for your next flight. In fact, it looks like you'll even have time to grab some food: all flights are currently delayed due to issues in luggage processing.
//!
//! Due to recent aviation regulations, many rules (your puzzle input) are being enforced about bags and their contents; bags must be color-coded and must contain specific quantities of other color-coded bags. Apparently, nobody responsible for these regulations considered how long they would take to enforce!
//!
//! For example, consider the following rules:
//!
//! ```
//! light red bags contain 1 bright white bag, 2 muted yellow bags.
//! dark orange bags contain 3 bright white bags, 4 muted yellow bags.
//! bright white bags contain 1 shiny gold bag.
//! muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
//! shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
//! dark olive bags contain 3 faded blue bags, 4 dotted black bags.
//! vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
//! faded blue bags contain no other bags.
//! dotted black bags contain no other bags.
//! ```
//!
//! These rules specify the required contents for 9 bag types. In this example, every faded blue bag is empty, every vibrant plum bag contains 11 bags (5 faded blue and 6 dotted black), and so on.
//!
//! You have a shiny gold bag. If you wanted to carry it in at least one other bag, how many different bag colors would be valid for the outermost bag? (In other words: how many colors can, eventually, contain at least one shiny gold bag?)
//!
//! In the above rules, the following options would be available to you:
//!
//! ```
//! A bright white bag, which can hold your shiny gold bag directly.
//! A muted yellow bag, which can hold your shiny gold bag directly, plus some other bags.
//! A dark orange bag, which can hold bright white and muted yellow bags, either of which could then hold your shiny gold bag.
//! A light red bag, which can hold bright white and muted yellow bags, either of which could then hold your shiny gold bag.
//! So, in this example, the number of bag colors that can eventually contain at least one shiny gold bag is 4.
//! ```
//!
//! How many bag colors can eventually contain at least one shiny gold bag? (The list of rules is quite long; make sure you get all of it.)
//!
//! Your puzzle answer was 235.
//!
//! ## --- Part Two ---
//!
//! It's getting pretty expensive to fly these days - not because of ticket prices, but because of the ridiculous number of bags you need to buy!
//!
//! Consider again your shiny gold bag and the rules from the above example:
//!
//! ```
//! faded blue bags contain 0 other bags.
//! dotted black bags contain 0 other bags.
//! vibrant plum bags contain 11 other bags: 5 faded blue bags and 6 dotted black bags.
//! dark olive bags contain 7 other bags: 3 faded blue bags and 4 dotted black bags.
//! ```
//!
//! So, a single shiny gold bag must contain 1 dark olive bag (and the 7 bags within it) plus 2 vibrant plum bags (and the 11 bags within each of those): 1 + 1*7 + 2 + 2*11 = 32 bags!
//!
//! Of course, the actual rules have a small chance of going several levels deeper than this example; be sure to count all of the bags, even if the nesting becomes topologically impractical!
//!
//! Here's another example:
//!
//! ```
//! shiny gold bags contain 2 dark red bags.
//! dark red bags contain 2 dark orange bags.
//! dark orange bags contain 2 dark yellow bags.
//! dark yellow bags contain 2 dark green bags.
//! dark green bags contain 2 dark blue bags.
//! dark blue bags contain 2 dark violet bags.
//! dark violet bags contain no other bags.
//! ```
//!
//! In this example, a single shiny gold bag must contain 126 other bags.
//!
//! How many individual bags are required inside your single shiny gold bag?
//!
//! Your puzzle answer was 158493.

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let res_p1 = part1(&contents);
    println!("res p1: {}", res_p1);

    let res_p2 = part2(&contents);
    println!("res p2: {}", res_p2);
}

/// calculates the number of bag colors that can eventually contain at least
/// one "shiny gold" bag
fn part1(input: &str) -> i32 {
    let rules: Vec<(String, HashSet<String>)> =
        input.lines().map(|line| parse_rule(line)).collect();

    let mut has_golden: HashSet<String> = HashSet::new();
    let mut bags: HashSet<String> = HashSet::new();
    bags.insert("shiny gold".to_string());

    loop {
        bags = update_bags(&rules, &bags);
        has_golden.extend(bags.clone());

        if bags.len() == 0 {
            break;
        }
    }

    has_golden.len() as i32
}

/// calculates the number of bags inside of a "shiny gold" bag
fn part2(input: &str) -> i32 {
    let rules: HashMap<String, Vec<(i32, String)>> =
        input.lines().map(|line| parse_rule2(line)).collect();

    // setup cache
    let mut count_cache = HashMap::new();

    // setup starting bag
    let bag = "shiny gold".to_string();

    count_bag(&rules, &mut count_cache, bag)
}

/// iterates through the rules and checks for bag that contain the bags that
/// you pass in. it returns a set with the bags that contain the passed in
/// bags
fn update_bags(rules: &[(String, HashSet<String>)], bags: &HashSet<String>) -> HashSet<String> {
    let has_bags: HashSet<String> = rules
        .iter()
        .filter(|(_, inside_bags)| {
            if inside_bags
                .intersection(bags)
                .collect::<HashSet<&String>>()
                .len()
                == 0
            {
                false
            } else {
                true
            }
        })
        .map(|(bag, _)| bag.to_string())
        .collect();

    has_bags
}

/// counts the number of bags inside of `bag`. updates `cache` with the values
/// for each bag
fn count_bag(
    rules: &HashMap<String, Vec<(i32, String)>>,
    cache: &mut HashMap<String, i32>,
    bag: String,
) -> i32 {
    // check cache first
    if cache.contains_key(&bag) {
        return *cache.get(&bag).unwrap();
    }

    // otherwise start counting bags
    let mut count = 0;
    let inside_bags = rules.get(&bag).unwrap();

    // for each `inside_bag` inside your `bag`, you want to count how many
    // bags are inside the `inside_bag`
    for (qty, inside_bag) in inside_bags {
        // this is the base case, when the qty is 0
        if *qty == 0 {
            if !cache.contains_key(&bag) {
                cache.insert(bag.clone(), *qty);
            }
            return 0;
        }

        // add the number of `bag`s AND the number of `bag`s multiplied by the
        // number of bags inside
        count += qty + qty * count_bag(rules, cache, inside_bag.to_string());
    }

    // update cache once we successfully get a count for a bag
    cache.insert(bag.clone(), count);
    count
}

/// parses a line into a rule. returns the bag and a set of bags that the main
/// bag contains
fn parse_rule(input: &str) -> (String, HashSet<String>) {
    let mut split = input.split("contain");

    // let the key be the first element in the split
    let bag = split
        .next()
        .unwrap()
        .strip_suffix(" bags ")
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

/// parses a line into a rule. returns the bag and a vector that contains
/// the inside bags and their quantities
fn parse_rule2(input: &str) -> (String, Vec<(i32, String)>) {
    let mut split = input.split("contain");

    // let the key be the first element in the split
    let bag = split
        .next()
        .unwrap()
        .strip_suffix(" bags ")
        .unwrap()
        .to_string();

    // parse all of the bags _inside_ of the key
    let inside_bags: Vec<(i32, String)> = split
        .next()
        .unwrap()
        .split(',')
        .map(|bag| bag.strip_prefix(' ').unwrap())
        .map(|bag| {
            let mut words = bag.split_whitespace();
            let qty: i32 = words.next().unwrap().parse().unwrap_or(0);

            let color = words
                .take(2)
                .map(|word| word.to_string())
                .collect::<Vec<String>>()
                .join(" ");

            (qty, color)
        })
        .collect();

    (bag, inside_bags)
}
