#![crate_name = "day02"]

//! ## --- Day 2: Dive! ---
//!
//! Now, you need to figure out how to pilot this thing.
//!
//! It seems like the submarine can take a series of commands like forward 1, down 2, or up 3:
//!
//! ```
//! forward X increases the horizontal position by X units.
//! down X increases the depth by X units.
//! up X decreases the depth by X units.
//! ```
//!
//! Note that since you're on a submarine, down and up affect your depth, and so they have the opposite result of what you might expect.
//!
//! The submarine seems to already have a planned course (your puzzle input). You should probably figure out where it's going. For example:
//!
//! ```
//! forward 5
//! down 5
//! forward 8
//! up 3
//! down 8
//! forward 2
//! ```
//!
//! Your horizontal position and depth both start at 0. The steps above would then modify them as follows:
//!
//! ```
//! forward 5 adds 5 to your horizontal position, a total of 5.
//! down 5 adds 5 to your depth, resulting in a value of 5.
//! forward 8 adds 8 to your horizontal position, a total of 13.
//! up 3 decreases your depth by 3, resulting in a value of 2.
//! down 8 adds 8 to your depth, resulting in a value of 10.
//! forward 2 adds 2 to your horizontal position, a total of 15.
//! ```
//!
//! After following these instructions, you would have a horizontal position of 15 and a depth of 10. (Multiplying these together produces 150.)
//!
//! Calculate the horizontal position and depth you would have after following the planned course. What do you get if you multiply your final horizontal position by your final depth?
//!
//! Your puzzle answer was 1484118.
//!
//! ## --- Part Two ---
//!
//! Based on your calculations, the planned course doesn't seem to make any sense. You find the submarine manual and discover that the process is actually slightly more complicated.
//!
//! In addition to horizontal position and depth, you'll also need to track a third value, aim, which also starts at 0. The commands also mean something entirely different than you first thought:
//!
//! ```
//! down X increases your aim by X units.
//! up X decreases your aim by X units.
//! forward X does two things:
//! It increases your horizontal position by X units.
//! It increases your depth by your aim multiplied by X.
//! ```
//!
//! Again note that since you're on a submarine, down and up do the opposite of what you might expect: "down" means aiming in the positive direction.
//!
//! Now, the above example does something different:
//!
//! ```
//! forward 5 adds 5 to your horizontal position, a total of 5. Because your aim is 0, your depth does not change.
//! down 5 adds 5 to your aim, resulting in a value of 5.
//! forward 8 adds 8 to your horizontal position, a total of 13. Because your aim is 5, your depth increases by 8*5=40.
//! up 3 decreases your aim by 3, resulting in a value of 2.
//! down 8 adds 8 to your aim, resulting in a value of 10.
//! forward 2 adds 2 to your horizontal position, a total of 15. Because your aim is 10, your depth increases by 2*10=20 to a total of 60.
//! ```
//!
//! After following these new instructions, you would have a horizontal position of 15 and a depth of 60. (Multiplying these produces 900.)
//!
//! Using this new interpretation of the commands, calculate the horizontal position and depth you would have after following the planned course. What do you get if you multiply your final horizontal position by your final depth?
//!
//! Your puzzle answer was 1463827010.

use std::fs;

/// All possible submarine commands
#[derive(Debug)]
enum Command {
    Forward(usize),
    Down(usize),
    Up(usize),
}

fn main() {
    let input = read_input("input.txt");
    let part1 = part1(&input);
    let part2 = part2(&input);

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}

fn part1(commands: &[Command]) -> usize {
    let pos = run_submarine_commands(commands);
    pos.0 * pos.1
}

fn part2(commands: &[Command]) -> usize {
    let pos = run_submarine_commands_with_aim(commands);
    pos.0 * pos.1
}

fn to_command(s: &str, val_str: &str) -> Command {
    match s {
        "forward" => Command::Forward(val_str.parse::<usize>().expect("invalid value")),
        "down" => Command::Down(val_str.parse::<usize>().expect("invalid value")),
        "up" => Command::Up(val_str.parse::<usize>().expect("invalid value")),
        _ => panic!("invalid command"),
    }
}

fn read_input(filename: &str) -> Vec<Command> {
    fs::read_to_string(filename)
        .expect("Failed to read input file")
        .lines()
        .map(|line: &str| {
            let mut split = line.split(' ');
            to_command(split.next().unwrap(), split.next().unwrap())
        })
        .collect()
}

/// Runs submarine commands with the following rules:
///
/// - forward X increases the horizontal position by X units.
/// - down X increases the depth by X units.
/// - up X decreases the depth by X units.
fn run_submarine_commands(commands: &[Command]) -> (usize, usize) {
    let mut x: usize = 0;
    let mut y: usize = 0;

    for command in commands {
        match command {
            Command::Forward(val) => x += val,
            Command::Down(val) => y += val,
            Command::Up(val) => y -= val,
        }
    }

    (x, y)
}

/// Runs submarine commands with aim. Uses the following rules:
///
/// - down X increases your aim by X units.
/// - up X decreases your aim by X units.
/// - forward X does two things:
///   - It increases your horizontal position by X units.
///   - It increases your depth by your aim multiplied by X.
fn run_submarine_commands_with_aim(commands: &[Command]) -> (usize, usize) {
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut aim: usize = 0;

    for command in commands {
        match command {
            Command::Forward(val) => {
                x += val;
                y += aim * val;
            }
            Command::Down(val) => aim += val,
            Command::Up(val) => aim -= val,
        }
    }

    (x, y)
}
