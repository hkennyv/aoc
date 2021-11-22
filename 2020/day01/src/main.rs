#![crate_name = "day01"]

//! ## --- Day 1: Report Repair ---
//!
//! After saving Christmas five years in a row, you've decided to take a vacation at a nice resort on a tropical island. Surely, Christmas will go on without you.
//!
//! The tropical island has its own currency and is entirely cash-only. The gold coins used there have a little picture of a starfish; the locals just call them stars. None of the currency exchanges seem to have heard of them, but somehow, you'll need to find fifty of these coins by the time you arrive so you can pay the deposit on your room.
//!
//! To save your vacation, you need to get all fifty stars by December 25th.
//!
//! Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!
//!
//! Before you leave, the Elves in accounting just need you to fix your expense report (your puzzle input); apparently, something isn't quite adding up.
//!
//! Specifically, they need you to find the two entries that sum to 2020 and then multiply those two numbers together.
//!
//! For example, suppose your expense report contained the following:
//!
//! ```
//! 1721
//! 979
//! 366
//! 299
//! 675
//! 1456
//! ```
//!
//! In this list, the two entries that sum to 2020 are 1721 and 299. Multiplying them together produces 1721 \* 299 = 514579, so the correct answer is 514579.
//!
//! Of course, your expense report is much larger. Find the two entries that sum to 2020; what do you get if you multiply them together?
//!
//! Your puzzle answer was 538464.
//!
//! ## --- Part Two ---
//!
//! The Elves in accounting are thankful for your help; one of them even offers you a starfish coin they had left over from a past vacation. They offer you a second one if you can find three numbers in your expense report that meet the same criteria.
//!
//! Using the above example again, the three entries that sum to 2020 are 979, 366, and 675. Multiplying them together produces the answer, 241861950.
//!
//! In your expense report, what is the product of the three entries that sum to 2020?
//!
//! Your puzzle answer was 278783190.

use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs;

/// The main function prints out the results for part1 and part2 of the day01
/// AOC
fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let input: Vec<i32> = contents
        .split('\n')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    let res_pt1 = part1(&input).unwrap();
    println!("Part 1 result: {}\n", res_pt1);

    let res_pt2 = part2(&input).unwrap();
    println!("Part 2 result: {}\n", res_pt2);
}

/// The part1 function calculates the result for part1
fn part1(input: &[i32]) -> Result<i32, String> {
    let unique_inputs: HashSet<i32> = input.iter().cloned().collect();

    // find compliment
    for val in unique_inputs.iter() {
        let compliment = 2020 - val;
        if unique_inputs.contains(&compliment) {
            println!("RESULT: {} + {} = 2020", compliment, val);
            println!("\t{} x {} = {}", compliment, val, compliment * val);
            return Ok(compliment * val);
        }
    }

    Err(String::from("no result :(\n"))
}

/// The part2 function calculates the result for part2
fn part2(input: &[i32]) -> Result<i32, String> {
    // clone and sort input
    let mut input = input.to_owned();
    input.sort_unstable();

    // find 3 numbers that add up to 2020
    // use 3 indices to track our place in the vec (2 on the left, 1 on the
    // right)
    let target = 2020;

    let mut i1 = 0;
    let mut i2 = 1;
    let mut i3 = input.len() - 1;

    loop {
        let n1 = input[i1];
        let n2 = input[i2];
        let n3 = input[i3];

        let sum = n1 + n2 + n3;

        match sum.cmp(&target) {
            // if the sum is less than the target, we want to increment one of
            // the left indices to get a higher sum
            Ordering::Less => {
                if (i2 - i1) > 1 {
                    i1 += 1;
                } else {
                    i2 += 1;
                }

                if !assert_indices_havent_crossed(i1, i2, i3) {
                    return Err(String::from("no result :(\n"));
                }
            }

            // if the sum is greater than the target, we want to decrement the
            // right index to get a smaller number to lower our sum
            Ordering::Greater => {
                i3 -= 1;

                if !assert_indices_havent_crossed(i1, i2, i3) {
                    return Err(String::from("no result :(\n"));
                }
            }

            // if the sum is equal, then we have found a result!!
            Ordering::Equal => {
                println!("RESULT: {} + {} + {} = {}", n1, n2, n3, target);
                println!("\t{} x {} x {} = {}", n1, n2, n3, n1 * n2 * n3);
                return Ok(n1 * n2 * n3);
            }
        }
    }
}

/// utility function that helps to check index bounds
fn assert_indices_havent_crossed(i1: usize, i2: usize, i3: usize) -> bool {
    (i1 < i2) && (i2 < i3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assert_indices_havent_crossed() {
        assert!(assert_indices_havent_crossed(1, 2, 3));
        assert!(assert_indices_havent_crossed(5, 6, 13));
        assert!(assert_indices_havent_crossed(0, 2, 3));
        assert!(assert_indices_havent_crossed(6, 1000, 1001));
    }

    #[test]
    fn test_assert_indices_havent_crossed_fails() {
        assert!(!assert_indices_havent_crossed(2, 2, 3));
        assert!(!assert_indices_havent_crossed(1, 2, 2));
        assert!(!assert_indices_havent_crossed(5, 3, 1));
        assert!(!assert_indices_havent_crossed(1, 1, 1));
    }
}
