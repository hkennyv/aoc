#![crate_name = "day18"]

use std::collections::{HashMap, VecDeque};
use std::fs;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Token {
    Num(i32),
    LeftP,
    Add,
    Mul,
}

fn main() {
    let input = parse_input("input.txt");
    let res_p1 = part1(&input);
    println!("res p1: {}", res_p1);
}

fn parse_input(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).unwrap();
    contents.lines().map(|line| line.to_string()).collect()
}

fn part1(expressions: &[String]) -> i32 {
    // parenthesis take first precedance
    let mut precedance: HashMap<Token, i32> = HashMap::new();
    precedance.insert(Token::Add, 1);
    precedance.insert(Token::Mul, 1);
    precedance.insert(Token::LeftP, 0);

    expressions
        .iter()
        .map(|expression| evaluate_expression(expression, &precedance))
        .sum()
}

/// evaluates the expression using a
/// [shunting-yard algorithm](https://brilliant.org/wiki/shunting-yard-algorithm/)
fn evaluate_expression(expression: &str, precedance: &HashMap<Token, i32>) -> i32 {
    let mut output_queue: VecDeque<Token> = VecDeque::new();
    let mut operator_stack: Vec<Token> = vec![];

    let tokens = expression.chars().filter(|ch| !ch.is_whitespace());

    for token in tokens {
        match token {
            '+' => {
                let p = precedance.get(&Token::Add).unwrap();

                loop {
                    match operator_stack.pop() {
                        Some(top) => {
                            let top_p = precedance.get(&top).unwrap();

                            if top_p < p {
                                output_queue.push_back(top);
                            } else {
                                operator_stack.push(top);
                                operator_stack.push(Token::Add);
                                break;
                            }
                        }
                        None => {
                            operator_stack.push(Token::Add);
                            break;
                        }
                    }
                }
            }
            '*' => {
                let p = precedance.get(&Token::Mul).unwrap();

                loop {
                    match operator_stack.pop() {
                        Some(top) => {
                            let top_p = precedance.get(&top).unwrap();
                            if top_p < p {
                                output_queue.push_back(top);
                            } else {
                                operator_stack.push(top);
                                operator_stack.push(Token::Mul);
                                break;
                            }
                        }
                        None => {
                            operator_stack.push(Token::Mul);
                            break;
                        }
                    }
                }
            }
            '(' => operator_stack.push(Token::LeftP),
            ')' => loop {
                match operator_stack.pop() {
                    Some(Token::LeftP) => break,
                    Some(top) => output_queue.push_back(top),
                    None => break,
                }
            },
            n => output_queue.push_back(Token::Num(n.to_digit(10).unwrap() as i32)),
        }
    }

    // while there are operators on the stack, pop them to the queue
    loop {
        match operator_stack.pop() {
            Some(t) => output_queue.push_back(t),
            None => break,
        }
    }

    println!("{:?}", output_queue);

    solve_reverse_polish(&mut output_queue)
}

fn solve_reverse_polish(deque: &mut VecDeque<Token>) -> i32 {
    let mut stack: Vec<i32> = vec![];

    println!("deque: {:?}", deque);

    loop {
        match deque.pop_front() {
            Some(Token::Add) => {
                let lh = stack.pop().unwrap();
                let rh = stack.pop().unwrap();
                let sum = lh + rh;
                println!("{} + {}", lh, rh);
                stack.push(sum);
            }
            Some(Token::Mul) => {
                let lh = stack.pop().unwrap();
                let rh = stack.pop().unwrap();
                let product = lh * rh;
                println!("{} * {}", lh, rh);
                stack.push(product);
            }
            Some(Token::Num(num)) => stack.push(num),
            None | _ => {}
        }

        println!("stack: {:?}", stack);

        if deque.len() == 0 && stack.len() == 1 {
            return stack[0];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_expression() {
        // parenthesis take first precedance
        let mut precedance: HashMap<Token, i32> = HashMap::new();
        precedance.insert(Token::Add, 1);
        precedance.insert(Token::Mul, 1);
        precedance.insert(Token::LeftP, 0);

        let pairs = vec![
            ("1 + 1 + 1", 3),
            ("2 * 2 * 1", 4),
            ("1 + 4 * 2", 10),
            ("2 * 2 * 1 + 1", 5),
            ("2 * 3 + (4 * 5)", 26),
            ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437),
            ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240),
            ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632),
        ];

        for (expr, ans) in pairs {
            assert_eq!(evaluate_expression(expr, &precedance), ans);
        }
    }
}
