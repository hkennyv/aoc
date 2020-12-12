#![crate_name = "day10"]

use std::fs;
fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let input: Vec<i64> = contents.lines().map(|line| line.parse().unwrap()).collect();

    let res_p1 = part1(&input);
    println!("res p1: {}", res_p1);

    let res_p2 = part2(&input);
    println!("res p2: {}", res_p2);
}

fn part1(input: &[i64]) -> i64 {
    // initial joltage of outlet is 0
    let mut vec: Vec<i64> = vec![0];
    vec.extend(input);
    vec.sort_unstable();

    let diffs: Vec<i64> = vec.windows(2).map(|win| win[1] - win[0]).collect();

    let ones = diffs.iter().filter(|&&num| num == 1).count();

    // the number of 3's always has a +1 because of your phone
    let threes = diffs.iter().filter(|&&num| num == 3).count() + 1;

    (ones * threes) as i64
}

fn part2(input: &[i64]) -> i64 {
    // initial joltage of outlet is 0
    let mut vec: Vec<i64> = vec![0];
    vec.extend(input);
    vec.sort_unstable();

    let mut paths = vec![0; vec.len()];

    // there is only 1 way you can arrange the last adapter
    paths[vec.len() - 1] = 1;

    // work backwards, knowing that each adapter can make sum(# of connections
    // that each of the adapters in the larger range can make)
    for i in (0..vec.len() - 1).rev() {
        paths[i] = 0;
        let mut lookahead = 1;

        // sum the adapters in the larger range
        while i + lookahead < vec.len() && vec[i + lookahead] - vec[i] <= 3 && lookahead <= 3 {
            paths[i] += paths[i + lookahead];
            lookahead += 1;
        }
    }

    paths[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_p1() {
        let contents = fs::read_to_string("sample.txt").unwrap();
        let input: Vec<i64> = contents.lines().map(|line| line.parse().unwrap()).collect();

        let res_p1 = part1(&input);
        assert_eq!(res_p1, 220);
    }

    #[test]
    fn test_sample_p2() {
        let contents = fs::read_to_string("sample.txt").unwrap();
        let input: Vec<i64> = contents.lines().map(|line| line.parse().unwrap()).collect();

        let res_p2 = part2(&input);
        assert_eq!(res_p2, 19208);
    }
}
