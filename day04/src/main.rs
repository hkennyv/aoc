#![crate_name = "day04"]

use std::collections::HashMap;
use std::fs;

use regex::Regex;

mod passport;
use passport::ValidatePassport;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let input: Vec<String> = contents
        .split("\n\n")
        .map(|line| line.to_string())
        .collect();

    let res_p1 = input
        .iter()
        .map(|line| parse_passport(&line))
        .map(|passport| passport.is_valid_p1())
        .filter(|res| *res == true)
        .count();

    println!("RESULT pt1: {}", res_p1);

    let res_p2 = input
        .iter()
        .map(|line| parse_passport(&line))
        .map(|passport| passport.is_valid_p2())
        .filter(|res| *res == true)
        .count();

    println!("RESULT pt2: {}", res_p2);
}

/// parses the key, value pairs from the passport string and returns a hashmap
fn parse_passport(data: &String) -> passport::Passport {
    let mut passport: passport::Passport = HashMap::new();

    data.split_whitespace().for_each(|field| {
        let field_split: Vec<&str> = field.split(":").collect();
        let key = field_split[0];
        let value = field_split[1];

        passport.insert(String::from(key), String::from(value));
    });

    passport
}
