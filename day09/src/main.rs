#![crate_name = "day09"]

use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
}

/// parses the input from a String to a Vec<i32>
fn parse_input(input: &str) -> Vec<i32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}
