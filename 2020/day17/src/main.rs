#![crate_name = "day17"]
#![allow(clippy::all)]

//! ## --- Day 17: Conway Cubes ---
//!
//! As your flight slowly drifts through the sky, the Elves at the Mythical Information Bureau at the North Pole contact you. They'd like some help debugging a malfunctioning experimental energy source aboard one of their super-secret imaging satellites.
//!
//! The experimental energy source is based on cutting-edge technology: a set of Conway Cubes contained in a pocket dimension! When you hear it's having problems, you can't help but agree to take a look.
//!
//! The pocket dimension contains an infinite 3-dimensional grid. At every integer 3-dimensional coordinate (x,y,z), there exists a single cube which is either active or inactive.
//!
//! In the initial state of the pocket dimension, almost all cubes start inactive. The only exception to this is a small flat region of cubes (your puzzle input); the cubes in this region start in the specified active (#) or inactive (.) state.
//!
//! The energy source then proceeds to boot up by executing six cycles.
//!
//! Each cube only ever considers its neighbors: any of the 26 other cubes where any of their coordinates differ by at most 1. For example, given the cube at x=1,y=2,z=3, its neighbors include the cube at x=2,y=2,z=2, the cube at x=0,y=2,z=3, and so on.
//!
//! During a cycle, all cubes simultaneously change their state according to the following rules:
//!
//! If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains active. Otherwise, the cube becomes inactive.
//! If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active. Otherwise, the cube remains inactive.
//! The engineers responsible for this experimental energy source would like you to simulate the pocket dimension and determine what the configuration of cubes should be at the end of the six-cycle boot process.
//!
//! For example, consider the following initial state:
//!
//! ```
//! .#.
//! ..#
//! ###
//! ```
//!
//! Even though the pocket dimension is 3-dimensional, this initial state represents a small 2-dimensional slice of it. (In particular, this initial state defines a 3x3x1 region of the 3-dimensional space.)
//!
//! Simulating a few cycles from this initial state produces the following configurations, where the result of each cycle is shown layer-by-layer at each given z coordinate (and the frame of view follows the active cells in each cycle):
//!
//! ```
//! Before any cycles:
//!
//! z=0
//! .#.
//! ..#
//! ###
//!
//!
//! After 1 cycle:
//!
//! z=-1
//! #..
//! ..#
//! .#.
//!
//! z=0
//! #.#
//! .##
//! .#.
//!
//! z=1
//! #..
//! ..#
//! .#.
//!
//!
//! After 2 cycles:
//!
//! z=-2
//! .....
//! .....
//! ..#..
//! .....
//! .....
//!
//! z=-1
//! ..#..
//! .#..#
//! ....#
//! .#...
//! .....
//!
//! z=0
//! ##...
//! ##...
//! #....
//! ....#
//! .###.
//!
//! z=1
//! ..#..
//! .#..#
//! ....#
//! .#...
//! .....
//!
//! z=2
//! .....
//! .....
//! ..#..
//! .....
//! .....
//!
//!
//! After 3 cycles:
//!
//! z=-2
//! .......
//! .......
//! ..##...
//! ..###..
//! .......
//! .......
//! .......
//!
//! z=-1
//! ..#....
//! ...#...
//! #......
//! .....##
//! .#...#.
//! ..#.#..
//! ...#...
//!
//! z=0
//! ...#...
//! .......
//! #......
//! .......
//! .....##
//! .##.#..
//! ...#...
//!
//! z=1
//! ..#....
//! ...#...
//! #......
//! .....##
//! .#...#.
//! ..#.#..
//! ...#...
//!
//! z=2
//! .......
//! .......
//! ..##...
//! ..###..
//! .......
//! .......
//! .......
//! ```
//!
//! After the full six-cycle boot process completes, 112 cubes are left in the active state.
//!
//! Starting with your given initial configuration, simulate six cycles. How many cubes are left in the active state after the sixth cycle?
//!
//! Your puzzle answer was 338.
//!
//! ## --- Part Two ---
//!
//! For some reason, your simulated results don't match what the experimental energy source engineers expected. Apparently, the pocket dimension actually has four spatial dimensions, not three.
//!
//! The pocket dimension contains an infinite 4-dimensional grid. At every integer 4-dimensional coordinate (x,y,z,w), there exists a single cube (really, a hypercube) which is still either active or inactive.
//!
//! Each cube only ever considers its neighbors: any of the 80 other cubes where any of their coordinates differ by at most 1. For example, given the cube at x=1,y=2,z=3,w=4, its neighbors include the cube at x=2,y=2,z=3,w=3, the cube at x=0,y=2,z=3,w=4, and so on.
//!
//! The initial state of the pocket dimension still consists of a small flat region of cubes. Furthermore, the same rules for cycle updating still apply: during each cycle, consider the number of active neighbors of each cube.
//!
//! For example, consider the same initial state as in the example above. Even though the pocket dimension is 4-dimensional, this initial state represents a small 2-dimensional slice of it. (In particular, this initial state defines a 3x3x1x1 region of the 4-dimensional space.)
//!
//! Simulating a few cycles from this initial state produces the following configurations, where the result of each cycle is shown layer-by-layer at each given z and w coordinate:
//!
//! ```
//! Before any cycles:
//!
//! z=0, w=0
//! .#.
//! ..#
//! ###
//!
//!
//! After 1 cycle:
//!
//! z=-1, w=-1
//! #..
//! ..#
//! .#.
//!
//! z=0, w=-1
//! #..
//! ..#
//! .#.
//!
//! z=1, w=-1
//! #..
//! ..#
//! .#.
//!
//! z=-1, w=0
//! #..
//! ..#
//! .#.
//!
//! z=0, w=0
//! #.#
//! .##
//! .#.
//!
//! z=1, w=0
//! #..
//! ..#
//! .#.
//!
//! z=-1, w=1
//! #..
//! ..#
//! .#.
//!
//! z=0, w=1
//! #..
//! ..#
//! .#.
//!
//! z=1, w=1
//! #..
//! ..#
//! .#.
//!
//!
//! After 2 cycles:
//!
//! z=-2, w=-2
//! .....
//! .....
//! ..#..
//! .....
//! .....
//!
//! z=-1, w=-2
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=0, w=-2
//! ###..
//! ##.##
//! #...#
//! .#..#
//! .###.
//!
//! z=1, w=-2
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=2, w=-2
//! .....
//! .....
//! ..#..
//! .....
//! .....
//!
//! z=-2, w=-1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=-1, w=-1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=0, w=-1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=1, w=-1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=2, w=-1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=-2, w=0
//! ###..
//! ##.##
//! #...#
//! .#..#
//! .###.
//!
//! z=-1, w=0
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=0, w=0
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=1, w=0
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=2, w=0
//! ###..
//! ##.##
//! #...#
//! .#..#
//! .###.
//!
//! z=-2, w=1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=-1, w=1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=0, w=1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=1, w=1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=2, w=1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=-2, w=2
//! .....
//! .....
//! ..#..
//! .....
//! .....
//!
//! z=-1, w=2
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=0, w=2
//! ###..
//! ##.##
//! #...#
//! .#..#
//! .###.
//!
//! z=1, w=2
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=2, w=2
//! .....
//! .....
//! ..#..
//! .....
//! .....
//! ```
//!
//! After the full six-cycle boot process completes, 848 cubes are left in the active state.
//!
//! Starting with your given initial configuration, simulate six cycles in a 4-dimensional space. How many cubes are left in the active state after the sixth cycle?
//!
//! Your puzzle answer was 2440.

