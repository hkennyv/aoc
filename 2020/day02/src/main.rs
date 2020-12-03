#![crate_name = "day02"]

//! ## --- Day 2: Password Philosophy ---
//!
//! Your flight departs in a few days from the coastal airport; the easiest way down to the coast from here is via toboggan.
//!
//! The shopkeeper at the North Pole Toboggan Rental Shop is having a bad day. "Something's wrong with our computers; we can't log in!" You ask if you can take a look.
//!
//! Their password database seems to be a little corrupted: some of the passwords wouldn't have been allowed by the Official Toboggan Corporate Policy that was in effect when they were chosen.
//!
//! To try to debug the problem, they have created a list (your puzzle input) of passwords (according to the corrupted database) and the corporate policy when that password was set.
//!
//! For example, suppose you have the following list:
//!
//! ```
//! 1-3 a: abcde
//! 1-3 b: cdefg
//! 2-9 c: ccccccccc
//! ```
//!
//! Each line gives the password policy and then the password. The password policy indicates the lowest and highest number of times a given letter must appear for the password to be valid. For example, 1-3 a means that the password must contain a at least 1 time and at most 3 times.
//!
//! In the above example, 2 passwords are valid. The middle password, cdefg, is not; it contains no instances of b, but needs at least 1. The first and third passwords are valid: they contain one a or nine c, both within the limits of their respective policies.
//!
//! How many passwords are valid according to their policies?
//!
//! ## --- Part Two ---
//!
//! While it appears you validated the passwords correctly, they don't seem to be what the Official Toboggan Corporate Authentication System is expecting.
//!
//! The shopkeeper suddenly realizes that he just accidentally explained the password policy rules from his old job at the sled rental place down the street! The Official Toboggan Corporate Policy actually works a little differently.
//!
//! Each policy actually describes two positions in the password, where 1 means the first character, 2 means the second character, and so on. (Be careful; Toboggan Corporate Policies have no concept of "index zero"!) Exactly one of these positions must contain the given letter. Other occurrences of the letter are irrelevant for the purposes of policy enforcement.
//!
//! Given the same example list from above:
//!
//! ```
//! 1-3 a: abcde is valid: position 1 contains a and position 3 does not.
//! 1-3 b: cdefg is invalid: neither position 1 nor position 3 contains b.
//! 2-9 c: ccccccccc is invalid: both position 2 and position 9 contain c.
//! ```
//!
//! How many passwords are valid according to the new interpretation of the policies?
//!

use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let input: Vec<String> = contents.lines().map(|line| line.to_string()).collect();

    // part 1
    let results_p1: Vec<bool> = input
        .iter()
        .map(|row| handle_row_p1(String::from(row)))
        .collect();

    println!(
        "part1 result: {}",
        results_p1.iter().filter(|res| **res).count()
    );

    // part 2
    let results_p2: Vec<bool> = input
        .iter()
        .map(|row| handle_row_p2(String::from(row)))
        .collect();

    println!(
        "part2 result: {}",
        results_p2.iter().filter(|res| **res).count()
    );
}

/// part1 row handler
fn handle_row_p1(row: String) -> bool {
    let (policy, password) = parse_input_row(row);
    password_is_valid_p1(policy, password)
}

/// part2 row handler
fn handle_row_p2(row: String) -> bool {
    let (policy, password) = parse_input_row(row);
    password_is_valid_p2(policy, password)
}

/// determines if the password is valid (based on part1 criteria)
fn password_is_valid_p1(policy: String, password: String) -> bool {
    let (min, max, letter) = parse_policy(policy);

    let mut letter_count: HashMap<char, i32> = HashMap::new();

    for ch in password.chars() {
        let count = letter_count.entry(ch).or_insert(0);
        *count += 1;
    }

    if letter_count.contains_key(&letter) {
        let val = *letter_count.get(&letter).unwrap();
        return val >= min && val <= max;
    } else {
        return false;
    }
}

/// determines if the password is valid (based on part2 criteria)
fn password_is_valid_p2(policy: String, password: String) -> bool {
    println!("{} {}", policy, password);
    let (pos1, pos2, letter) = parse_policy(policy);
    println!("{} {} {}", pos1, pos2, letter);

    if pos2 > password.len() as i32 {
        return false;
    }

    let pos1_match = (password.chars().nth((pos1 - 1) as usize).unwrap()) == letter;
    let pos2_match = (password.chars().nth((pos2 - 1) as usize).unwrap()) == letter;

    (pos1_match || pos2_match) && !(pos1_match && pos2_match)
}

/// takes in a row in the format:
///     policy: password
/// and it returns a tuple containing the (policy, password)
fn parse_input_row(row: String) -> (String, String) {
    let split: Vec<&str> = row.split(":").collect();
    let policy = String::from(split[0]);
    let mut password = String::from(split[1]);
    password = password.strip_prefix(" ").unwrap().to_string();

    (policy, password)
}

/// parses the policy into a two numbers and a letter
fn parse_policy(policy: String) -> (i32, i32, char) {
    let split: Vec<&str> = policy.split(" ").collect();
    let nums = split[0];
    let letter = split[1].chars().nth(0).unwrap();

    let nums_split: Vec<i32> = nums.split("-").map(|n| n.parse::<i32>().unwrap()).collect();
    let num1 = nums_split[0];
    let num2 = nums_split[1];

    (num1, num2, letter)
}
