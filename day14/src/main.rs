#![crate_name = "day14"]

use std::fs;

mod system;
use system::DockSystem;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let res_p1 = part1(&contents);

    println!("res p1: {}", res_p1);
}

fn part1(input: &str) -> u64 {
    let mut sys = DockSystem::new();

    for line in input.lines() {
        if line.starts_with("mask") {
            let mask = line.split_whitespace().last().unwrap().to_string();
            sys.update_mask(&mask);
        } else {
            let value: u64 = line.split_whitespace().last().unwrap().parse().unwrap();
            let address: u64 = line.split(" = ").collect::<Vec<&str>>()[0]
                .strip_prefix("mem[")
                .unwrap()
                .strip_suffix(']')
                .unwrap()
                .parse()
                .unwrap();

            sys.update_memory(address, value);
        }
    }

    sys.get_sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_p1() {
        let contents = fs::read_to_string("sample.txt").unwrap();
        let res_p1 = part1(&contents);

        assert_eq!(res_p1, 165);
    }
}