use std::fs;

/// 1-D type
type Line = Vec<bool>;
type Layer = Vec<Line>;
type Grid = Vec<Layer>;
type Grid4d = Vec<Grid>;

fn main() {
    let input = parse_input("input.txt");

    let res_p1 = part1(&input, 3, 6);
    println!("res p1: {}", res_p1);

    let res_p2 = part2(&input, 4, 6);
    println!("res p2: {}", res_p2);
}

fn parse_input(filename: &str) -> Layer {
    let contents = fs::read_to_string(filename).unwrap();
    let slice: Vec<Vec<bool>> = contents
        .lines()
        .map(|line| line.chars().map(|ch| matches!(ch, '#')).collect())
        .collect();

    slice
}

/// simulates the cube growing for ndim = 3
fn part1(layer: &Layer, ndim: usize, cycles: usize) -> i32 {
    // since it is a cube, the length == width
    let length = layer.len();

    // the max length is the maximum potential size that the cube can grow to
    // given the # of cycles. this way, we can preallocate all the memory
    let max_length = length + (cycles * (ndim - 1));

    // initialize grid dimensions based on cycles and ndim
    // and copy the initial layer into the grid where the initial layer should
    // start at (max_length/2 - length/2)
    let mut grid: Grid = vec![vec![vec![false; max_length]; max_length]; max_length];

    {
        let starting_point = max_length / 2 - length / 2;
        for x in starting_point..(starting_point + length) {
            for y in starting_point..(starting_point + length) {
                let i = x - starting_point;
                let j = y - starting_point;
                grid[max_length / 2][x][y] = layer[i][j];
            }
        }
    }

    for _ in 0..cycles {
        simulate_cycle(&mut grid);
    }

    count_active(&grid)
}

