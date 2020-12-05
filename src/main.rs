#![crate_name = "aoc_2020"]

//! # Advent of Code (2020)
//! 
//! ## Introduction
//! 
//! Advent of Code (AOC) is crated by [Eric Wastl](http://was.tl/) and is an
//! annual advent calendar of small programming puzzles. They can be solved
//! in any language (you don't really need code at all to solve some of them)
//! and its intended for people of all skill levels.
//! 
//! ## Overview
//! 
//! This is the top-level crate for the Advent of Code (AOC) 2020. I'm using
//! this year's AOC as an opportunity to sharpen my rust skills and get more
//! familiar with the language and toolchain.
//! 
//! You'll notice that the entire project is a rust crate, so I can take
//! advantage of the fantastic [rustdoc](https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html)
//! tooling to build the entire project.
//! 
//! The way this project is structured is each day has its own crate that is
//! built as a part of this workspace (see the [Cargo.toml](https://github.com/hkennyv/aoc/blob/master/Cargo.toml)).
//! 
//! Any crate can be run by:
//! 
//! - navigating into the dayXX crate
//! - running `cargo run`
//! 
//! I try to keep most of the functions and modules documented the best I can
//! so that you can view that in these docs, however sometimes I'll rush
//! through these aoc challenges if I'm crunched on time.
//! 
//! Most crates also have some runnable tests that you can run using:
//! 
//! - `cargo test`
//! 
//! Cheers & happy coding!
//!

fn main() {
    println!("Hello, world!");
}
