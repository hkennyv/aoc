#![crate_name = "day05"]

//! ## --- Day 5: Binary Boarding ---
//!
//! You board your plane only to discover a new problem: you dropped your boarding pass! You aren't sure which seat is yours, and all of the flight attendants are busy with the flood of people that suddenly made it through passport control.
//!
//! You write a quick program to use your phone's camera to scan all of the nearby boarding passes (your puzzle input); perhaps you can find your seat through process of elimination.
//!
//! Instead of zones or groups, this airline uses binary space partitioning to seat people. A seat might be specified like FBFBBFFRLR, where F means "front", B means "back", L means "left", and R means "right".
//!
//! The first 7 characters will either be F or B; these specify exactly one of the 128 rows on the plane (numbered 0 through 127). Each letter tells you which half of a region the given seat is in. Start with the whole list of rows; the first letter indicates whether the seat is in the front (0 through 63) or the back (64 through 127). The next letter indicates which half of that region the seat is in, and so on until you're left with exactly one row.
//!
//! For example, consider just the first seven characters of FBFBBFFRLR:
//!
//! ```
//! Start by considering the whole range, rows 0 through 127.
//! F means to take the lower half, keeping rows 0 through 63.
//! B means to take the upper half, keeping rows 32 through 63.
//! F means to take the lower half, keeping rows 32 through 47.
//! B means to take the upper half, keeping rows 40 through 47.
//! B keeps rows 44 through 47.
//! F keeps rows 44 through 45.
//! The final F keeps the lower of the two, row 44.
//! ```
//!
//! The last three characters will be either L or R; these specify exactly one of the 8 columns of seats on the plane (numbered 0 through 7). The same process as above proceeds again, this time with only three steps. L means to keep the lower half, while R means to keep the upper half.
//!
//! For example, consider just the last 3 characters of FBFBBFFRLR:
//!
//! ```
//! Start by considering the whole range, columns 0 through 7.
//! R means to take the upper half, keeping columns 4 through 7.
//! L means to take the lower half, keeping columns 4 through 5.
//! The final R keeps the upper of the two, column 5.
//! So, decoding FBFBBFFRLR reveals that it is the seat at row 44, column 5.
//! ```
//!
//! Every seat also has a unique seat ID: multiply the row by 8, then add the column. In this example, the seat has ID 44 \* 8 + 5 = 357.
//!
//! Here are some other boarding passes:
//!
//! ```
//! BFFFBBFRRR: row 70, column 7, seat ID 567.
//! FFFBBBFRRR: row 14, column 7, seat ID 119.
//! BBFFBBFRLL: row 102, column 4, seat ID 820.
//! ```
//!
//! As a sanity check, look through your list of boarding passes. What is the highest seat ID on a boarding pass?
//!
//! Your puzzle answer was 998.
//!
//! ## --- Part Two ---
//!
//! Ding! The "fasten seat belt" signs have turned on. Time to find your seat.
//!
//! It's a completely full flight, so your seat should be the only missing boarding pass in your list. However, there's a catch: some of the seats at the very front and back of the plane don't exist on this aircraft, so they'll be missing from your list as well.
//!
//! Your seat wasn't at the very front or back, though; the seats with IDs +1 and -1 from yours will be in your list.
//!
//! What is the ID of your seat?
//!
//! Your puzzle answer was 676.

use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let input: Vec<String> = contents.lines().map(|s| s.to_string()).collect();

    let res_p1 = input
        .iter()
        .map(|line| (get_row(line), get_column(line)))
        .map(|(row, col)| get_seat_id(row, col))
        .max()
        .unwrap();

    println!("RESULT p1: {}", res_p1);

    let res_p2 = get_my_seat_id(&input);

    println!("RESULT p2: {}", res_p2);
}

/// calculates the row from the first 7 characters of the passport. the first
/// 7 characters specify exactly one of 128 rows on the plane (numbered 0
/// through 127). the first letter indicates which half of the region the seat
/// is in (binary). think of 'B' as a 1 and 'F' as 0.
///
/// e.g. FBFBBFF = 0101100 = row 44
fn get_row(s: &String) -> i32 {
    let charmap: HashMap<char, i32> = [('F', 0), ('B', 1)].iter().cloned().collect();

    let sum: i32 = s
        .chars()
        .map(|ch| charmap.get(&ch).unwrap())
        .take(7)
        .enumerate()
        .map(|(i, ch)| ch << (6 - i))
        .sum();

    sum
}

/// calculates the column from the last 3 characters of the passport. the last
/// 3 characters specify one of 8 columns on the plane (numbered 0-7). think
/// of 'L' as 0 and 'R' as 1
///
/// e.g. LLL = 000 = col 0, RRR = 111 = col 7
fn get_column(s: &String) -> i32 {
    let charmap: HashMap<char, i32> = [('L', 0), ('R', 1)].iter().cloned().collect();

    let sum: i32 = s
        .chars()
        .rev()
        .map(|ch| charmap.get(&ch).unwrap())
        .take(3)
        .enumerate()
        .map(|(i, ch)| ch << (i))
        .sum();

    sum
}

/// calculates the seat_id from the row and column
fn get_seat_id(row: i32, col: i32) -> i32 {
    row * 8 + col
}

/// calculates _my_ seat id
fn get_my_seat_id(input: &Vec<String>) -> i32 {
    let mut ids: Vec<i32> = input
        .iter()
        .map(|line| (get_row(line), get_column(line)))
        .map(|(row, col)| get_seat_id(row, col))
        .collect();

    ids.sort();

    let diffs: Vec<i32> = ids
        .windows(2)
        .map(|slice| (slice[1] - slice[0]))
        .enumerate()
        .filter(|(i, diff)| *diff > 1)
        .map(|(idx, _)| ids[idx] + 1)
        .collect();

    *diffs.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_row() {
        let test_strings: Vec<(&str, i32)> = vec![
            ("FBFBBFFRLR", 44),
            ("BFFFBBFRRR", 70),
            ("FFFBBBFRRR", 14),
            ("BBBBFFFRRL", 120),
        ];

        for (s, row) in test_strings.iter() {
            assert_eq!(get_row(&s.to_string()), *row);
        }
    }

    #[test]
    fn test_get_column() {
        let test_strings: Vec<(&str, i32)> = vec![
            ("FBFBBFFRLR", 5),
            ("BFFFBBFRRR", 7),
            ("FFFBBBFRRR", 7),
            ("BBBBFFFRRL", 6),
        ];

        for (s, row) in test_strings.iter() {
            assert_eq!(get_column(&s.to_string()), *row);
        }
    }

    #[test]
    fn test_get_seat_id() {
        let test_inputs = vec![(44, 3, 355), (12, 2, 98), (127, 0, 1016)];

        for (row, col, res) in test_inputs {
            assert_eq!(get_seat_id(row, col), res);
        }
    }
}
