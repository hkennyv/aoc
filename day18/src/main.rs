#![crate_name = "day18"]

//! ## --- Day 18: Operation Order ---
//!
//! As you look out the window and notice a heavily-forested continent slowly appear over the horizon, you are interrupted by the child sitting next to you. They're curious if you could help them with their math homework.
//!
//! Unfortunately, it seems like this "math" follows different rules than you remember.
//!
//! The homework (your puzzle input) consists of a series of expressions that consist of addition (+), multiplication (*), and parentheses ((...)). Just like normal math, parentheses indicate that the expression inside must be evaluated before it can be used by the surrounding expression. Addition still finds the sum of the numbers on both sides of the operator, and multiplication still finds the product.
//!
//! However, the rules of operator precedence have changed. Rather than evaluating multiplication before addition, the operators have the same precedence, and are evaluated left-to-right regardless of the order in which they appear.
//!
//! For example, the steps to evaluate the expression 1 + 2 * 3 + 4 * 5 + 6 are as follows:
//!
//! ```
//! 1 + 2 * 3 + 4 * 5 + 6
//!   3   * 3 + 4 * 5 + 6
//!       9   + 4 * 5 + 6
//!          13   * 5 + 6
//!              65   + 6
//!                  71
//! ```
//!
//! Parentheses can override this order; for example, here is what happens if parentheses are added to form 1 + (2 * 3) + (4 * (5 + 6)):
//!
//! ```
//! 1 + (2 * 3) + (4 * (5 + 6))
//! 1 +    6    + (4 * (5 + 6))
//!      7      + (4 * (5 + 6))
//!      7      + (4 *   11   )
//!      7      +     44
//!             51
//! ```
//!
//! Here are a few more examples:
//!
//! ```
//! 2 * 3 + (4 * 5) becomes 26.
//! 5 + (8 * 3 + 9 + 3 * 4 * 3) becomes 437.
//! 5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4)) becomes 12240.
//! ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2 becomes 13632.
//! ```
//!
//! Before you can help with the homework, you need to understand it yourself. Evaluate the expression on each line of the homework; what is the sum of the resulting values?
//!
//! Your puzzle answer was 14208061823964.
//!
//! ## --- Part Two ---
//! You manage to answer the child's questions and they finish part 1 of their homework, but get stuck when they reach the next section: advanced math.
//!
//! Now, addition and multiplication have different precedence levels, but they're not the ones you're familiar with. Instead, addition is evaluated before multiplication.
//!
//! For example, the steps to evaluate the expression 1 + 2 * 3 + 4 * 5 + 6 are now as follows:
//!
//! ```
//! 1 + 2 * 3 + 4 * 5 + 6
//!   3   * 3 + 4 * 5 + 6
//!   3   *   7   * 5 + 6
//!   3   *   7   *  11
//!      21       *  11
//!          231
//! ```
//!
//! Here are the other examples from above:
//!
//! ```
//! 1 + (2 * 3) + (4 * (5 + 6)) still becomes 51.
//! 2 * 3 + (4 * 5) becomes 46.
//! 5 + (8 * 3 + 9 + 3 * 4 * 3) becomes 1445.
//! 5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4)) becomes 669060.
//! ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2 becomes 23340.
//! ```
//!
//! What do you get if you add up the results of evaluating the homework problems using these new rules?
//!
//! Your puzzle answer was 320536571743074.

use std::collections::{HashMap, VecDeque};
use std::fs;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Token {
    Num(u64),
    LeftP,
    Add,
    Mul,
}

fn main() {
    let input = parse_input("input.txt");

    let res_p1 = part1(&input);
    println!("res p1: {}", res_p1);

    let res_p2 = part2(&input);
    println!("res p2: {}", res_p2);
}

fn parse_input(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).unwrap();
    contents.lines().map(|line| line.to_string()).collect()
}

/// evaluates the expression using a
/// [shunting-yard algorithm](https://brilliant.org/wiki/shunting-yard-algorithm/)
fn part1(expressions: &[String]) -> u64 {
    // parenthesis take first precedance
    let mut precedance: HashMap<Token, u64> = HashMap::new();
    precedance.insert(Token::Add, 1);
    precedance.insert(Token::Mul, 1);
    precedance.insert(Token::LeftP, 0);

    expressions
        .iter()
        .map(|expression| solve_reverse_polish(&mut make_reverse_polish(expression, &precedance)))
        .sum()
}

