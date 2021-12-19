#![crate_name = "day09"]

//! ## --- Day 9: Smoke Basin ---
//!
//! These caves seem to be lava tubes. Parts are even still volcanically active; small hydrothermal vents release smoke into the caves that slowly settles like rain.
//!
//! If you can model how the smoke flows through the caves, you might be able to avoid it and be that much safer. The submarine generates a heightmap of the floor of the nearby caves for you (your puzzle input).
//!
//! Smoke flows to the lowest point of the area it's in. For example, consider the following heightmap:
//!
//! ```
//! 2199943210
//! 3987894921
//! 9856789892
//! 8767896789
//! 9899965678
//! ```
//!
//! Each number corresponds to the height of a particular location, where 9 is the highest and 0 is the lowest a location can be.
//!
//! Your first goal is to find the low points - the locations that are lower than any of its adjacent locations. Most locations have four adjacent locations (up, down, left, and right); locations on the edge or corner of the map have three or two adjacent locations, respectively. (Diagonal locations do not count as adjacent.)
//!
//! In the above example, there are four low points, all highlighted: two are in the first row (a 1 and a 0), one is in the third row (a 5), and one is in the bottom row (also a 5). All other locations on the heightmap have some lower adjacent location, and so are not low points.
//!
//! The risk level of a low point is 1 plus its height. In the above example, the risk levels of the low points are 2, 1, 6, and 6. The sum of the risk levels of all low points in the heightmap is therefore 15.
//!
//! Find all of the low points on your heightmap. What is the sum of the risk levels of all low points on your heightmap?
//!
//! Your puzzle answer was 502.
//!
//! ## --- Part Two ---
//!
//! Next, you need to find the largest basins so you know what areas are most important to avoid.
//!
//! A basin is all locations that eventually flow downward to a single low point. Therefore, every low point has a basin, although some basins are very small. Locations of height 9 do not count as being in any basin, and all other locations will always be part of exactly one basin.
//!
//! The size of a basin is the number of locations within the basin, including the low point. The example above has four basins.
//!
//! The top-left basin, size 3:
//!
//! ```
//! 2199943210
//! 3987894921
//! 9856789892
//! 8767896789
//! 9899965678
//! ```
//!
//! The top-right basin, size 9:
//!
//! ```
//! 2199943210
//! 3987894921
//! 9856789892
//! 8767896789
//! 9899965678
//! ```
//!
//! The middle basin, size 14:
//!
//! ```
//! 2199943210
//! 3987894921
//! 9856789892
//! 8767896789
//! 9899965678
//! ```
//!
//! The bottom-right basin, size 9:
//!
//! ```
//! 2199943210
//! 3987894921
//! 9856789892
//! 8767896789
//! 9899965678
//! ```
//!
//! Find the three largest basins and multiply their sizes together. In the above example, this is 9 _ 14 _ 9 = 1134.
//!
//! What do you get if you multiply together the sizes of the three largest basins?
//!
//! Your puzzle answer was 1330560.

use std::collections::HashSet;
use std::fs;

fn main() {
    let input = read_input("input.txt");
    let part1 = part1(&input);
    let part2 = part2(&input);

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}

fn part1(input: &[Vec<usize>]) -> usize {
    find_minimums(input)
        .iter()
        .map(|minimum| input[minimum.0][minimum.1] + 1)
        .sum()
}

fn part2(input: &[Vec<usize>]) -> usize {
    let (rows, cols) = (input.len(), input[0].len());
    let minimums = find_minimums(input);

    let mut basins: Vec<usize> = Vec::new();

    for minimum in minimums {
        let mut seen: HashSet<(usize, usize)> = HashSet::new();
        basins.push(get_basin(
            input, &mut seen, minimum.0, minimum.1, rows, cols,
        ));
    }

    basins.sort_unstable();
    basins.reverse();

    basins[0] * basins[1] * basins[2]
}

fn get_basin(
    input: &[Vec<usize>],
    seen: &mut HashSet<(usize, usize)>,
    row: usize,
    col: usize,
    rows: usize,
    cols: usize,
) -> usize {
    let mut basin = 0;

    // base case
    if input[row][col] == 9 {
        seen.insert((row, col));
        return 0;
    }

    // add current cell
    seen.insert((row, col));
    basin += 1;

    // get clamped bounds
    let ((start_x, end_x), (start_y, end_y)) = get_bounds(rows, cols, row, col);

    for m in start_y..=end_y {
        if !seen.contains(&(m, col)) {
            seen.insert((m, col));
            basin += get_basin(input, seen, m, col, rows, cols);
        }
    }

    for n in start_x..=end_x {
        if !seen.contains(&(row, n)) {
            seen.insert((row, n));
            basin += get_basin(input, seen, row, n, rows, cols);
        }
    }

    basin
}

fn find_minimums(input: &[Vec<usize>]) -> Vec<(usize, usize)> {
    let mut res = Vec::new();

    // brute force, check each cell
    let (rows, cols) = (input.len(), input[0].len());

    for i in 0..rows {
        for j in 0..cols {
            let mut is_minimum = true;
            let cell = input[i][j];

            // 9 cannot be a minimum, it is the highest point
            if cell == 9 {
                continue;
            }

            // get clamped bounds
            let ((start_x, end_x), (start_y, end_y)) = get_bounds(rows, cols, i, j);

            // check vertical neighbors
            #[allow(clippy::needless_range_loop)]
            for m in start_y..=end_y {
                if input[m][j] < cell {
                    is_minimum = false;
                    break;
                }
            }

            // check horizontal neighbors
            #[allow(clippy::needless_range_loop)]
            for n in start_x..=end_x {
                if input[i][n] < cell {
                    is_minimum = false;
                    break;
                }
            }

            if is_minimum {
                res.push((i, j));
            }
        }
    }

    res
}

/// Utility function that takes in the max rows and cols and returns two pairs
/// of (start, end) indicies for the rows and cols. Clamps the beginning to 0
/// if it is less than 0 and clamps the end to the max rows/cols if it is
/// greater than the max rows/cols.
fn get_bounds(
    max_rows: usize,
    max_cols: usize,
    row: usize,
    col: usize,
) -> ((usize, usize), (usize, usize)) {
    let start_x = if (col as i32) - 1i32 < 0 {
        col
    } else {
        col - 1
    };
    let end_x = if col + 1 == max_cols { col } else { col + 1 };

    let start_y = if (row as i32) - 1i32 < 0 {
        row
    } else {
        row - 1
    };
    let end_y = if row + 1 == max_rows { row } else { row + 1 };

    ((start_x, end_x), (start_y, end_y))
}

fn read_input(filename: &str) -> Vec<Vec<usize>> {
    fs::read_to_string(filename)
        .expect("Failed to read input file")
        .lines()
        .map(|line| {
            line.chars()
                // convert a string digit to a usize by subtracting '0' (48 decimal)
                // https://www.asciitable.com/
                .map(|d| (d as usize - '0' as usize) as usize)
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input: Vec<Vec<usize>> = vec![
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678",
        ]
        .iter()
        .map(|line| {
            line.chars()
                .map(|d| (d as usize - '0' as usize) as usize)
                .collect()
        })
        .collect();

        assert_eq!(part1(&input), 15);
    }

    #[test]
    fn test_part2() {
        let input: Vec<Vec<usize>> = vec![
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678",
        ]
        .iter()
        .map(|line| {
            line.chars()
                .map(|d| (d as usize - '0' as usize) as usize)
                .collect()
        })
        .collect();

        assert_eq!(part2(&input), 1134);
    }
}
