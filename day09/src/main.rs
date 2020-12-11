#![crate_name = "day09"]

//! ## --- Day 9: Encoding Error ---
//!
//! With your neighbor happily enjoying their video game, you turn your attention to an open data port on the little screen in the seat in front of you.
//!
//! Though the port is non-standard, you manage to connect it to your computer through the clever use of several paperclips. Upon connection, the port outputs a series of numbers (your puzzle input).
//!
//! The data appears to be encrypted with the eXchange-Masking Addition System (XMAS) which, conveniently for you, is an old cypher with an important weakness.
//!
//! XMAS starts by transmitting a preamble of 25 numbers. After that, each number you receive should be the sum of any two of the 25 immediately previous numbers. The two numbers will have different values, and there might be more than one such pair.
//!
//! For example, suppose your preamble consists of the numbers 1 through 25 in a random order. To be valid, the next number must be the sum of two of those numbers:
//!
//! ```
//! 26 would be a valid next number, as it could be 1 plus 25 (or many other pairs, like 2 and 24).
//! 49 would be a valid next number, as it is the sum of 24 and 25.
//! 100 would not be valid; no two of the previous 25 numbers sum to 100.
//! 50 would also not be valid; although 25 appears in the previous 25 numbers, the two numbers in the pair must be different.
//! Suppose the 26th number is 45, and the first number (no longer an option, as it is more than 25 numbers ago) was 20. Now, for the next number to be valid, there needs to be some pair of numbers among 1-19, 21-25, or 45 that add up to it:
//!
//! 26 would still be a valid next number, as 1 and 25 are still within the previous 25 numbers.
//! 65 would not be valid, as no two of the available numbers sum to it.
//! 64 and 66 would both be valid, as they are the result of 19+45 and 21+45 respectively.
//! ```
//!
//! Here is a larger example which only considers the previous 5 numbers (and has a preamble of length 5):
//!
//! ```
//! 35
//! 20
//! 15
//! 25
//! 47
//! 40
//! 62
//! 55
//! 65
//! 95
//! 102
//! 117
//! 150
//! 182
//! 127
//! 219
//! 299
//! 277
//! 309
//! 576
//! ```
//!
//! In this example, after the 5-number preamble, almost every number is the sum of two of the previous 5 numbers; the only number that does not follow this rule is 127.
//!
//! The first step of attacking the weakness in the XMAS data is to find the first number in the list (after the preamble) which is not the sum of two of the 25 numbers before it. What is the first number that does not have this property?
//!
//! Your puzzle answer was 26134589.
//!
//! ## --- Part Two ---
//!
//! The final step in breaking the XMAS encryption relies on the invalid number you just found: you must find a contiguous set of at least two numbers in your list which sum to the invalid number from step 1.
//!
//! Again consider the above example:
//!
//! ```
//! 35
//! 20
//! 15
//! 25
//! 47
//! 40
//! 62
//! 55
//! 65
//! 95
//! 102
//! 117
//! 150
//! 182
//! 127
//! 219
//! 299
//! 277
//! 309
//! 576
//! ```
//!
//! In this list, adding up all of the numbers from 15 through 40 produces the invalid number from step 1, 127. (Of course, the contiguous set of numbers in your actual list might be much longer.)
//!
//! To find the encryption weakness, add together the smallest and largest number in this contiguous range; in this example, these are 15 and 47, producing 62.
//!
//! What is the encryption weakness in your XMAS-encrypted list of numbers?
//!
//! Your puzzle answer was 3535124.

use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let input = parse_input(&contents);

    let res_p1 = part1(&input, 25);
    println!("res p1: {}", res_p1);

    let res_p2 = part2(&input, res_p1);
    println!("res p2: {}", res_p2);
}

/// finds the first number that does not have 2 numbers in the preamble that
/// adds up to the value immediately after the preamble
fn part1(input: &[i64], preamble_size: usize) -> i64 {
    let windows = input.windows(preamble_size);

    for (i, window) in windows.enumerate() {
        let value = input[i + preamble_size];
        let mut has_sum = false;

        for num in window {
            if has_sum {
                break;
            }

            let compliment = value - *num;

            if window.contains(&compliment) {
                has_sum = true;
            }
        }

        if !has_sum {
            return value;
        }
    }

    -1
}

/// returns the min plus the max of a set of contiguous numbers that adds up
/// to the `res_p1`
fn part2(input: &[i64], res_p1: i64) -> i64 {
    let mut nums: VecDeque<i64> = VecDeque::new();
    let mut sum = 0;

    let mut idx = 0;

    loop {
        let num = input[idx];

        match sum.cmp(&res_p1) {
            Ordering::Less => {
                nums.push_back(num);
                sum += num;
                idx += 1;
            }
            Ordering::Greater => {
                let removed = nums.pop_front().unwrap();
                sum -= removed;
            }
            Ordering::Equal => {
                break;
            }
        }
    }

    nums.iter().max().unwrap() + nums.iter().min().unwrap()
}

/// parses the input from a String to a Vec<i32>
fn parse_input(input: &str) -> Vec<i64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_p1() {
        let contents = fs::read_to_string("sample.txt").unwrap();
        let input = parse_input(&contents);

        assert_eq!(part1(&input, 5), 127);
    }

    #[test]
    fn test_sample_p2() {
        let contents = fs::read_to_string("sample.txt").unwrap();
        let input = parse_input(&contents);
        let res_p1 = part1(&input, 5);

        assert_eq!(part2(&input, res_p1), 62);
    }
}
