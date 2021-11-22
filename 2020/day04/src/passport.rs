use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

pub type Passport = HashMap<String, String>;

/// a trait that exposes methods to validate the passport for part1 and part2
pub trait ValidatePassport {
    /// method that validates a passport based on p1 criteria
    fn is_valid_p1(&self) -> bool;

    /// method that validates a passport based on p2 criteria
    fn is_valid_p2(&self) -> bool;

    /// validates birth year is a number between 1920 and 2002
    fn validate_byr(byr: String) -> bool;

    /// validates issue year is a number between 2010 and 2020
    fn validate_iyr(iyr: String) -> bool;

    /// validates expiration year is between 2020 and 2030
    fn validate_eyr(eyr: String) -> bool;

    /// validates hgt is a number followed by either `cm` or `in`. if cm, the
    /// number must be at least 150 and at most 193. if in, the number must be
    /// 59 and at most 76.
    fn validate_hgt(hgt: String) -> bool;

    /// validates hcl is a hex color that starts with a '#' followed by exactly
    /// six characters 0-9 or a-f or A-F
    fn validate_hcl(hcl: String) -> bool;

    /// validates ecl is one of "amb", "blu", "brn", "gry", "grn", "hzl", "oth"
    fn validate_ecl(ecl: String) -> bool;

    /// validates pid is a nine digit number including leading zeroes
    fn validate_pid(pid: String) -> bool;

    /// validates cid, automatically true if it exists
    fn validate_cid(cid: String) -> bool;
}

impl ValidatePassport for Passport {
    fn is_valid_p1(&self) -> bool {
        let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        // let optional_fields = vec!["cid"];

        for field in required_fields {
            // if !passport.contains_key(&String::from(field)) {
            if !self.contains_key(field) {
                return false;
            }
        }

        true
    }
    fn is_valid_p2(&self) -> bool {
        let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        let validation_fns: Vec<fn(String) -> bool> = vec![
            Self::validate_byr,
            Self::validate_iyr,
            Self::validate_eyr,
            Self::validate_hgt,
            Self::validate_hcl,
            Self::validate_ecl,
            Self::validate_pid,
        ];
        // let optional_fields = vec!["cid"];

        for i in 0..required_fields.len() {
            let field = required_fields[i];
            let validation_fn = validation_fns[i];
            let val = self.get(field);

            match val {
                Some(s) => {
                    if !validation_fn(s.to_string()) {
                        return false;
                    }
                }
                None => return false,
            }
        }

        true
    }

    fn validate_byr(byr: String) -> bool {
        let year: i32 = byr.parse().unwrap_or(-1);
        (1920..=2002).contains(&year)
    }

    fn validate_iyr(iyr: String) -> bool {
        let year: i32 = iyr.parse().unwrap_or(-1);
        (2010..=2020).contains(&year)
    }

    fn validate_eyr(eyr: String) -> bool {
        let year: i32 = eyr.parse().unwrap_or(-1);
        (2020..=2030).contains(&year)
    }

    fn validate_hgt(hgt: String) -> bool {
        let re = Regex::new(r"^(\d+)(cm|in)$").unwrap();

        match re.captures(&hgt) {
            Some(captures) => {
                let val: i32 = captures
                    .get(1)
                    .unwrap()
                    .as_str()
                    .to_string()
                    .parse()
                    .unwrap_or(-1);
                let unit = captures.get(2).unwrap().as_str();

                match unit {
                    "cm" => (150..=193).contains(&val),
                    "in" => (59..=76).contains(&val),
                    _ => false,
                }
            }
            None => false,
        }
    }

    fn validate_hcl(hcl: String) -> bool {
        let re = Regex::new(r"^#[\daA-fF]{6}$").unwrap();

        re.is_match(&hcl)
    }

    fn validate_ecl(ecl: String) -> bool {
        let valid_eyecolors: HashSet<&str> =
            HashSet::from_iter(vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]);

        valid_eyecolors.contains(ecl.as_str())
    }

    fn validate_pid(pid: String) -> bool {
        let re = Regex::new(r"^\d{9}$").unwrap();

        re.is_match(&pid)
    }

    #[allow(unused_variables)]
    fn validate_cid(cid: String) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_validate_hgt() {
        assert!(!Passport::validate_hgt("120cm".to_string()));
        assert!(Passport::validate_hgt("150cm".to_string()));
        assert!(Passport::validate_hgt("193cm".to_string()));
        assert!(!Passport::validate_hgt("200cm".to_string()));

        assert!(Passport::validate_hgt("59in".to_string()));
        assert!(Passport::validate_hgt("76in".to_string()));
        assert!(!Passport::validate_hgt("77in".to_string()));
        assert!(!Passport::validate_hgt("7in".to_string()));
    }

    #[test]
    fn test_validate_ecl() {
        let valid_eyecolors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        let invalid_eyecolors = vec!["wut", "helo", "goodbye"];

        for color in valid_eyecolors {
            assert!(Passport::validate_ecl(color.to_string()));
        }

        for color in invalid_eyecolors {
            assert!(!Passport::validate_ecl(color.to_string()));
        }
    }

    #[test]
    fn test_validate_pid() {
        let valid_pids = vec!["000111222", "000234222", "123456000", "000398498"];
        let invalid_pids = vec!["1234567", "9999999"];

        for pid in valid_pids {
            assert!(Passport::validate_pid(pid.to_string()));
        }

        for pid in invalid_pids {
            assert!(!Passport::validate_pid(pid.to_string()));
        }
    }
}
