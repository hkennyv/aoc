use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let input: Vec<i32> = contents
        .split('\n')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    let res_pt1 = part1(&input).unwrap();
    println!("Part 1 result: {}\n", res_pt1);

    let res_pt2 = part2(&input).unwrap();
    println!("Part 2 result: {}\n", res_pt2);
}

fn part1(input: &Vec<i32>) -> Result<i32, String> {
    let unique_inputs: HashSet<i32> = input.iter().cloned().collect();

    // find compliment
    for val in unique_inputs.iter() {
        let compliment = 2020 - val;
        if unique_inputs.contains(&compliment) {
            println!("RESULT: {} + {} = 2020", compliment, val);
            println!("\t{} x {} = {}", compliment, val, compliment * val);
            return Ok(compliment * val);
        }
    }

    Err(String::from("no result :(\n"))
}

fn part2(input: &Vec<i32>) -> Result<i32, String> {
    // clone and sort input
    let mut input = input.clone();
    input.sort();

    // find 3 numbers that add up to 2020
    // use 3 indices to track our place in the vec (2 on the left, 1 on the
    // right)
    let target = 2020;

    let mut i1 = 0;
    let mut i2 = 1;
    let mut i3 = input.len() - 1;

    loop {
        let n1 = input[i1];
        let n2 = input[i2];
        let n3 = input[i3];

        let sum = n1 + n2 + n3;

        match sum.cmp(&target) {
            // if the sum is less than the target, we want to increment one of
            // the left indices to get a higher sum
            Ordering::Less => {
                if (i2 - i1) > 1 {
                    i1 += 1;
                } else {
                    i2 += 1;
                }

                if !assert_indices_havent_crossed(i1, i2, i3) {
                    return Err(String::from("no result :(\n"));
                }
            }

            // if the sum is greater than the target, we want to decrement the
            // right index to get a smaller number to lower our sum
            Ordering::Greater => {
                i3 -= 1;

                if !assert_indices_havent_crossed(i1, i2, i3) {
                    return Err(String::from("no result :(\n"));
                }
            }

            // if the sum is equal, then we have found a result!!
            Ordering::Equal => {
                println!("RESULT: {} + {} + {} = {}", n1, n2, n3, target);
                println!("\t{} x {} x {} = {}", n1, n2, n3, n1 * n2 * n3);
                return Ok(n1 * n2 * n3);
            }
        }
    }
}

fn assert_indices_havent_crossed(i1: usize, i2: usize, i3: usize) -> bool {
    (i1 < i2) && (i2 < i3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assert_indices_havent_crossed() {
        assert_eq!(assert_indices_havent_crossed(1, 2, 3), true);
        assert_eq!(assert_indices_havent_crossed(5, 6, 13), true);
        assert_eq!(assert_indices_havent_crossed(0, 2, 3), true);
        assert_eq!(assert_indices_havent_crossed(6, 1000, 1001), true);
    }

    #[test]
    fn test_assert_indices_havent_crossed_fails() {
        assert_eq!(assert_indices_havent_crossed(2, 2, 3), false);
        assert_eq!(assert_indices_havent_crossed(1, 2, 2), false);
        assert_eq!(assert_indices_havent_crossed(5, 3, 1), false);
        assert_eq!(assert_indices_havent_crossed(1, 1, 1), false);
    }
}
