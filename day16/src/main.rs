#![crate_name = "day16"]

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;

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
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    (rules, my_ticket, nearby_tickets)
}

fn part1(rules: &Rules, nearby_tickets: &[Vec<i32>]) -> i32 {
    let mut invalid = 0;
    let mut count = 0;

    for ticket in nearby_tickets {
        for num in ticket {
            let mut is_valid = false;
            for rule in rules.values() {
                if ticket_num_is_valid(rule, *num) {
                    is_valid = true;
                }
            }

            if !is_valid {
                count += 1;
                invalid += num;
            }
        }
    }

    invalid
}

fn part2(rules: &Rules, my_ticket: &[i32], nearby_tickets: &[Vec<i32>]) -> i32 {
    let valid_tickets: Vec<&Vec<i32>> = nearby_tickets
        .iter()
        .filter(|&ticket| ticket_is_valid(rules, ticket))
        .collect();

    // let rule_matrix: Vec<Vec<bool>> = nearby_tickets
    //     .iter()
    //     .map(|ticket| {
    //         ticket
    //             .iter()
    //             .map(|num| {
    //                 let mut is_valid = false;
    //                 for rule in rules.values() {
    //                     if ticket_num_is_valid(rule, *num) {
    //                         is_valid = true;
    //                     }
    //                 }

    //                 is_valid
    //             })
    //             .collect()
    //     })
    //     .collect();

    // println!("{:?}", rules.keys());
    // println!("{:?}", rule_matrix);

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
    solve_matrix(&mut matrix);
    print_matrix(&matrix);

    // println!("{:?}", matrix);

    0
}

fn print_matrix(matrix: &[(&str, HashSet<usize>)]) {
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

fn solve_matrix(matrix: &mut [(&str, HashSet<usize>)]) {
    let mut finished: HashSet<&str> = HashSet::new();
    let mut i = 0;

    loop {
        // end condition
        if finished.len() == 20 {
            break;
        }

        println!("{}", matrix[i % matrix.len()].0);

        // find rules that that only has single item in it
        let len = matrix.len();
        // let rule = &mut matrix[i % matrix.len()];

        // remove that rule's idx from all the other ones
        if matrix[i % matrix.len()].1.len() == 1 {
            // if rule.1.len() == 1 {
            let idx = *matrix[i % matrix.len()].1.iter().next().unwrap();

            for j in 0..len {
                if j == i {
                    continue;
                }

                matrix[j].1.remove(&idx);
            }

            finished.insert(matrix[i % matrix.len()].0);
            // println!("\n\n");
            // print_matrix(&matrix);
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
