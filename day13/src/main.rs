#![crate_name = "day13"]

use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();

    let (early_time, bus_ids) = parse_input(&content);
    let res_p1 = part1(early_time, &bus_ids);
    println!("p1 res: {}", res_p1);

    let bus_ids = parse_input2(&content);
    let res_p2 = part2(&bus_ids);
    println!("p2 res: {}", res_p2);
}

fn parse_input(contents: &str) -> (i64, Vec<i64>) {
    let input: Vec<String> = contents.lines().map(|line| line.to_string()).collect();
    let early_time: i64 = input[0].parse().unwrap();
    let bus_ids: Vec<i64> = input[1]
        .split(',')
        .filter(|item| item.parse::<i64>().is_ok())
        .map(|item| item.parse().unwrap())
        .collect();

    (early_time, bus_ids)
}

/// parses the input and returns a vector with the bus_ids, replaces 'x'
/// with 0's
fn parse_input2(contents: &str) -> Vec<i64> {
    let input: Vec<String> = contents.lines().map(|line| line.to_string()).collect();
    let bus_ids: Vec<i64> = input[1]
        .split(',')
        .map(|item| item.parse().unwrap_or(0))
        .collect();

    bus_ids
}

fn part1(early_time: i64, ids: &[i64]) -> i64 {
    let mut res: Vec<(i64, i64)> = Vec::new();

    // for each bus_id
    //   - take the greatest multiplier where bus_id * multiplier > early_time
    //   - take the id with the closest multiple to early_time
    //   - return the product of (bus_id * (closest_multiple - early_time))
    for id in ids {
        let mut multiplier = early_time / *id;
        let remainder = early_time % *id;

        if remainder == 0 {
            return *id * (*id * multiplier - early_time);
        }

        loop {
            multiplier += 1;
            let num = multiplier * *id;

            if num > early_time {
                res.push((num, multiplier));
                println!("{} {} {} target: {}", *id, multiplier, num, early_time);
                break;
            }
        }
    }

    let (num, multiplier) = *res.iter().min().unwrap();

    (num / multiplier) * (num - early_time)
}

#[allow(unused_variables)]
fn part2(ids: &[i64]) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_p1() {
        let content = fs::read_to_string("sample.txt").unwrap();
        let (early_time, bus_ids) = parse_input(&content);
        let res = part1(early_time, &bus_ids);

        assert_eq!(res, 295);
    }

    #[test]
    #[ignore]
    fn test_sample_p2() {
        let content = fs::read_to_string("sample.txt").unwrap();
        let (_, mut bus_ids) = parse_input(&content);
        let res = part2(&bus_ids);

        assert_eq!(res, 1068781);

        bus_ids = vec![17, 0, 13, 19];
        assert_eq!(part2(&bus_ids), 3417);

        bus_ids = vec![67, 7, 59, 61];
        assert_eq!(part2(&bus_ids), 754018);

        bus_ids = vec![67, 0, 7, 59, 61];
        assert_eq!(part2(&bus_ids), 779210);

        bus_ids = vec![67, 7, 0, 59, 61];
        assert_eq!(part2(&bus_ids), 1261476);

        bus_ids = vec![1789, 37, 47, 1889];
        assert_eq!(part2(&bus_ids), 1202161486);
    }
}
