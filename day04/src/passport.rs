use std::collections::HashMap;

pub type Passport = HashMap<String, String>;

trait ValidatePassport {
    fn is_valid_p1(&self) -> bool;
    fn is_valid_p2(&self) -> bool;
}

impl ValidatePassport for Passport {
    /// returns true if the passport contains all of the required fields
    fn is_valid_p1(&self) -> bool {
        let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        let optional_fields = vec!["cid"];

        for field in required_fields {
            // if !passport.contains_key(&String::from(field)) {
            if !self.contains_key(field) {
                return false;
            }
        }

        true
    }
    fn is_valid_p2(&self) -> bool {
        true
    }
}
