#![crate_name = "day03"]

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
    println!("Part1: {}", part1);
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
