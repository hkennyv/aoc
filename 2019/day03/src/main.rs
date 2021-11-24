#![crate_name = "day03"]

//! ## --- Day 3: Crossed Wires ---
//!
//! The gravity assist was successful, and you're well on your way to the Venus refuelling station. During the rush back on Earth, the fuel management system wasn't completely installed, so that's next on the priority list.
//!
//! Opening the front panel reveals a jumble of wires. Specifically, two wires are connected to a central port and extend outward on a grid. You trace the path each wire takes as it leaves the central port, one wire per line of text (your puzzle input).
//!
//! The wires twist and turn, but the two wires occasionally cross paths. To fix the circuit, you need to find the intersection point closest to the central port. Because the wires are on a grid, use the Manhattan distance for this measurement. While the wires do technically cross right at the central port where they both start, this point does not count, nor does a wire count as crossing with itself.
//!
//! For example, if the first wire's path is R8,U5,L5,D3, then starting from the central port (o), it goes right 8, up 5, left 5, and finally down 3:
//!
//! ```
//! ...........
//! ...........
//! ...........
//! ....+----+.
//! ....|....|.
//! ....|....|.
//! ....|....|.
//! .........|.
//! .o-------+.
//! ...........
//! ```
//!
//! Then, if the second wire's path is U7,R6,D4,L4, it goes up 7, right 6, down 4, and left 4:
//!
//! ```
//! ...........
//! .+-----+...
//! .|.....|...
//! .|..+--X-+.
//! .|..|..|.|.
//! .|.-X--+.|.
//! .|..|....|.
//! .|.......|.
//! .o-------+.
//! ...........
//! ```
//!
//! These wires cross at two locations (marked X), but the lower-left one is closer to the central port: its distance is 3 + 3 = 6.
//!
//! Here are a few more examples:
//!
//! ```
//! R75,D30,R83,U83,L12,D49,R71,U7,L72
//! U62,R66,U55,R34,D71,R55,D58,R83 = distance 159
//! R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
//! U98,R91,D20,R16,D67,R40,U7,R15,U6,R7 = distance 135
//! ```
//!
//! What is the Manhattan distance from the central port to the closest intersection?
//!
//! Your puzzle answer was 217.
//!
//! ## --- Part Two ---
//!
//! It turns out that this circuit is very timing-sensitive; you actually need to minimize the signal delay.
//!
//! To do this, calculate the number of steps each wire takes to reach each intersection; choose the intersection where the sum of both wires' steps is lowest. If a wire visits a position on the grid multiple times, use the steps value from the first time it visits that position when calculating the total value of a specific intersection.
//!
//! The number of steps a wire takes is the total number of grid squares the wire has entered to get to that location, including the intersection being considered. Again consider the example from above:
//!
//! ```
//! ...........
//! .+-----+...
//! .|.....|...
//! .|..+--X-+.
//! .|..|..|.|.
//! .|.-X--+.|.
//! .|..|....|.
//! .|.......|.
//! .o-------+.
//! ...........
//! ```
//!
//! In the above example, the intersection closest to the central port is reached after 8+5+5+2 = 20 steps by the first wire and 7+6+4+3 = 20 steps by the second wire for a total of 20+20 = 40 steps.
//!
//! However, the top-right intersection is better: the first wire takes only 8+5+2 = 15 and the second wire takes only 7+6+2 = 15, a total of 15+15 = 30 steps.
//!
//! Here are the best steps for the extra examples from above:
//!
//! ```
//! R75,D30,R83,U83,L12,D49,R71,U7,L72
//! U62,R66,U55,R34,D71,R55,D58,R83 = 610 steps
//! R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
//! U98,R91,D20,R16,D67,R40,U7,R15,U6,R7 = 410 steps
//! ```
//!
//! What is the fewest combined steps the wires must take to reach an intersection?
//!
//! Your puzzle answer was 3454.

use std::collections::HashMap;
use std::fs;

