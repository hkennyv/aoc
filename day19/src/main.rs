#![crate_name = "day19"]

use std::collections::HashMap;
use std::fs;
use std::iter::FromIterator;

fn main() {
    let (rules, messages) = parse_input("input.txt");

    println!("{:?}", messages);
}

fn parse_input(filename: &str) -> (HashMap<usize, String>, Vec<String>) {
    let contents = fs::read_to_string(filename).unwrap();
    let mut input = contents.split("\n\n");

    let rules: HashMap<usize, String> = HashMap::from_iter(
        input
            .next()
            .unwrap()
            .lines()
            .map(|line| line.split(':'))
            .map(|mut split| {
                (
                    split.next().unwrap().parse::<usize>().unwrap(),
                    split.next().unwrap().to_string(),
                )
            })
            .collect::<Vec<(usize, String)>>(),
    );

    let messages = input
        .next()
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect();

    (rules, messages)
}

fn part1(rules: &HashMap<usize, String>, messages: &[&str]) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut rules = HashMap::new();
        rules.insert(0, String::from("4 1 5"));
        rules.insert(1, String::from("2 3 | 3 2"));
        rules.insert(2, String::from("4 4 | 5 5"));
        rules.insert(3, String::from("4 5 | 5 4 "));
        rules.insert(4, String::from("a"));

        let messages = vec!["ababbb", "bababa", "abbbab", "aaabbb", "aaaabbb"];

        assert_eq!(part1(&rules, &messages), 2);
    }
}
