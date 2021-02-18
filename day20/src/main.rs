#![crate_name = "day20"]
#![allow(clippy::all)]

use std::collections::HashMap;
use std::fs;

mod tile;
use tile::Tile;

fn main() {
    let input = parse_input("input.txt");

    let res_p1 = part1(&input);
    println!("res p1: {}", res_p1);
}

/// parses the input file into a HashMap<i32, Tile> where the id is the the
/// key and the `Tile` type is the value
fn parse_input(filename: &str) -> HashMap<i32, Tile> {
    fs::read_to_string(filename)
        .unwrap()
        .split("\n\n")
        .map(|block| {
            let mut lines = block.lines();

            let id = lines
                .next()
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .strip_suffix(':')
                .unwrap()
                .parse::<i32>()
                .unwrap();

            let tile: Tile = lines.map(|line| line.chars().collect()).collect();

            (id, tile)
        })
        .collect::<HashMap<i32, Tile>>()
}

fn part1(tilemap: &HashMap<i32, Tile>) -> i64 {
    let length = tilemap.iter().next().unwrap().1.len();
    let width = tilemap.iter().next().unwrap().1[0].len();

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_p1() {
        let input = parse_input("sample.txt");
        assert_eq!(part1(&input), 20899048083289);
    }
}