enum Direction {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

fn main() {
    let input = read_input("input.txt");

    let part1 = part1(&input);
    let part2 = part2(&input);

    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
}

fn part1(directions: &(Vec<String>, Vec<String>)) -> usize {
    let line1_directions: Vec<Direction> = directions
        .0
        .iter()
        .map(|s| s.as_str())
        .map(to_direction)
        .collect();
    let line2_directions: Vec<Direction> = directions
        .1
        .iter()
        .map(|s| s.as_str())
        .map(to_direction)
        .collect();

    let mut grid: HashMap<(i32, i32), usize> = HashMap::new();

    update_grid(&mut grid, &line1_directions);
    update_grid(&mut grid, &line2_directions);

    let intersections: Vec<&(i32, i32)> = grid.keys().filter(|&key| grid[key] > 1).collect();

    // return the minimum distance
    intersections
        .iter()
        .map(|&coord| manhattan_distance(&(0, 0), coord))
        .min()
        .expect("distances are empty!")
}

fn part2(directions: &(Vec<String>, Vec<String>)) -> usize {
    let line1_directions: Vec<Direction> = directions
        .0
        .iter()
        .map(|s| s.as_str())
        .map(to_direction)
        .collect();
    let line2_directions: Vec<Direction> = directions
        .1
        .iter()
        .map(|s| s.as_str())
        .map(to_direction)
        .collect();

    let mut grid: HashMap<(i32, i32), (Vec<usize>, Vec<usize>)> = HashMap::new();
    update_grid2(&mut grid, &line1_directions, 0);
    update_grid2(&mut grid, &line2_directions, 1);

    // an intersection will have a non-empty vector for each line
    let intersections: Vec<&(i32, i32)> = grid
        .keys()
        .filter(|&key| !grid[key].0.is_empty() && !grid[key].1.is_empty())
        .collect();

    // sum up the number of steps for each intersection and return the minimum
    intersections
        .iter()
        .map(|key| grid[key].0.iter().min().unwrap() + grid[key].1.iter().min().unwrap())
        .min()
        .unwrap()
}

fn read_input(input: &str) -> (Vec<String>, Vec<String>) {
    let string = fs::read_to_string(input).expect("Unable to read file");

    let lines = string
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.to_owned())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    assert_eq!(lines.len(), 2, "Expected 2 lines, check your input");

    (lines[0].clone(), lines[1].clone())
}

fn to_direction(direction: &str) -> Direction {
    let dir = direction.chars().next().unwrap();
    let value = direction[1..].parse::<usize>().unwrap();

    match dir {
        'U' => Direction::Up(value),
        'D' => Direction::Down(value),
        'L' => Direction::Left(value),
        'R' => Direction::Right(value),
        _ => panic!("Unknown direction"),
    }
}

fn update_grid(grid: &mut HashMap<(i32, i32), usize>, directions: &[Direction]) {
    let mut current: (i32, i32) = (0, 0);

    for direction in directions {
        match direction {
            Direction::Up(value) => {
                for _ in 0..*value {
                    // increment current
                    current.1 += 1;

                    // update grid
                    let counter = grid.entry(current).or_insert(0);
                    *counter += 1;
                }
            }
            Direction::Down(value) => {
                for _ in 0..*value {
                    // increment current
                    current.1 -= 1;

                    // update grid
                    let counter = grid.entry(current).or_insert(0);
                    *counter += 1;
                }
            }
            Direction::Left(value) => {
                for _ in 0..*value {
                    // increment current
                    current.0 -= 1;

                    // update grid
                    let counter = grid.entry(current).or_insert(0);
                    *counter += 1;
                }
            }
            Direction::Right(value) => {
                for _ in 0..*value {
                    // increment current
                    current.0 += 1;

                    // update grid
                    let counter = grid.entry(current).or_insert(0);
                    *counter += 1;
                }
            }
        }
    }
}

