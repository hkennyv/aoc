#![crate_name = "day04"]

use std::collections::HashMap;
use std::fs;

use regex::Regex;

mod passport;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let input: Vec<String> = contents
        .split("\n\n")
        .map(|line| line.to_string())
        .collect();

    let res_p1 = input
        .iter()
        .map(|line| parse_passport(&line))
        .map(|passport| passport_is_valid_p1(passport))
        .filter(|res| *res == true)
        .count();

    println!("RESULT pt1: {}", res_p1);
}

/// returns true if the passport contains all of the required fields
fn passport_is_valid_p1(passport: passport::Passport) -> bool {
    let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let optional_fields = vec!["cid"];

    for field in required_fields {
        // if !passport.contains_key(&String::from(field)) {
        if !passport.contains_key(field) {
            return false;
        }
    }

    true
}

/// returns true if the passport contains all the required fields and has valid
/// data
// fn passport_is_valid_p2(passport: Passport) -> bool {
//     for (key, value) in passport {
//         match key.as_str() {
//             "byr" => {
//                 let year: i32 = value.parse().unwrap();
//                 (year >= 1920) && (year <= 2002)
//             }
//             "iyr" => {
//                 let year: i32 = value.parse().unwrap();
//                 (year >= 2010) && (year <= 2020)
//             }
//             "eyr" => {
//                 let year: i32 = value.parse().unwrap();
//                 (year >= 2020) && (year <= 2030)
//             }
//             "hgt" => {
//                 let re = Regex::new(r"(\d.*)(cm|in)").unwrap();
//                 match re.captures(&value) {
//                     Some(cap) => {
//                         let unit = cap.get(2).map_or("", |m| m.as_str());
//                         let height: i32 = cap.get(1).map_or(0, |m| m.as_str().parse().unwrap());

//                         if 
//                     },
//                     None => false,
//                 }
//             }
//             "hcl" => true,
//             "ecl" => true,
//             "pid" => true,
//             _ => false,
//         };
//     }

//     true
// }

/// parses the key, value pairs from the passport string and returns a hashmap
fn parse_passport(data: &String) -> passport::Passport {
    let mut passport: passport::Passport = HashMap::new();

    let split = data.split_whitespace().for_each(|field| {
        let field_split: Vec<&str> = field.split(":").collect();
        let key = field_split[0];
        let value = field_split[1];

        passport.insert(String::from(key), String::from(value));
    });

    passport
}
