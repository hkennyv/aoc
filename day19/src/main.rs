#![crate_name = "day19"]

//! ## --- Day 19: Monster Messages ---
//!
//! You land in an airport surrounded by dense forest. As you walk to your high-speed train, the Elves at the Mythical Information Bureau contact you again. They think their satellite has collected an image of a sea monster! Unfortunately, the connection to the satellite is having problems, and many of the messages sent back from the satellite have been corrupted.
//!
//! They sent you a list of the rules valid messages should obey and a list of received messages they've collected so far (your puzzle input).
//!
//! The rules for valid messages (the top part of your puzzle input) are numbered and build upon each other. For example:
//!
//! ```
//! 0: 1 2
//! 1: "a"
//! 2: 1 3 | 3 1
//! 3: "b"
//! ```
//!
//! Some rules, like 3: "b", simply match a single character (in this case, b).
//!
//! The remaining rules list the sub-rules that must be followed; for example, the rule 0: 1 2 means that to match rule 0, the text being checked must match rule 1, and the text after the part that matched rule 1 must then match rule 2.
//!
//! Some of the rules have multiple lists of sub-rules separated by a pipe (|). This means that at least one list of sub-rules must match. (The ones that match might be different each time the rule is encountered.) For example, the rule 2: 1 3 | 3 1 means that to match rule 2, the text being checked must match rule 1 followed by rule 3 or it must match rule 3 followed by rule 1.
//!
//! Fortunately, there are no loops in the rules, so the list of possible matches will be finite. Since rule 1 matches a and rule 3 matches b, rule 2 matches either ab or ba. Therefore, rule 0 matches aab or aba.
//!
//! Here's a more interesting example:
//!
//! ```
//! 0: 4 1 5
//! 1: 2 3 | 3 2
//! 2: 4 4 | 5 5
//! 3: 4 5 | 5 4
//! 4: "a"
//! 5: "b"
//! ```
//!
//! Here, because rule 4 matches a and rule 5 matches b, rule 2 matches two letters that are the same (aa or bb), and rule 3 matches two letters that are different (ab or ba).
//!
//! Since rule 1 matches rules 2 and 3 once each in either order, it must match two pairs of letters, one pair with matching letters and one pair with different letters. This leaves eight possibilities: aaab, aaba, bbab, bbba, abaa, abbb, baaa, or babb.
//!
//! Rule 0, therefore, matches a (rule 4), then any of the eight options from rule 1, then b (rule 5): aaaabb, aaabab, abbabb, abbbab, aabaab, aabbbb, abaaab, or ababbb.
//!
//! The received messages (the bottom part of your puzzle input) need to be checked against the rules so you can determine which are valid and which are corrupted. Including the rules and the messages together, this might look like:
//!
//! ```
//! 0: 4 1 5
//! 1: 2 3 | 3 2
//! 2: 4 4 | 5 5
//! 3: 4 5 | 5 4
//! 4: "a"
//! 5: "b"
//!
//! ababbb
//! bababa
//! abbbab
//! aaabbb
//! aaaabbb
//! ```
//!
//! Your goal is to determine the number of messages that completely match rule 0. In the above example, ababbb and abbbab match, but bababa, aaabbb, and aaaabbb do not, producing the answer 2. The whole message must match all of rule 0; there can't be extra unmatched characters in the message. (For example, aaaabbb might appear to match rule 0 above, but it has an extra unmatched b on the end.)
//!
//! How many messages completely match rule 0?
//!
//! Your puzzle answer was 171.
//!
//! ## --- Part Two ---
//!
//! As you look over the list of messages, you realize your matching rules aren't quite right. To fix them, completely replace rules 8: 42 and 11: 42 31 with the following:
//!
//! ```
//! 8: 42 | 42 8
//! 11: 42 31 | 42 11 31
//! ```
//!
//! This small change has a big impact: now, the rules do contain loops, and the list of messages they could hypothetically match is infinite. You'll need to determine how these changes affect which messages are valid.
//!
//! Fortunately, many of the rules are unaffected by this change; it might help to start by looking at which rules always match the same set of values and how those rules (especially rules 42 and 31) are used by the new versions of rules 8 and 11.
//!
//! (Remember, you only need to handle the rules you have; building a solution that could handle any hypothetical combination of rules would be significantly more difficult.)
//!
//! For example:
//!
//! ```
//! 42: 9 14 | 10 1
//! 9: 14 27 | 1 26
//! 10: 23 14 | 28 1
//! 1: "a"
//! 11: 42 31
//! 5: 1 14 | 15 1
//! 19: 14 1 | 14 14
//! 12: 24 14 | 19 1
//! 16: 15 1 | 14 14
//! 31: 14 17 | 1 13
//! 6: 14 14 | 1 14
//! 2: 1 24 | 14 4
//! 0: 8 11
//! 13: 14 3 | 1 12
//! 15: 1 | 14
//! 17: 14 2 | 1 7
//! 23: 25 1 | 22 14
//! 28: 16 1
//! 4: 1 1
//! 20: 14 14 | 1 15
//! 3: 5 14 | 16 1
//! 27: 1 6 | 14 18
//! 14: "b"
//! 21: 14 1 | 1 14
//! 25: 1 1 | 1 14
//! 22: 14 14
//! 8: 42
//! 26: 14 22 | 1 20
//! 18: 15 15
//! 7: 14 5 | 1 21
//! 24: 14 1
//!
//! abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
//! bbabbbbaabaabba
//! babbbbaabbbbbabbbbbbaabaaabaaa
//! aaabbbbbbaaaabaababaabababbabaaabbababababaaa
//! bbbbbbbaaaabbbbaaabbabaaa
//! bbbababbbbaaaaaaaabbababaaababaabab
//! ababaaaaaabaaab
//! ababaaaaabbbaba
//! baabbaaaabbaaaababbaababb
//! abbbbabbbbaaaababbbbbbaaaababb
//! aaaaabbaabaaaaababaa
//! aaaabbaaaabbaaa
//! aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
//! babaaabbbaaabaababbaabababaaab
//! aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
//! ```
//!
//! Without updating rules 8 and 11, these rules only match three messages: bbabbbbaabaabba, ababaaaaaabaaab, and ababaaaaabbbaba.
//!
//! However, after updating rules 8 and 11, a total of 12 messages match:
//!
//! ```
//! bbabbbbaabaabba
//! babbbbaabbbbbabbbbbbaabaaabaaa
//! aaabbbbbbaaaabaababaabababbabaaabbababababaaa
//! bbbbbbbaaaabbbbaaabbabaaa
//! bbbababbbbaaaaaaaabbababaaababaabab
//! ababaaaaaabaaab
//! ababaaaaabbbaba
//! baabbaaaabbaaaababbaababb
//! abbbbabbbbaaaababbbbbbaaaababb
//! aaaaabbaabaaaaababaa
//! aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
//! aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
//! ```
//!
//! After updating rules 8 and 11, how many messages completely match rule 0?
//!
//! Your puzzle answer was 369.

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
