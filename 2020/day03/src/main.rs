#![crate_name = "day03"]

//! ## --- Day 3: Toboggan Trajectory ---
//!
//! With the toboggan login problems resolved, you set off toward the airport. While travel by toboggan might be easy, it's certainly not safe: there's very minimal steering and the area is covered in trees. You'll need to see which angles will take you near the fewest trees.
//!
//! Due to the local geology, trees in this area only grow on exact integer coordinates in a grid. You make a map (your puzzle input) of the open squares (.) and trees (#) you can see. For example:
//!
//! ```
//! ..##.......
//! #...#...#..
//! .#....#..#.
//! ..#.#...#.#
//! .#...##..#.
//! ..#.##.....
//! .#.#.#....#
//! .#........#
//! #.##...#...
//! #...##....#
//! .#..#...#.#
//! ```
//!
//! These aren't the only trees, though; due to something you read about once involving arboreal genetics and biome stability, the same pattern repeats to the right many times:
//!
//! ```
//! ..##.........##.........##.........##.........##.........##.......  --->
//! #...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
//! .#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
//! ..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
//! .#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
//! ..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....  --->
//! .#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
//! .#........#.#........#.#........#.#........#.#........#.#........#
//! #.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
//! #...##....##...##....##...##....##...##....##...##....##...##....#
//! .#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
//! ```
//!
//! You start on the open square (.) in the top-left corner and need to reach the bottom (below the bottom-most row on your map).
//!
//! The toboggan can only follow a few specific slopes (you opted for a cheaper model that prefers rational numbers); start by counting all the trees you would encounter for the slope right 3, down 1:
//!
//! From your starting position at the top-left, check the position that is right 3 and down 1. Then, check the position that is right 3 and down 1 from there, and so on until you go past the bottom of the map.
//!
//! The locations you'd check in the above example are marked here with O where there was an open square and X where there was a tree:
//!
//! ```
//! ..##.........##.........##.........##.........##.........##.......  --->
//! #..O#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
//! .#....X..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
//! ..#.#...#O#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
//! .#...##..#..X...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
//! ..#.##.......#.X#.......#.##.......#.##.......#.##.......#.##.....  --->
//! .#.#.#....#.#.#.#.O..#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
//! .#........#.#........X.#........#.#........#.#........#.#........#
//! #.##...#...#.##...#...#.X#...#...#.##...#...#.##...#...#.##...#...
//! #...##....##...##....##...#X....##...##....##...##....##...##....#
//! .#..#...#.#.#..#...#.#.#..#...X.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
//! ```
//!
//! In this example, traversing the map using this slope would cause you to encounter 7 trees.
//!
//! Starting at the top-left corner of your map and following a slope of right 3 and down 1, how many trees would you encounter?
//!
//! Your puzzle answer was 159.
//!
//! ## --- Part Two ---
//!
//! Time to check the rest of the slopes - you need to minimize the probability of a sudden arboreal stop, after all.
//!
//! Determine the number of trees you would encounter if, for each of the following slopes, you start at the top-left corner and traverse the map all the way to the bottom:
//!
//! ```
//! Right 1, down 1.
//! Right 3, down 1. (This is the slope you already checked.)
//! Right 5, down 1.
//! Right 7, down 1.
//! Right 1, down 2.
//! ```
//!
//! In the above example, these slopes would find 2, 7, 3, 4, and 2 tree(s) respectively; multiplied together, these produce the answer 336.
//!
//! What do you get if you multiply together the number of trees encountered on each of the listed slopes?
//!
//! Your puzzle answer was 6419669520.

use std::fs;

fn main() {
    let contents: String = fs::read_to_string("input.txt").unwrap();
    let input: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.to_string().chars().collect())
        .collect();

    println!("{}", count_trees_p1(&input));
    println!("{}", count_trees_p2(&input));
}

/// performs the tree counting for part1
fn count_trees_p1(forest: &[Vec<char>]) -> i32 {
    count_trees(forest, 3, 1)
}

/// performs the tree counting for part2
fn count_trees_p2(forest: &[Vec<char>]) -> i64 {
    let res = vec![
        count_trees(forest, 1, 1).into(),
        count_trees(forest, 3, 1).into(),
        count_trees(forest, 5, 1).into(),
        count_trees(forest, 7, 1).into(),
        count_trees(forest, 1, 2).into(),
    ];

    res.iter().product()
}

/// counts the number of trees given a forest, a right_slope, and a left_slope
/// where the forest is a 2D vector of of length m x n where the trees are
/// represented by a '#' and the empty space is represented by a '.'
fn count_trees(forest: &[Vec<char>], right_slope: i32, left_slope: i32) -> i32 {
    let height = forest.len();
    let width = forest[0].len(); // assumes all rows are same length

    let mut count = 0;

    let mut i = 0;
    let mut j = 0;

    loop {
        if i >= height {
            break;
        }

        let ch: char = forest[i][j % width];

        if ch == '#' {
            count += 1;
        }

        j += right_slope as usize;
        i += left_slope as usize;
    }

    count
}