#[allow(clippy::type_complexity)]
fn update_grid2(
    grid: &mut HashMap<(i32, i32), (Vec<usize>, Vec<usize>)>,
    directions: &[Direction],
    line_num: usize,
) {
    let mut current: (i32, i32) = (0, 0);
    let mut steps: usize = 0;

    for direction in directions {
        match direction {
            Direction::Up(value) => {
                for _ in 0..*value {
                    // increment current and steps
                    current.1 += 1;
                    steps += 1;

                    // update grid
                    let counter = grid.entry(current).or_insert((Vec::new(), Vec::new()));

                    if line_num == 0 {
                        counter.0.push(steps);
                    } else {
                        counter.1.push(steps);
                    }
                }
            }
            Direction::Down(value) => {
                for _ in 0..*value {
                    // increment current and steps
                    current.1 -= 1;
                    steps += 1;

                    // update grid
                    let counter = grid.entry(current).or_insert((Vec::new(), Vec::new()));
                    if line_num == 0 {
                        counter.0.push(steps);
                    } else {
                        counter.1.push(steps);
                    }
                }
            }
            Direction::Left(value) => {
                for _ in 0..*value {
                    // increment current and steps
                    current.0 -= 1;
                    steps += 1;

                    // update grid
                    let counter = grid.entry(current).or_insert((Vec::new(), Vec::new()));
                    if line_num == 0 {
                        counter.0.push(steps);
                    } else {
                        counter.1.push(steps);
                    }
                }
            }
            Direction::Right(value) => {
                for _ in 0..*value {
                    // increment current and steps
                    current.0 += 1;
                    steps += 1;

                    // update grid
                    let counter = grid.entry(current).or_insert((Vec::new(), Vec::new()));
                    if line_num == 0 {
                        counter.0.push(steps);
                    } else {
                        counter.1.push(steps);
                    }
                }
            }
        }
    }
}

fn manhattan_distance(p1: &(i32, i32), p2: &(i32, i32)) -> usize {
    let (x1, y1) = p1;
    let (x2, y2) = p2;

    (x1 - x2).abs() as usize + (y1 - y2).abs() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_part1() {
        let directions: Vec<(Vec<String>, Vec<String>)> = vec![
            (
                vec![
                    "R75".into(),
                    "D30".into(),
                    "R83".into(),
                    "U83".into(),
                    "L12".into(),
                    "D49".into(),
                    "R71".into(),
                    "U7".into(),
                    "L72".into(),
                ],
                vec![
                    "U62".into(),
                    "R66".into(),
                    "U55".into(),
                    "R34".into(),
                    "D71".into(),
                    "R55".into(),
                    "D58".into(),
                    "R83".into(),
                ],
            ),
            (
                vec![
                    "R98".into(),
                    "U47".into(),
                    "R26".into(),
                    "D63".into(),
                    "R33".into(),
                    "U87".into(),
                    "L62".into(),
                    "D20".into(),
                    "R33".into(),
                    "U53".into(),
                    "R51".into(),
                ],
                vec![
                    "U98".into(),
                    "R91".into(),
                    "D20".into(),
                    "R16".into(),
                    "D67".into(),
                    "R40".into(),
                    "U7".into(),
                    "R15".into(),
                    "U6".into(),
                    "R7".into(),
                ],
            ),
        ];

        let expected = vec![159, 135];

        for (i, (line1, line2)) in directions.iter().enumerate() {
            let result = part1(&(line1.clone(), line2.clone()));
            assert_eq!(result, expected[i]);
        }
    }

    #[test]
    fn test_manhattan_distance() {
        #[allow(clippy::type_complexity)]
        let points: Vec<((i32, i32), (i32, i32), usize)> = vec![
            ((0, 0), (0, 0), 0),
            ((0, 0), (1, 0), 1),
            ((0, 0), (0, 1), 1),
            ((0, 0), (-1, 0), 1),
            ((0, 0), (0, -1), 1),
            ((0, 0), (1, 1), 2),
            ((0, 0), (-1, -1), 2),
            ((0, 0), (-1, 1), 2),
            ((0, 0), (1, -1), 2),
            ((0, 0), (2, 0), 2),
            ((0, 0), (0, 2), 2),
            ((0, 0), (-2, 0), 2),
            ((0, 0), (0, -2), 2),
            ((0, 0), (2, 2), 4),
            ((0, 0), (-2, -2), 4),
            ((0, 0), (2, -2), 4),
            ((0, 0), (-2, 2), 4),
        ];

        for (p1, p2, expected) in points {
            assert_eq!(manhattan_distance(&p1, &p2), expected);
        }
    }
}
