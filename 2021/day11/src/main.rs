#![crate_name = "day11"]

use std::fs;

fn main() {
    let input = read_input("input.txt");
}

fn part1(input: &[Vec<usize>], steps: usize) -> usize {
    0
}

fn read_input(filename: &str) -> Vec<Vec<usize>> {
    fs::read_to_string(filename)
        .expect("Failed to read input file")
        .lines()
        .map(|line| line.chars().map(|c| c as usize).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input: Vec<Vec<usize>> = vec![
            "5483143223",
            "2745854711",
            "5264556173",
            "6141336146",
            "6357385478",
            "4167524645",
            "2176841721",
            "6882881134",
            "4846848554",
            "5283751526",
        ]
        .iter()
        .map(|line| line.chars().map(|c| c as usize).collect())
        .collect();

        assert_eq!(part1(&input, 100), 1656);
    }
}