/// simulates the cube growing for ndim = 4
fn part2(layer: &Layer, ndim: usize, cycles: usize) -> i32 {
    // since it is a cube, the length == width
    let length = layer.len();

    // the max length is the maximum potential size that the cube can grow to
    // given the # of cycles. this way, we can preallocate all the memory
    let max_length = length + (cycles * (ndim - 1));

    // initialize grid dimensions based on cycles and ndim
    // and copy the initial layer into the grid where the initial layer should
    // start at (max_length/2 - length/2)
    let mut grid: Grid4d =
        vec![vec![vec![vec![false; max_length]; max_length]; max_length]; max_length];

    {
        let starting_point = max_length / 2 - length / 2;

        for x in starting_point..(starting_point + length) {
            for y in starting_point..(starting_point + length) {
                let i = x - starting_point;
                let j = y - starting_point;
                grid[max_length / 2][max_length / 2][x][y] = layer[i][j];
            }
        }
    }

    for _ in 0..cycles {
        simulate_cycle4d(&mut grid);
    }

    count_active4d(&grid)
}

fn simulate_cycle4d(grid: &mut Grid4d) {
    let mut to_active: Vec<(usize, usize, usize, usize)> = Vec::new();
    let mut to_inactive: Vec<(usize, usize, usize, usize)> = Vec::new();

    for w in 0..grid.len() {
        for z in 0..grid.len() {
            for x in 0..grid.len() {
                for y in 0..grid.len() {
                    let point = grid[w][z][x][y];

                    let n_active_neighbors = get_active_neighbors4d(grid, w, z, x, y);

                    // condition 1
                    if point {
                        if !(n_active_neighbors == 2 || n_active_neighbors == 3) {
                            to_inactive.push((w, z, x, y));
                        }
                    // condition 2
                    } else {
                        if n_active_neighbors == 3 {
                            to_active.push((w, z, x, y));
                        }
                    }
                }
            }
        }
    }

    for point in to_active {
        grid[point.0][point.1][point.2][point.3] = true;
    }

    for point in to_inactive {
        grid[point.0][point.1][point.2][point.3] = false;
    }
}

