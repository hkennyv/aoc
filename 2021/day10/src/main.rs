#![crate_name = "day10"]

//! ## --- Day 10: Syntax Scoring ---
//!
//! You ask the submarine to determine the best route out of the deep-sea cave, but it only replies:
//!
//! Syntax error in navigation subsystem on line: all of them
//! All of them?! The damage is worse than you thought. You bring up a copy of the navigation subsystem (your puzzle input).
//!
//! The navigation subsystem syntax is made of several lines containing chunks. There are one or more chunks on each line, and chunks contain zero or more other chunks. Adjacent chunks are not separated by any delimiter; if one chunk stops, the next chunk (if any) can immediately start. Every chunk must open and close with one of four legal pairs of matching characters:
//!
//! ```
//! If a chunk opens with (, it must close with ).
//! If a chunk opens with [, it must close with ].
//! If a chunk opens with {, it must close with }.
//! If a chunk opens with <, it must close with >.
//! ```
//!
//! So, `()` is a legal chunk that contains no other chunks, as is `[]`. More complex but valid chunks include `([])`, `{()()()}`, `<([{}])>`, `[<>({}){}[([])<>]]`, and even `(((((((((())))))))))`.
//!
//! Some lines are incomplete, but others are corrupted. Find and discard the corrupted lines first.
//!
//! A corrupted line is one where a chunk closes with the wrong character - that is, where the characters it opens and closes with do not form one of the four legal pairs listed above.
//!
//! Examples of corrupted chunks include `(]`, `{()()()>`, `(((()))}`, and `<([]){()}[{}])`. Such a chunk can appear anywhere within a line, and its presence causes the whole line to be considered corrupted.
//!
//! For example, consider the following navigation subsystem:
//!
//! ```
//! [({(<(())[]>[[{[]{<()<>>
//! [(()[<>])]({[<{<<[]>>(
//! {([(<{}[<>[]}>{[]{[(<()>
//! (((({<>}<{<{<>}{[]{[]{}
//! [[<[([]))<([[{}[[()]]]
//! [{[{({}]{}}([{[{{{}}([]
//! {<[[]]>}<{[{[{[]{()[[[]
//! [<(<(<(<{}))><([]([]()
//! <{([([[(<>()){}]>(<<{{
//! <{([{{}}[<[[[<>{}]]]>[]]
//! ```
//!
//! Some of the lines aren't corrupted, just incomplete; you can ignore these lines for now. The remaining five lines are corrupted:
//!
//! ```
//! {([(<{}[<>[]}>{[]{[(<()> - Expected ], but found } instead.
//! [[<[([]))<([[{}[[()]]] - Expected ], but found ) instead.
//! [{[{({}]{}}([{[{{{}}([] - Expected ), but found ] instead.
//! [<(<(<(<{}))><([]([]() - Expected >, but found ) instead.
//! <{([([[(<>()){}]>(<<{{ - Expected ], but found > instead.
//! ```
//!
//! Stop at the first incorrect closing character on each corrupted line.
//!
//! Did you know that syntax checkers actually have contests to see who can get the high score for syntax errors in a file? It's true! To calculate the syntax error score for a line, take the first illegal character on the line and look it up in the following table:
//!
//! - `)`: 3 points.
//! - `]`: 57 points.
//! - `}`: 1197 points.
//! - `>` : 25137 points.
//!
//! In the above example, an illegal `)` was found twice (`2 * 3 = 6` points), an illegal `]` was found once (57 points), an illegal `}` was found once (1197 points), and an illegal `>` was found once (25137 points). So, the total syntax error score for this file is 6+57+1197+25137 = 26397 points!
//!
//! Find the first illegal character in each corrupted line of the navigation subsystem. What is the total syntax error score for those errors?
//!
//! Your puzzle answer was 392421.
//!
//! ## --- Part Two ---
//!
//! Now, discard the corrupted lines. The remaining lines are incomplete.
//!
//! Incomplete lines don't have any incorrect characters - instead, they're missing some closing characters at the end of the line. To repair the navigation subsystem, you just need to figure out the sequence of closing characters that complete all open chunks in the line.
//!
//! You can only use closing characters (`)`, `]`, `}`, or `>`), and you must add them in the correct order so that only legal pairs are formed and all chunks end up closed.
//!
//! In the example above, there are five incomplete lines:
//!
//! ```
//! [({(<(())[]>[[{[]{<()<>> - Complete by adding }}]])})].
//! [(()[<>])]({[<{<<[]>>( - Complete by adding )}>]}).
//! (((({<>}<{<{<>}{[]{[]{} - Complete by adding }}>}>)))).
//! {<[[]]>}<{[{[{[]{()[[[] - Complete by adding]]}}]}]}>.
//! <{([{{}}[<[[[<>{}]]]>[]] - Complete by adding ])}>.
//! ```
//!
//! Did you know that autocomplete tools also have contests? It's true! The score is determined by considering the completion string character-by-character. Start with a total score of 0. Then, for each character, multiply the total score by 5 and then increase the total score by the point value given for the character in the following table:
//!
//! - `)`: 1 point.
//! - `]`: 2 points.
//! - `}`: 3 points.
//! - `>` : 4 points.
//!
//! So, the last completion string above - `])}>` - would be scored as follows:
//!
//! ```
//! Start with a total score of 0.
//! Multiply the total score by 5 to get 0, then add the value of ] (2) to get a new total score of 2.
//! Multiply the total score by 5 to get 10, then add the value of ) (1) to get a new total score of 11.
//! Multiply the total score by 5 to get 55, then add the value of } (3) to get a new total score of 58.
//! Multiply the total score by 5 to get 290, then add the value of > (4) to get a new total score of 294.
//! ```
//!
//! The five lines' completion strings have total scores as follows:
//!
//! ```
//! }}]])})] - 288957 total points.
//! )}>]}) - 5566 total points.
//! }}>}>)))) - 1480781 total points.
//! ]]}}]}]}> - 995444 total points.
//! ])}> - 294 total points.
//! ```
//!
//! Autocomplete tools are an odd bunch: the winner is found by sorting all of the scores and then taking the middle score. (There will always be an odd number of scores to consider.) In this example, the middle score is 288957 because there are the same number of scores smaller and larger than it.
//!
//! Find the completion string for each incomplete line, score the completion strings, and sort the scores. What is the middle score?
//!
//! Your puzzle answer was 2769449099.

