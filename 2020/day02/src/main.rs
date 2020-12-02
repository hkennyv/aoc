fn main() {
    println!("Hello, world!");
    parse_input_row(String::from("1-3 a: abcde"));
}

fn password_is_valid(policy: String, password: String) -> bool {
    true
}

/// takes in a row in the format:
///     policy: password
/// and it returns a tuple containing the (policy, password)
fn parse_input_row(row: String) -> (String, String) {
    let split: Vec<&str> = row.split(":").collect();
    let policy = split[0];
    let password = split[1];

    println!("policy: {}, password: {}", policy, password);

    (String::from(""), String::from(""))
}

fn parse_policy(policy: String) -> (i32, i32, char) {
    let split = policy.split(" ");
    ()
}