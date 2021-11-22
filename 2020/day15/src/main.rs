#![crate_name = "day15"]

//! ## --- Day 15: Rambunctious Recitation ---
//!
//! You catch the airport shuttle and try to book a new flight to your vacation island. Due to the storm, all direct flights have been cancelled, but a route is available to get around the storm. You take it.
//!
//! While you wait for your flight, you decide to check in with the Elves back at the North Pole. They're playing a memory game and are ever so excited to explain the rules!
//!
//! In this game, the players take turns saying numbers. They begin by taking turns reading from a list of starting numbers (your puzzle input). Then, each turn consists of considering the most recently spoken number:
//!
//! If that was the first time the number has been spoken, the current player says 0.
//! Otherwise, the number had been spoken before; the current player announces how many turns apart the number is from when it was previously spoken.
//! So, after the starting numbers, each turn results in that player speaking aloud either 0 (if the last number is new) or an age (if the last number is a repeat).
//!
//! For example, suppose the starting numbers are 0,3,6:
//!
//! ```
//! Turn 1: The 1st number spoken is a starting number, 0.
//! Turn 2: The 2nd number spoken is a starting number, 3.
//! Turn 3: The 3rd number spoken is a starting number, 6.
//! Turn 4: Now, consider the last number spoken, 6. Since that was the first time the number had been spoken, the 4th number spoken is 0.
//! Turn 5: Next, again consider the last number spoken, 0. Since it had been spoken before, the next number to speak is the difference between the turn number when it was last spoken (the previous turn, 4) and the turn number of the time it was most recently spoken before then (turn 1). Thus, the 5th number spoken is 4 - 1, 3.
//! Turn 6: The last number spoken, 3 had also been spoken before, most recently on turns 5 and 2. So, the 6th number spoken is 5 - 2, 3.
//! Turn 7: Since 3 was just spoken twice in a row, and the last two turns are 1 turn apart, the 7th number spoken is 1.
//! Turn 8: Since 1 is new, the 8th number spoken is 0.
//! Turn 9: 0 was last spoken on turns 8 and 4, so the 9th number spoken is the difference between them, 4.
//! Turn 10: 4 is new, so the 10th number spoken is 0.
//! ```
//!
//! (The game ends when the Elves get sick of playing or dinner is ready, whichever comes first.)
//!
//! Their question for you is: what will be the 2020th number spoken? In the example above, the 2020th number spoken will be 436.
//!
//! Here are a few more examples:
//!
//! ```
//! Given the starting numbers 1,3,2, the 2020th number spoken is 1.
//! Given the starting numbers 2,1,3, the 2020th number spoken is 10.
//! Given the starting numbers 1,2,3, the 2020th number spoken is 27.
//! Given the starting numbers 2,3,1, the 2020th number spoken is 78.
//! Given the starting numbers 3,2,1, the 2020th number spoken is 438.
//! Given the starting numbers 3,1,2, the 2020th number spoken is 1836.
//! Given your starting numbers, what will be the 2020th number spoken?
//! ```
//!
//! Your puzzle answer was 1085.
//!
//! ## --- Part Two ---
//!
//! Impressed, the Elves issue you a challenge: determine the 30000000th number spoken. For example, given the same starting numbers as above:
//!
//! ```
//! Given 0,3,6, the 30000000th number spoken is 175594.
//! Given 1,3,2, the 30000000th number spoken is 2578.
//! Given 2,1,3, the 30000000th number spoken is 3544142.
//! Given 1,2,3, the 30000000th number spoken is 261214.
//! Given 2,3,1, the 30000000th number spoken is 6895259.
//! Given 3,2,1, the 30000000th number spoken is 18.
//! Given 3,1,2, the 30000000th number spoken is 362.
//! ```
//!
//! Given your starting numbers, what will be the 30000000th number spoken?
//!
//! Your puzzle answer was 10652.

use std::collections::HashMap;
use std::fs;

fn main() {
    let input = parse_input("input.txt");
    let res_p1 = part1(&input, 2020);
    println!("res p1: {}", res_p1);

    let res_p2 = part1(&input, 30000000);
    println!("res p2: {}", res_p2);
}

/// parses the input into a vector of i32's
fn parse_input(filename: &str) -> Vec<i32> {
    fs::read_to_string(filename)
        .unwrap()
        .split(',')
        .map(|line| line.parse().unwrap())
        .collect()
}

#[allow(dead_code)]
/// a hashmap implementation of the memory game. this is slower but more
/// memory efficient than preallocating a vector/array and using that as a
/// cache. it requires costly hash calculations
fn part1_old(nums: &[i32], max_turn: i32) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut last = 0;

    // iterate over starting numbers
    for (i, &num) in nums.iter().enumerate() {
        let turn = i as i32;

        let entry = map.entry(num).or_insert(0);
        *entry = turn;
        last = num;
    }

    // calculate the number for each turn until max_turn
    for i in nums.len()..(max_turn as usize) {
        let turn = i as i32;
        let current = match map.get(&last) {
            Some(num) => turn - 1 - num,
            None => 0,
        };

        map.insert(last, turn - 1);
        last = current;
    }

    last
}

/// a preallocated vector implementation of the memory game. this one uses
/// up more memory since it preallocates 4x max_turn as a cache, however it
/// doesn't require costly hash calculations
fn part1(nums: &[i32], max_turn: i32) -> i32 {
    let mut cache = vec![0; max_turn as usize];
    let mut last = 0;

    // add starting numbers to cache
    for (i, num) in nums.iter().copied().enumerate() {
        cache[num as usize] = i + 1;
        last = num;
    }

    // calculate num for each of the next turns until max_turn
    for i in nums.len()..(max_turn as usize) {
        let current = match cache[last as usize] {
            0 => 0,
            turn => (i - turn) as i32,
        };

        cache[last as usize] = i;
        last = current;
    }

    last
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_p1() {
        let inputs: Vec<(Vec<i32>, i32)> = vec![
            (vec![0, 3, 6], 436),
            (vec![1, 3, 2], 1),
            (vec![2, 1, 3], 10),
            (vec![1, 2, 3], 27),
            (vec![2, 3, 1], 78),
            (vec![3, 2, 1], 438),
            (vec![3, 1, 2], 1836),
        ];

        for (input, answer) in inputs {
            assert_eq!(part1(&input, 2020), answer);
        }
    }

    #[test]
    fn test_sample_p2() {
        let inputs: Vec<(Vec<i32>, i32)> = vec![
            (vec![0, 3, 6], 175594),
            (vec![1, 3, 2], 2578),
            (vec![2, 1, 3], 3544142),
            (vec![1, 2, 3], 261214),
            (vec![2, 3, 1], 6895259),
            (vec![3, 2, 1], 18),
            (vec![3, 1, 2], 362),
        ];

        for (input, answer) in inputs {
            assert_eq!(part1(&input, 30000000), answer);
        }
    }
}
