#![crate_name = "day10"]

use std::fs;
fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let input: Vec<i32> = contents.lines().map(|line| line.parse().unwrap()).collect();

    let res_p1 = part1(&input);
    println!("res p1: {}", res_p1);
}

fn part1(input: &[i32]) -> i32 {
    let mut vec: Vec<i32> = vec![0];
    vec.extend(input);
    vec.sort_unstable();

    let diffs: Vec<i32> = vec.windows(2).map(|win| win[1] - win[0]).collect();

    let ones = diffs.iter().filter(|&&num| num == 1).count();

    // the number of 3's always has a +1 because of your phone
    let threes = diffs.iter().filter(|&&num| num == 3).count() + 1;

    println!("sorted: {:?}\ndiffs: {:?}", vec, diffs);
    println!("ones: {} threes: {}", ones, threes);

    (ones * threes) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_p1() {}
}
