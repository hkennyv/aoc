#![crate_name = "day19"]

use std::collections::HashMap;
use std::fs;
use std::iter::FromIterator;

use regex::Regex;

/// This enum defines a "Rule". That is, either a single character, a list
/// of rule numbers, or multiple lists of sub rule numbers separated by a
/// pipe character (|).
#[derive(Clone, Debug, PartialEq)]
enum Rule {
    /// A single character (this is essentially our base case)
    Character(char),
    /// A vector of rule numbers, we will need to recursively evaluate each
    /// rule number until we get a sequence of characters
    Single(Vec<usize>),
    /// Multiple vectors of rule numbers. We will need to recursively evaluate
    /// each side of the pipe character. The evaluated sequence of characters
    /// for this type of rule is (A|B) where A is the left side and B is the
    /// right side of the pipe.
    Multiple(Vec<Vec<usize>>),
}

fn main() {
    let (rules, messages) = parse_input("input.txt");
    let res_p1 = part1(&rules, &messages);
    println!("res p1: {}", res_p1);

    let res_p2 = part2(&rules, &messages);
    println!("res p2: {}", res_p2);
}

/// The input.txt is split up into two portions: a top and a bottom. The top
/// portion contains key value pairs (k: v) of the rules. The bottom half
/// contains the test messages.
fn parse_input(filename: &str) -> (HashMap<usize, Rule>, Vec<String>) {
    let contents = fs::read_to_string(filename).unwrap();
    let mut input = contents.split("\n\n");

    let rules: HashMap<usize, Rule> = HashMap::from_iter(
        input
            .next()
            .unwrap()
            .lines()
            .map(|line| line.split(':'))
            .map(|mut split| {
                (
                    split.next().unwrap().parse::<usize>().unwrap(),
                    parse_rule(split.next().unwrap()),
                )
            })
            .collect::<Vec<(usize, Rule)>>(),
    );

    let messages = input
        .next()
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect();

    (rules, messages)
}

/// We want to evaluate rule 0 and see which messages match the evaluated
/// character sequence.
fn part1(rules: &HashMap<usize, Rule>, messages: &[String]) -> usize {
    let rule = rules.get(&0).unwrap();

    let mut rgx = match rule {
        Rule::Single(v) => evaluate_rule(rules, v),
        _ => String::new(),
    };

    // we need to add the "start of string" and "end of string" tokens to make
    // sure we are matching _only_ what the rules say
    rgx = format!("^{}$", rgx);

    let re = Regex::new(&rgx).unwrap();

    let mut count = 0;
    for message in messages {
        if re.is_match(message) {
            count += 1;
        }
    }

    count
}

/// In this part, we replace rule 8 and 11 with 42 | 42 8 and 42 31 | 42 11 31
/// respectively. For this implementation, we define a maximum depth that we
/// want to generate the rules for since they are recursive (5 is the maximum
/// depth required for _my_ input).
fn part2(rules: &HashMap<usize, Rule>, messages: &[String]) -> usize {
    const MAX_DEPTH: usize = 5;
    let mut count = 0;

    let rule = rules.get(&0).unwrap();

    // build rule8
    let mut new_rule8 = Vec::new();
    for i in 0..MAX_DEPTH {
        new_rule8.push(vec![42; i + 1])
    }

    // build rul11
    let mut new_rule11 = Vec::new();
    for i in 0..MAX_DEPTH {
        let mut inner = vec![42; i + 1];
        inner.extend(vec![31; i + 1]);
        new_rule11.push(inner);
    }

    let mut new_rules = rules.clone();

    new_rules.insert(8, Rule::Multiple(new_rule8));
    new_rules.insert(11, Rule::Multiple(new_rule11));

    let mut rgx = match rule {
        Rule::Single(v) => evaluate_rule(&new_rules, v),
        _ => String::new(),
    };

    // we need to add the "start of string" and "end of string" tokens to make
    // sure we are matching _only_ what the rules say
    rgx = format!("^{}$", rgx);

    let re = Regex::new(&rgx).unwrap();

    for message in messages {
        if re.is_match(message) {
            count += 1;
        }
    }

    count
}

/// Parses a rule given a string slice
fn parse_rule(s: &str) -> Rule {
    // Multiple
    if s.contains('|') {
        Rule::Multiple(s.split('|').map(|rule| s_to_vec(rule)).collect())

    // Character
    } else if s.contains('"') {
        Rule::Character(
            s.chars()
                .filter(|ch| ch.is_alphabetic())
                .collect::<Vec<char>>()[0],
        )
    // Single
    } else {
        Rule::Single(s_to_vec(s))
    }
}

/// Evaluates a rule recurively and returns the string correpsonding to the
/// evaluated character sequence.
fn evaluate_rule(rules: &HashMap<usize, Rule>, v: &[usize]) -> String {
    let mut s: String = String::new();

    for n in v {
        let rule = rules.get(n).unwrap();

        match rule {
            Rule::Character(c) => s.push_str(&format!("({})", *c)),
            Rule::Single(sub_rules) => s.push_str(&evaluate_rule(rules, sub_rules)),
            Rule::Multiple(sub_rules) => {
                let inner = sub_rules
                    .iter()
                    .map(|sub_rule| evaluate_rule(rules, sub_rule))
                    .collect::<Vec<String>>()
                    .join("|");

                s.push_str(&format!("({})", inner));
            }
        }
    }

    s
}

/// A simple helper function to convert a string to a Vec<usize>
fn s_to_vec(s: &str) -> Vec<usize> {
    s.trim()
        .split(' ')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rule() {
        let strings = vec![
            "4 1 5",
            "2 3 | 3 2",
            "4 4 | 5 5",
            "4 5 | 5 4",
            "\"a\"",
            "\"b\"",
        ];
        let answers = vec![
            Rule::Single(vec![4, 1, 5]),
            Rule::Multiple(vec![vec![2, 3], vec![3, 2]]),
            Rule::Multiple(vec![vec![4, 4], vec![5, 5]]),
            Rule::Multiple(vec![vec![4, 5], vec![5, 4]]),
            Rule::Character('a'),
            Rule::Character('b'),
        ];

        for i in 0..strings.len() {
            assert_eq!(parse_rule(strings[i]), answers[i]);
        }
    }

    #[test]
    fn test_part1() {
        let mut rules = HashMap::new();
        rules.insert(0, parse_rule("4 1 5"));
        rules.insert(1, parse_rule("2 3 | 3 2"));
        rules.insert(2, parse_rule("4 4 | 5 5"));
        rules.insert(3, parse_rule("4 5 | 5 4 "));
        rules.insert(4, parse_rule("\"a\""));
        rules.insert(5, parse_rule("\"b\""));

        let messages = vec![
            "ababbb".to_string(),
            "bababa".to_string(),
            "abbbab".to_string(),
            "aaabbb".to_string(),
            "aaaabbb".to_string(),
        ];

        assert_eq!(part1(&rules, &messages), 2);
    }
}
