#![crate_name = "day04"]

//! ## --- Day 4: Secure Container ---
//!
//! You arrive at the Venus fuel depot only to discover it's protected by a password. The Elves had written the password on a sticky note, but someone threw it out.
//!
//! However, they do remember a few key facts about the password:
//!
//! ```
//! It is a six-digit number.
//! The value is within the range given in your puzzle input.
//! Two adjacent digits are the same (like 22 in 122345).
//! Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
//! ```
//!
//! Other than the range rule, the following are true:
//!
//! ```
//! 111111 meets these criteria (double 11, never decreases).
//! 223450 does not meet these criteria (decreasing pair of digits 50).
//! 123789 does not meet these criteria (no double).
//! ```
//!
//! How many different passwords within the range given in your puzzle input meet these criteria?
//!
//! Your puzzle answer was 1919.
//!
//! ## --- Part Two ---
//!
//! An Elf just remembered one more important detail: the two adjacent matching digits are not part of a larger group of matching digits.
//!
//! Given this additional criterion, but still ignoring the range rule, the following are now true:
//!
//! ```
//! 112233 meets these criteria because the digits never decrease and all repeated digits are exactly two digits long.
//! 123444 no longer meets the criteria (the repeated 44 is part of a larger group of 444).
//! 111122 meets the criteria (even though 1 is repeated more than twice, it still contains a double 22).
//! ```
//!
//! How many different passwords within the range given in your puzzle input meet all of the criteria?
//!
//! Your puzzle answer was 1291.

use std::collections::HashMap;
use std::fs;

fn main() {
    let input = read_input("input.txt");
    let part1 = part1(input.0, input.1);
    let part2 = part2(input.0, input.1);

    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
}

fn part1(min: usize, max: usize) -> usize {
    let mut valid_passwords: Vec<usize> = Vec::new();

    for num in min..max + 1 {
        if is_valid_password(num) {
            valid_passwords.push(num);
        }
    }

    valid_passwords.len()
}

fn part2(min: usize, max: usize) -> usize {
    let mut valid_passwords: Vec<usize> = Vec::new();

    for num in min..max + 1 {
        if is_valid_password2(num) {
            valid_passwords.push(num);
        }
    }

    valid_passwords.len()
}

fn read_input(filename: &str) -> (usize, usize) {
    let string = fs::read_to_string(filename).expect("Couldn't read file");

    let mut split = string
        .split('-')
        .map(|s| s.parse::<usize>().expect("Couldn't parse usize"));

    (
        split.next().expect("empty split!"),
        split.next().expect("empty split!"),
    )
}

fn is_valid_password(mut num: usize) -> bool {
    let mut has_double = false;

    // get first rightmost digit
    let mut digit = num % 10;
    num /= 10;

    while num > 0 {
        let next_digit = num % 10;
        num /= 10;

        // never decreasing means that the next digit must be _lower or equal_
        // to the current digit
        if next_digit > digit {
            return false;
        }

        // check for duplicate
        if next_digit == digit {
            has_double = true;
        }

        // rotate digit
        digit = next_digit;
    }

    has_double
}

fn is_valid_password2(mut num: usize) -> bool {
    let mut digits: HashMap<usize, usize> = HashMap::new();

    // get first rightmost digit
    let mut digit = num % 10;
    digits.insert(digit, 1);

    num /= 10;

    while num > 0 {
        let next_digit = num % 10;
        num /= 10;

        // never decreasing means that the next digit must be _lower or equal_
        // to the current digit
        if next_digit > digit {
            return false;
        }

        // rotate digit and add the current digit to the hashmap
        digit = next_digit;
        let count = digits.entry(digit).or_insert(0);
        *count += 1;
    }

    // sanity check, number of digits should add up to 6
    assert_eq!(digits.values().sum::<usize>(), 6);

    let has_double: bool = digits.values().any(|&count| count == 2);

    has_double
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_password() {
        let pairs = vec![(111111, true), (223450, false), (123789, false)];

        for (num, expected) in pairs {
            assert_eq!(is_valid_password(num), expected);
        }
    }

    #[test]
    fn test_is_valid_password2() {
        let pairs = vec![
            (112233, true),
            (123444, false),
            (111122, true),
            (223450, false),
            (123789, false),
        ];

        for (num, expected) in pairs {
            assert_eq!(is_valid_password2(num), expected);
        }
    }
}
