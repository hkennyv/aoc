#![crate_name = "day03"]

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
}

fn part1(directions: &(Vec<String>, Vec<String>)) -> usize {
    let line1 = to_coordinates(&directions.0);
    let line2 = to_coordinates(&directions.1);

    let intersections = get_intersections(&line1, &line2);

    0
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

fn to_coordinates(directions: &[String]) -> Vec<(i32, i32)> {
    let mut current: (i32, i32) = (0, 0);
    let mut coordinates: Vec<(i32, i32)> = vec![current];

    for string in directions {
        let direction = to_direction(string);

        match direction {
            Direction::Up(value) => current.1 += value as i32,
            Direction::Down(value) => current.1 -= value as i32,
            Direction::Left(value) => current.0 -= value as i32,
            Direction::Right(value) => current.0 += value as i32,
        }

        coordinates.push(current);
    }

    coordinates
}

fn lines_intersect(
    p11: &(i32, i32),
    p12: &(i32, i32),
    p21: &(i32, i32),
    p22: &(i32, i32),
) -> Option<(i32, i32)> {
    let (x1, y1) = p11;
    let (x2, y2) = p12;

    let (x3, y3) = p21;
    let (x4, y4) = p22;

    // check if lines are parallel
    if x1 == x2 && x3 == x4 && x1 != x3 || y1 == y2 && y3 == y4 && y1 != y3 {
        return None;
    }

    // line 1 is a vertical line, so check line2 for a horzontal intersection
    if x1 == x2 && std::cmp::min(x3, x4) <= x1 && std::cmp::max(x3, x4) >= x1 {
        return Some((*x1, *y3));
    }

    // line 1 is a horizontal line, so check line2 for a vertical intersection
    if y1 == y2 && std::cmp::min(y3, y4) <= y1 && std::cmp::max(y3, y4) >= y1 {
        return Some((*x3, *y1));
    }

    None
}

fn get_intersections(line1: &[(i32, i32)], line2: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let mut intersections: Vec<(i32, i32)> = vec![];

    for i in 1..line1.len() {
        let l11: (i32, i32) = line1[i];
        let l12: (i32, i32) = line1[i - 1];

        for j in 1..line2.len() {
            let l21: (i32, i32) = line2[j];
            let l22: (i32, i32) = line2[j - 1];

            if let Some(intersection) = lines_intersect(&l11, &l12, &l21, &l22) {
                println!(
                    "intersection! {:?} {:?} {:?} {:?} x{:?}",
                    l11, l12, l21, l22, intersection
                );
                intersections.push((l11.0, l11.1));
            }
        }
    }

    intersections
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let directions: Vec<(Vec<String>, Vec<String>)> = vec![
            (
                vec!["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"]
                    .iter()
                    .map(|s| s.to_owned())
                    .collect(),
                vec!["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"]
                    .iter()
                    .map(|s| s.to_owned())
                    .collect(),
            ),
            (
                vec![
                    "R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51",
                ]
                .iter()
                .map(|s| s.to_owned()),
                vec![
                    "U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7",
                ]
                .iter()
                .map(|s| s.to_owned()),
            ),
        ];

        let expected = vec![159, 135];
    }
}