use std::collections::HashMap;
use std::fs;

fn main() {
    let input = read_input("input.txt");
    let part1 = part1(&input);
    let part2 = part2(&input);

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}

fn part1(input: &[String]) -> usize {
    // flat map is equivalent to .map().filter(|e| e), which filters out all
    // of the `None` values.
    input.iter().flat_map(|line| calculate_error(line)).sum()
}

fn part2(input: &[String]) -> usize {
    let mut scores: Vec<usize> = input.iter().flat_map(|line| score_line(line)).collect();

    scores.sort_unstable();

    // return the middle element
    scores[scores.len() / 2]
}

/// Returns a score if the line is incomplete and returns `None` if the line
/// is corrupted.
fn score_line(line: &str) -> Option<usize> {
    // use a hashmap to map a closing or opening character to its corresponding
    // opening or closing character
    let compliment_map: HashMap<char, char> = HashMap::from_iter(vec![
        (')', '('), // parentheses
        ('(', ')'),
        (']', '['), // square brackets
        ('[', ']'),
        ('}', '{'), // curly bracketes
        ('{', '}'),
        ('>', '<'), // angled brackets
        ('<', '>'),
    ]);

    // we're going to use a stack to manage the characters in the string,
    // **pushing** the opening characters onto the stack and **popping**
    // characters off the stack when we encounter a closing character.
    //
    // When we encounter a corrupted line, the character we pop off the stack
    // will not match the compliment. In this case, return `None`.
    let mut stack = Vec::new();

    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' | ']' | '}' | '>' => {
                if let Some(last) = stack.pop() {
                    let compliment = *compliment_map.get(&c).unwrap();

                    // if it is corrupted, we want to ignore this line and
                    // return None
                    if last != compliment {
                        return None;
                    }
                } else {
                    panic!("Started with an ending character?");
                }
            }
            _ => panic!("Unexpected character: {}", c),
        }
    }

    // if we're here, then the string is _not_ corrupted, just incomplete
    // and all of the remaining characters in the `stack` should just be
    // opening characaters. We can use these to calculate the score.
    let mut score = 0;

    while !stack.is_empty() {
        let last = stack.pop().unwrap();
        match compliment_map.get(&last).unwrap() {
            ')' => score = score * 5 + 1,
            ']' => score = score * 5 + 2,
            '}' => score = score * 5 + 3,
            '>' => score = score * 5 + 4,
            _ => panic!("Unexpected character {}", last),
        };
    }

    Some(score)
}

fn calculate_error(line: &str) -> Option<usize> {
    let compliment_map: HashMap<char, char> =
        HashMap::from_iter(vec![(')', '('), (']', '['), ('}', '{'), ('>', '<')]);

    // maps the closing characters to a value that is used to calculate
    // the error score
    let error_map: HashMap<char, usize> =
        HashMap::from_iter(vec![(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);

    let mut stack = Vec::new();

    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' | ']' | '}' | '>' => {
                if let Some(last) = stack.pop() {
                    let compliment = *compliment_map.get(&c).unwrap();

                    // if the last element isn't the closing token, then we
                    // have an error. Expected `compliment` but got `c` instead
                    if last != compliment {
                        return Some(*error_map.get(&c).unwrap());
                    }
                } else {
                    panic!("Started with an ending character?");
                }
            }
            _ => (),
        }
    }

    None
}

fn read_input(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .expect("Failed to read input file")
        .lines()
        .map(|l| l.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input: Vec<String> = vec![
            r"[({(<(())[]>[[{[]{<()<>>",
            r"[(()[<>])]({[<{<<[]>>(",
            r"{([(<{}[<>[]}>{[]{[(<()>",
            r"(((({<>}<{<{<>}{[]{[]{}",
            r"[[<[([]))<([[{}[[()]]]",
            r"[{[{({}]{}}([{[{{{}}([]",
            r"{<[[]]>}<{[{[{[]{()[[[]",
            r"[<(<(<(<{}))><([]([]()",
            r"<{([([[(<>()){}]>(<<{{",
            r"<{([{{}}[<[[[<>{}]]]>[]]",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        assert_eq!(part1(&input), 26397);
    }

    #[test]
    fn test_part2() {
        let input: Vec<String> = vec![
            r"[({(<(())[]>[[{[]{<()<>>",
            r"[(()[<>])]({[<{<<[]>>(",
            r"{([(<{}[<>[]}>{[]{[(<()>",
            r"(((({<>}<{<{<>}{[]{[]{}",
            r"[[<[([]))<([[{}[[()]]]",
            r"[{[{({}]{}}([{[{{{}}([]",
            r"{<[[]]>}<{[{[{[]{()[[[]",
            r"[<(<(<(<{}))><([]([]()",
            r"<{([([[(<>()){}]>(<<{{",
            r"<{([{{}}[<[[[<>{}]]]>[]]",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        assert_eq!(part2(&input), 288957);
    }
}
