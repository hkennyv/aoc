use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("file cannot be found");
    let mut rucksacks = input.lines();

    let p1 = &rucksacks
        .map(split_half)
        .map(|(s1, s2)| find_common2(s1, s2))
        .map(calculate_priority)
        .sum::<usize>();

    println!("part1: {p1}");

    rucksacks = input.lines();

    let p2 = rucksacks
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|s| find_common3(s[0], s[1], s[2]))
        .map(calculate_priority)
        .sum::<usize>();

    println!("part2: {p2}");
}

fn split_half(s: &str) -> (&str, &str) {
    let l = s.len() / 2;
    (&s[0..l], &s[l..])
}

fn find_common3(s1: &str, s2: &str, s3: &str) -> char {
    let mut h1 = HashSet::new();
    let mut h2 = HashSet::new();
    let mut h3 = HashSet::new();

    for c in s1.chars() {
        h1.insert(c);
    }

    for c in s2.chars() {
        h2.insert(c);
    }

    for c in s3.chars() {
        h3.insert(c);
    }

    let i1: HashSet<_> = h1.intersection(&h2).collect();
    let i2: HashSet<_> = h1.intersection(&h3).collect();

    let res = i1.intersection(&i2).collect::<Vec<&&char>>();

    **res[0]
}

fn find_common2(s1: &str, s2: &str) -> char {
    let mut h1 = HashSet::new();
    let mut h2 = HashSet::new();

    for c in s1.chars() {
        h1.insert(c);
    }

    for c in s2.chars() {
        h2.insert(c);
    }

    let res = h1.intersection(&h2).collect::<Vec<&char>>();

    *res[0]
}

fn calculate_priority(c: char) -> usize {
    if !('A'..='z').contains(&c) {
        panic!("out of range priority");
    }

    if ('a'..='z').contains(&c) {
        (c as usize - 'a' as usize) as usize + 1
    } else {
        (c as usize - 'A' as usize) as usize + 27
    }
}
