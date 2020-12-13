#![crate_name = "day12"]

use std::fs;

mod ship;
use ship::Ship;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let input: Vec<(char, i32)> = contents
        .lines()
        .map(|line| (line.chars().next().unwrap(), line[1..].parse().unwrap()))
        .collect();

    let res_p1 = part1(&input);
    println!("res p1: {}", res_p1);

    let res_p1 = part2(&input);
    println!("res p2: {}", res_p1);
}

fn part1(input: &[(char, i32)]) -> i32 {
    let mut ship = Ship::new();

    for (action, value) in input {
        ship.handle_action_p1(*action, *value);
    }

    ship.get_manhattan_distance()
}

fn part2(input: &[(char, i32)]) -> i32 {
    let mut ship = Ship::new();

    for (action, value) in input {
        ship.handle_action_p2(*action, *value);
    }

    ship.get_manhattan_distance()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_p1() {
        let contents = fs::read_to_string("sample.txt").unwrap();
        let input: Vec<(char, i32)> = contents
            .lines()
            .map(|line| (line.chars().next().unwrap(), line[1..].parse().unwrap()))
            .collect();

        assert_eq!(part1(&input), 25);
    }

    #[test]
    #[ignore]
    fn test_sample_p2() {
        let contents = fs::read_to_string("sample.txt").unwrap();
        let input: Vec<(char, i32)> = contents
            .lines()
            .map(|line| (line.chars().next().unwrap(), line[1..].parse().unwrap()))
            .collect();

        assert_eq!(part2(&input), 25);
    }
}
