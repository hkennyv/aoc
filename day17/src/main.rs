#![crate_name = "day17"]
#![allow(clippy::all)]
#![allow(warnings)]
use std::fs;

mod cube;

/// 1-D type
type Line = Vec<bool>;
type Layer = Vec<Line>;
type Grid = Vec<Layer>;

fn main() {
    let input = parse_input("input.txt");
    let res_p1 = part1(input, 3, 6);
}

fn parse_input(filename: &str) -> Layer {
    let contents = fs::read_to_string(filename).unwrap();
    let slice: Vec<Vec<bool>> = contents
        .lines()
        .map(|line| line.chars().map(|ch| matches!(ch, '#')).collect())
        .collect();

    slice
}

fn part1(layer: Layer, ndim: usize, cycles: i32) -> i32 {
    // since it is a cube, the length == width
    let length = layer.len();

    // initialize grid dimensions based on cycles and ndim

    0
}

fn part2() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_sample_p1() {
        let input = parse_input("sample.txt");
        assert_eq!(part1(input), 112);
    }
}
