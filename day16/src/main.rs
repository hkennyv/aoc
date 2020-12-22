#![crate_name = "day16"]

//! ## --- Day 16: Ticket Translation ---
//!
//! As you're walking to yet another connecting flight, you realize that one of the legs of your re-routed trip coming up is on a high-speed train. However, the train ticket you were given is in a language you don't understand. You should probably figure out what it says before you get to the train station after the next flight.
//!
//! Unfortunately, you can't actually read the words on the ticket. You can, however, read the numbers, and so you figure out the fields these tickets must have and the valid ranges for values in those fields.
//!
//! You collect the rules for ticket fields, the numbers on your ticket, and the numbers on other nearby tickets for the same train service (via the airport security cameras) together into a single document you can reference (your puzzle input).
//!
//! The rules for ticket fields specify a list of fields that exist somewhere on the ticket and the valid ranges of values for each field. For example, a rule like class: 1-3 or 5-7 means that one of the fields in every ticket is named class and can be any value in the ranges 1-3 or 5-7 (inclusive, such that 3 and 5 are both valid in this field, but 4 is not).
//!
//! Each ticket is represented by a single line of comma-separated values. The values are the numbers on the ticket in the order they appear; every ticket has the same format. For example, consider this ticket:
//!
//! ```
//! .--------------------------------------------------------.
//! | ????: 101 ?????: 102 ??????????: 103 ???: 104 |
//! | |
//! | ??: 301 ??: 302 ???????: 303 ??????? |
//! | ??: 401 ??: 402 ???? ????: 403 ????????? |
//! '--------------------------------------------------------'
//! ```
//!
//! Here, ? represents text in a language you don't understand. This ticket might be represented as 101,102,103,104,301,302,303,401,402,403; of course, the actual train tickets you're looking at are much more complicated. In any case, you've extracted just the numbers in such a way that the first number is always the same specific field, the second number is always a different specific field, and so on - you just don't know what each position actually means!
//!
//! Start by determining which tickets are completely invalid; these are tickets that contain values which aren't valid for any field. Ignore your ticket for now.
//!
//! For example, suppose you have the following notes:
//!
//! ```
//! class: 1-3 or 5-7
//! row: 6-11 or 33-44
//! seat: 13-40 or 45-50
//!
//! your ticket:
//! 7,1,14
//!
//! nearby tickets:
//! 7,3,47
//! 40,4,50
//! 55,2,20
//! 38,6,12
//! ```
//!
//! It doesn't matter which position corresponds to which field; you can identify invalid nearby tickets by considering only whether tickets contain values that are not valid for any field. In this example, the values on the first nearby ticket are all valid for at least one field. This is not true of the other three nearby tickets: the values 4, 55, and 12 are are not valid for any field. Adding together all of the invalid values produces your ticket scanning error rate: 4 + 55 + 12 = 71.
//!
//! Consider the validity of the nearby tickets you scanned. What is your ticket scanning error rate?
//!
//! Your puzzle answer was 25895.
//!
//! ## --- Part Two ---
//!
//! Now that you've identified which tickets contain invalid values, discard those tickets entirely. Use the remaining valid tickets to determine which field is which.
//!
//! Using the valid ranges for each field, determine what order the fields appear on the tickets. The order is consistent between all tickets: if seat is the third field, it is the third field on every ticket, including your ticket.
//!
//! For example, suppose you have the following notes:
//!
//! ```
//! class: 0-1 or 4-19
//! row: 0-5 or 8-19
//! seat: 0-13 or 16-19
//!
//! your ticket:
//! 11,12,13
//!
//! nearby tickets:
//! 3,9,18
//! 15,1,5
//! 5,14,9
//! ```
//!
//! Based on the nearby tickets in the above example, the first position must be row, the second position must be class, and the third position must be seat; you can conclude that in your ticket, class is 12, row is 11, and seat is 13.
//!
//! Once you work out which field is which, look for the six fields on your ticket that start with the word departure. What do you get if you multiply those six values together?
//!
//! Your puzzle answer was 5865723727753.

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

type Rule = ((i32, i32), (i32, i32));
type Rules = HashMap<String, Rule>;

fn main() {
    let (rules, my_ticket, nearby_tickets) = parse_input("input.txt");

    let res_p1 = part1(&rules, &nearby_tickets);
    println!("res p1: {}", res_p1);

    let res_p2 = part2(&rules, &my_ticket, &nearby_tickets);
    println!("res p2: {}", res_p2);
}

