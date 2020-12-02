use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let input: Vec<i32> = contents
        .split('\n')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    let unique_inputs: HashSet<i32> = input.iter().cloned().collect();

    println!("input: {:?}\n", input);
    println!("unique input: {:?}\n", unique_inputs);

    // find compliment
    for val in unique_inputs.iter() {
        let compliment = 2020 - val;
        if unique_inputs.contains(&compliment) {
            println!("RESULT: {} x {} = {}", compliment, val, compliment * val);
        }
    }
}
