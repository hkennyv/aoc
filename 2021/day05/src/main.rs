#![crate_name = "day05"]

//! ## --- Day 5: Hydrothermal Venture ---
//!
//! You come across a field of hydrothermal vents on the ocean floor! These vents constantly produce large, opaque clouds, so it would be best to avoid them if possible.
//!
//! They tend to form in lines; the submarine helpfully produces a list of nearby lines of vents (your puzzle input) for you to review. For example:
//!
//! ```
//! 0,9 -> 5,9
//! 8,0 -> 0,8
//! 9,4 -> 3,4
//! 2,2 -> 2,1
//! 7,0 -> 7,4
//! 6,4 -> 2,0
//! 0,9 -> 2,9
//! 3,4 -> 1,4
//! 0,0 -> 8,8
//! 5,5 -> 8,2
//! ```
//!
//! Each line of vents is given as a line segment in the format x1,y1 -> x2,y2 where x1,y1 are the coordinates of one end the line segment and x2,y2 are the coordinates of the other end. These line segments include the points at both ends. In other words:
//!
//! - An entry like 1,1 -> 1,3 covers points 1,1, 1,2, and 1,3.
//! - An entry like 9,7 -> 7,7 covers points 9,7, 8,7, and 7,7.
//!
//! For now, only consider horizontal and vertical lines: lines where either x1 = x2 or y1 = y2.
//!
//! So, the horizontal and vertical lines from the above list would produce the following diagram:
//!
//! ```
//! .......1..
//! ..1....1..
//! ..1....1..
//! .......1..
//! .112111211
//! ..........
//! ..........
//! ..........
//! ..........
//! 222111....
//! ```
//!
//! In this diagram, the top left corner is 0,0 and the bottom right corner is 9,9. Each position is shown as the number of lines which cover that point or . if no line covers that point. The top-left pair of 1s, for example, comes from 2,2 -> 2,1; the very bottom row is formed by the overlapping lines 0,9 -> 5,9 and 0,9 -> 2,9.
//!
//! To avoid the most dangerous areas, you need to determine the number of points where at least two lines overlap. In the above example, this is anywhere in the diagram with a 2 or larger - a total of 5 points.
//!
//! Consider only horizontal and vertical lines. At how many points do at least two lines overlap?
//!
//! Your puzzle answer was 6564.
//!
//! ## --- Part Two ---
//!
//! Unfortunately, considering only horizontal and vertical lines doesn't give you the full picture; you need to also consider diagonal lines.
//!
//! Because of the limits of the hydrothermal vent mapping system, the lines in your list will only ever be horizontal, vertical, or a diagonal line at exactly 45 degrees. In other words:
//!
//! - An entry like 1,1 -> 3,3 covers points 1,1, 2,2, and 3,3.
//! - An entry like 9,7 -> 7,9 covers points 9,7, 8,8, and 7,9.
//!
//! Considering all lines from the above example would now produce the following diagram:
//!
//! ```
//! 1.1....11.
//! .111...2..
//! ..2.1.111.
//! ...1.2.2..
//! .112313211
//! ...1.2....
//! ..1...1...
//! .1.....1..
//! 1.......1.
//! 222111....
//! ```
//!
//! You still need to determine the number of points where at least two lines overlap. In the above example, this is still anywhere in the diagram with a 2 or larger - now a total of 12 points.
//!
//! Consider all of the lines. At how many points do at least two lines overlap?
//!
//! Your puzzle answer was 19172.

use std::collections::HashMap;
use std::fs;

type Coordinate = (usize, usize);
type Line = (Coordinate, Coordinate);

fn main() {
    let input = read_input("input.txt");
    let part1 = count_overlaps(&input, false);
    let part2 = count_overlaps(&input, true);

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}

fn count_overlaps(lines: &[Line], include_diagonals: bool) -> usize {
    let mut points: HashMap<Coordinate, usize> = HashMap::new();

    for (start, end) in lines {
        // horizontal line
        if start.0 == end.0 {
            let min = std::cmp::min(start.1, end.1);
            let max = std::cmp::max(start.1, end.1);

            for n in min..max + 1 {
                let counter = points.entry((start.0, n)).or_insert(0);
                *counter += 1;
            }
        }
        // vertical line
        else if start.1 == end.1 {
            let min = std::cmp::min(start.0, end.0);
            let max = std::cmp::max(start.0, end.0);

            for n in min..max + 1 {
                let counter = points.entry((n, start.1)).or_insert(0);
                *counter += 1;
            }
        }
        // diagonal line - moves from the starting point to the ending point
        else {
            if !include_diagonals {
                continue;
            };

            let (mut x1, mut y1) = start;

            while x1 != end.0 && y1 != end.1 {
                let counter = points.entry((x1, y1)).or_insert(0);
                *counter += 1;

                // line goes up + right
                if x1 < end.0 && y1 < end.1 {
                    x1 += 1;
                    y1 += 1;
                // line goes down + left
                } else if x1 > end.0 && y1 > end.1 {
                    x1 -= 1;
                    y1 -= 1;
                // line goes down + right
                } else if x1 < end.0 && y1 > end.1 {
                    x1 += 1;
                    y1 -= 1;
                // line goes up + left
                } else if x1 > end.0 && y1 < end.1 {
                    x1 -= 1;
                    y1 += 1;
                }
            }

            // don't forget to include the end point
            let counter = points.entry((x1, y1)).or_insert(0);
            *counter += 1;
        }
    }

    points.values().filter(|&&val| val > 1).count()
}

/// I'm so sorry, this is so ugly...
fn read_input(filename: &str) -> Vec<Line> {
    fs::read_to_string(filename)
        .expect("Failed to read input file")
        .lines()
        .map(|line| {
            let mut split = line.split("->").map(|pair| {
                let s = pair.trim().to_string();
                let mut nums = s.split(',').map(|num| num.parse::<usize>().unwrap());

                // (usize, usize) -- Coordinate
                (nums.next().unwrap(), nums.next().unwrap())
            });

            // ((usize, usize), (usize, usize)) -- Line
            (split.next().unwrap(), split.next().unwrap())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let lines = vec![
            ((0, 9), (5, 9)),
            ((8, 0), (0, 8)),
            ((9, 4), (3, 4)),
            ((2, 2), (2, 1)),
            ((7, 0), (7, 4)),
            ((6, 4), (2, 0)),
            ((0, 9), (2, 9)),
            ((3, 4), (1, 4)),
            ((0, 0), (8, 8)),
            ((5, 5), (8, 2)),
        ];

        assert_eq!(count_overlaps(&lines, false), 5);
    }
    #[test]
    fn test_part2() {
        let lines = vec![
            ((0, 9), (5, 9)),
            ((8, 0), (0, 8)),
            ((9, 4), (3, 4)),
            ((2, 2), (2, 1)),
            ((7, 0), (7, 4)),
            ((6, 4), (2, 0)),
            ((0, 9), (2, 9)),
            ((3, 4), (1, 4)),
            ((0, 0), (8, 8)),
            ((5, 5), (8, 2)),
        ];

        assert_eq!(count_overlaps(&lines, true), 12);
    }
}