fn simulate_cycle(grid: &mut Grid) {
    let mut to_active: Vec<(usize, usize, usize)> = Vec::new();
    let mut to_inactive: Vec<(usize, usize, usize)> = Vec::new();

    for z in 0..grid.len() {
        for x in 0..grid.len() {
            for y in 0..grid.len() {
                let point = grid[z][x][y];

                let n_active_neighbors = get_active_neighbors(grid, z, x, y);

                // condition 1
                if point {
                    if !(n_active_neighbors == 2 || n_active_neighbors == 3) {
                        to_inactive.push((z, x, y));
                    }
                // condition 2
                } else {
                    if n_active_neighbors == 3 {
                        to_active.push((z, x, y));
                    }
                }
            }
        }
    }

    for point in to_active {
        grid[point.0][point.1][point.2] = true;
    }

    for point in to_inactive {
        grid[point.0][point.1][point.2] = false;
    }
}
fn get_active_neighbors(grid: &Grid, z: usize, x: usize, y: usize) -> i32 {
    let mut count = 0;
    let length = grid.len() as i32;

    for i in -1..=1i32 {
        for j in -1..=1i32 {
            for l in -1..=1i32 {
                let (z2, x2, y2) = (z as i32 + i, x as i32 + j, y as i32 + l);
                if !(i == 0 && j == 0 && l == 0)
                    && !(z2 >= length || x2 >= length || y2 >= length)
                    && !(z2 < 0 || x2 < 0 || y2 < 0)
                {
                    if grid[z2 as usize][x2 as usize][y2 as usize] {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn get_active_neighbors4d(grid: &Grid4d, w: usize, z: usize, x: usize, y: usize) -> i32 {
    let mut count = 0;
    let length = grid.len() as i32;

    for i in -1..=1i32 {
        for j in -1..=1i32 {
            for l in -1..=1i32 {
                for m in -1..=1i32 {
                    let (w2, z2, x2, y2) = (w as i32 + i, z as i32 + j, x as i32 + l, y as i32 + m);
                    if !(i == 0 && j == 0 && l == 0 && m == 0)
                        && !(w2 >= length || z2 >= length || x2 >= length || y2 >= length)
                        && !(w2 < 0 || z2 < 0 || x2 < 0 || y2 < 0)
                    {
                        if grid[w2 as usize][z2 as usize][x2 as usize][y2 as usize] {
                            count += 1;
                        }
                    }
                }
            }
        }
    }

    count
}

fn count_active(grid: &Grid) -> i32 {
    let length = grid.len();
    let mut count = 0;

    for z in 0..length {
        for x in 0..length {
            for y in 0..length {
                if grid[z][x][y] {
                    count += 1;
                }
            }
        }
    }

    count
}

fn count_active4d(grid: &Grid4d) -> i32 {
    let length = grid.len();
    let mut count = 0;

    for w in 0..length {
        for z in 0..length {
            for x in 0..length {
                for y in 0..length {
                    if grid[w][z][x][y] {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_p1() {
        let input = parse_input("sample.txt");
        assert_eq!(part1(&input, 3, 6), 112);
    }

    #[test]
    fn test_sample_p2() {
        let input = parse_input("sample.txt");
        assert_eq!(part2(&input, 4, 6), 848);
    }

    #[test]
    fn test_get_active_neighbors() {
        let grid: Grid = vec![
            vec![
                vec![true, false, false],
                vec![false, false, true],
                vec![false, true, false],
            ],
            vec![
                vec![true, false, true],
                vec![false, true, true],
                vec![false, true, false],
            ],
            vec![
                vec![true, false, false],
                vec![false, false, true],
                vec![false, true, false],
            ],
        ];

        assert_eq!(get_active_neighbors(&grid, 1, 1, 1), 10);
        assert_eq!(get_active_neighbors(&grid, 0, 1, 1), 8);
        assert_eq!(get_active_neighbors(&grid, 0, 0, 1), 6);
        assert_eq!(get_active_neighbors(&grid, 2, 0, 1), 6);
        assert_eq!(get_active_neighbors(&grid, 2, 2, 2), 5);
    }
}