fn parse_input(filename: &str) -> (Rules, Vec<i32>, Vec<Vec<i32>>) {
    let contents = fs::read_to_string(filename).unwrap();
    let split: Vec<String> = contents.split("\n\n").map(|s| s.to_string()).collect();

    // populate rules
    let rules: Rules = (split[0].split('\n').map(|line| {
        let mut spl = line.split(':');
        let name = spl.next().unwrap().to_string();
        let nums: ((i32, i32), (i32, i32)) = {
            let mut range_iter = spl.next().unwrap().split("or").map(|range| {
                let mut iter = range.split('-').map(|n| n.trim().parse::<i32>().unwrap());
                (iter.next().unwrap(), iter.next().unwrap())
            });

            (range_iter.next().unwrap(), range_iter.next().unwrap())
        };
        (name, nums)
    }))
    .collect();

    // populate my ticket
    let my_ticket: Vec<i32> = split[1]
        .split('\n')
        .nth(1)
        .unwrap()
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    // populate nearby_tickets
    let nearby_tickets: Vec<Vec<i32>> = split[2]
        .split('\n')
        .skip(1)
        .map(|line| {
            line.split(',')
                .map(|n| n.strip_suffix('\r').unwrap_or(n).parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    (rules, my_ticket, nearby_tickets)
}

fn part1(rules: &Rules, nearby_tickets: &[Vec<i32>]) -> i32 {
    let mut invalid = 0;

    for ticket in nearby_tickets {
        for num in ticket {
            let mut is_valid = false;
            for rule in rules.values() {
                if ticket_num_is_valid(rule, *num) {
                    is_valid = true;
                }
            }

            if !is_valid {
                invalid += num;
            }
        }
    }

    invalid
}

fn part2(rules: &Rules, my_ticket: &[i32], nearby_tickets: &[Vec<i32>]) -> u64 {
    let valid_tickets: Vec<&Vec<i32>> = nearby_tickets
        .iter()
        .filter(|&ticket| ticket_is_valid(rules, ticket))
        .collect();

    let mut matrix: Vec<(&str, HashSet<usize>)> = rules
        .iter()
        .map(|(name, rule)| {
            let mut v = HashSet::new();
            let ticket_size = valid_tickets[0].len();

            for i in 0..ticket_size {
                let rule_match = valid_tickets
                    .iter()
                    .map(|ticket| ticket_num_is_valid(rule, ticket[i]))
                    .all(|val| val);

                if rule_match {
                    v.insert(i);
                }
            }

            (name.as_str(), v)
        })
        .collect();

    print_matrix(&matrix);
    println!("\n");
    solve_matrix(&mut matrix);
    print_matrix(&matrix);

    let mut res: Vec<u64> = Vec::new();
    for (name, set) in &matrix {
        let num = *set.iter().next().unwrap();

        if name.starts_with("departure") {
            res.push(my_ticket[num] as u64);
        }
    }

    res.iter().product()
}

fn print_matrix(matrix: &[(&str, HashSet<usize>)]) {
    println!("\n");
    for (name, vals) in matrix {
        let mut line = format!("{:<20}| ", name);

        for i in 0..20 {
            if vals.contains(&(i as usize)) {
                line.push('X');
            } else {
                line.push('.');
            }
        }

        println!("{}", line);
    }
}

fn solve_matrix(matrix: &mut Vec<(&str, HashSet<usize>)>) {
    let mut finished: HashSet<&str> = HashSet::new();
    let mut i = 0;

    loop {
        // end condition
        if finished.len() == 20 {
            break;
        }

        // find rules that that only has single item in it
        let len = matrix.len();

        // remove that rule's idx from all the other ones
        if matrix[i % matrix.len()].1.len() == 1 {
            let idx = *matrix[i % matrix.len()].1.iter().next().unwrap();

            for j in 0..len {
                if j == (i % matrix.len()) {
                    continue;
                }

                matrix[j].1.remove(&idx);
            }

            finished.insert(matrix[i % matrix.len()].0);
            i += 1;
        } else {
            i += 1;
        }
    }
}

fn ticket_num_is_valid(rule: &Rule, num: i32) -> bool {
    (num >= rule.0 .0 && num <= rule.0 .1) || (num >= rule.1 .0 && num <= rule.1 .1)
}

fn ticket_is_valid(rules: &Rules, ticket: &[i32]) -> bool {
    let valid_rules: Vec<bool> = ticket
        .iter()
        .map(|num| {
            let mut is_valid = false;
            for rule in rules.values() {
                if ticket_num_is_valid(rule, *num) {
                    is_valid = true;
                }
            }
            is_valid
        })
        .collect();

    valid_rules.iter().all(|val| *val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_p1() {
        let (rules, _, nearby_tickets) = parse_input("sample.txt");
        assert_eq!(part1(&rules, &nearby_tickets), 71);
    }
}