/// evaluates the expression using a
/// [shunting-yard algorithm](https://brilliant.org/wiki/shunting-yard-algorithm/)
fn part2(expressions: &[String]) -> u64 {
    // parenthesis take first precedance
    let mut precedance: HashMap<Token, u64> = HashMap::new();
    precedance.insert(Token::Add, 2);
    precedance.insert(Token::Mul, 1);
    precedance.insert(Token::LeftP, 0);

    expressions
        .iter()
        .map(|expression| solve_reverse_polish(&mut make_reverse_polish(expression, &precedance)))
        .sum()
}

fn make_reverse_polish(expression: &str, precedance: &HashMap<Token, u64>) -> VecDeque<Token> {
    let mut stack: Vec<Token> = vec![];
    let mut output: VecDeque<Token> = VecDeque::new();

    let tokens: Vec<char> = expression
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .collect();

    for token in tokens {
        match token {
            '+' => loop {
                match stack.pop() {
                    Some(op) => {
                        let p1 = precedance.get(&Token::Add).unwrap();
                        let p2 = precedance.get(&op).unwrap();

                        if p2 >= p1 {
                            output.push_back(op);
                        } else {
                            stack.push(op);
                            stack.push(Token::Add);
                            break;
                        }
                    }
                    None => {
                        stack.push(Token::Add);
                        break;
                    }
                }
            },
            '*' => loop {
                match stack.pop() {
                    Some(op) => {
                        let p1 = precedance.get(&Token::Mul).unwrap();
                        let p2 = precedance.get(&op).unwrap();

                        if p2 >= p1 {
                            output.push_back(op);
                        } else {
                            stack.push(op);
                            stack.push(Token::Mul);
                            break;
                        }
                    }
                    None => {
                        stack.push(Token::Mul);
                        break;
                    }
                }
            },
            ')' => loop {
                match stack.pop() {
                    Some(Token::LeftP) | None => break,
                    Some(operator) => output.push_back(operator),
                }
            },
            '(' => stack.push(Token::LeftP),
            num => output.push_back(Token::Num(num.to_digit(10).unwrap() as u64)),
        }
    }

    while let Some(op) = stack.pop() {
        output.push_back(op);
    }

    output
}

fn solve_reverse_polish(output: &mut VecDeque<Token>) -> u64 {
    let mut stack: Vec<u64> = vec![];

    loop {
        match output.pop_front() {
            Some(Token::Add) => {
                let lh = stack.pop().unwrap();
                let rh = stack.pop().unwrap();
                let sum = lh + rh;
                stack.push(sum);
            }
            Some(Token::Mul) => {
                let lh = stack.pop().unwrap();
                let rh = stack.pop().unwrap();
                let product = lh * rh;
                stack.push(product);
            }
            Some(Token::Num(num)) => stack.push(num as u64),
            _ => {}
        }

        if output.is_empty() && stack.len() == 1 {
            return stack[0];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn test_make_reverse_polish() {
        // parenthesis take first precedance
        let mut precedance: HashMap<Token, u64> = HashMap::new();
        precedance.insert(Token::Add, 1);
        precedance.insert(Token::Mul, 1);
        precedance.insert(Token::LeftP, 0);

        let pairs = vec![
            (
                "1 * 4 + 2",
                VecDeque::from_iter(vec![
                    Token::Num(1),
                    Token::Num(4),
                    Token::Mul,
                    Token::Num(2),
                    Token::Add,
                ]),
            ),
            (
                "9 * 2 * 3",
                VecDeque::from_iter(vec![
                    Token::Num(9),
                    Token::Num(2),
                    Token::Mul,
                    Token::Num(3),
                    Token::Mul,
                ]),
            ),
            (
                "5 + (8 * 3 + 9 + 3 * 4  * 3)",
                VecDeque::from_iter(vec![
                    Token::Num(5),
                    Token::Num(8),
                    Token::Num(3),
                    Token::Mul,
                    Token::Num(9),
                    Token::Add,
                    Token::Num(3),
                    Token::Add,
                    Token::Num(4),
                    Token::Mul,
                    Token::Num(3),
                    Token::Mul,
                    Token::Add,
                ]),
            ),
        ];

        for (input, answer) in pairs {
            assert_eq!(make_reverse_polish(input, &precedance), answer);
        }
    }
}
