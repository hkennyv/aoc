#![crate_name = "day09"]

use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let input = parse_input(&contents);
    println!("{}", input.len());

    let res_p1 = part1(&input);
    println!("res p1: {}", res_p1);
}

fn part1(input: &[i64]) -> i64 {
    let PREAMBLE_SIZE = 25;

    let windows = input.windows(PREAMBLE_SIZE);

    for (i, window) in windows.enumerate() {
        let value = input[i + PREAMBLE_SIZE - 1];
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

/// parses the input from a String to a Vec<i32>
fn parse_input(input: &str) -> Vec<i64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}
