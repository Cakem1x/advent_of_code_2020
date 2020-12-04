use std::fs::File;
use std::io;
use std::io::prelude::*;

pub struct Passport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

impl Passport {
    pub fn from_string(passport_string: &str) -> Passport {
        let mut p = Passport {
            birth_year: None,
            issue_year: None,
            expiration_year: None,
            height: None,
            hair_color: None,
            eye_color: None,
            passport_id: None,
            country_id: None,
        };

        for key_value_pair in passport_string.split_whitespace() {
            let split_key_value_pair: Vec<&str> = key_value_pair.split(':').collect();
            assert!(split_key_value_pair.len() == 2);
            let key = split_key_value_pair[0];
            let value = split_key_value_pair[1];
            match key {
                "byr" => p.birth_year = Some(value.to_string()),
                "iyr" => p.issue_year = Some(value.to_string()),
                "eyr" => p.expiration_year = Some(value.to_string()),
                "hgt" => p.height = Some(value.to_string()),
                "hcl" => p.hair_color = Some(value.to_string()),
                "ecl" => p.eye_color = Some(value.to_string()),
                "pid" => p.passport_id = Some(value.to_string()),
                "cid" => p.country_id = Some(value.to_string()),
                _ => panic!(),
            }
        }

        return p;
    }

    pub fn is_valid_part1(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passport_id.is_some()
    }
}

fn main() -> io::Result<()> {
    let mut file = File::open("input").unwrap();
    let mut input_string = String::new();
    file.read_to_string(&mut input_string)?;

    let mut nr_valid_passports = 0;

    for passport_string in input_string.split("\n\n") {
        if Passport::from_string(passport_string).is_valid_part1() {
            nr_valid_passports += 1;
        }
    }

    println!("Part1: Nr valid passports: {}", nr_valid_passports);

    Ok(())
}

#[test]
fn test_from_string_example() {
    let passport_string =
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm";
    let passport = Passport::from_string(passport_string);
    assert_eq!(passport.eye_color, Some("gry".to_string()));
    assert_eq!(passport.passport_id, Some("860033327".to_string()));
    assert_eq!(passport.expiration_year, Some("2020".to_string()));
    assert_eq!(passport.hair_color, Some("#fffffd".to_string()));
    assert_eq!(passport.birth_year, Some("1937".to_string()));
    assert_eq!(passport.issue_year, Some("2017".to_string()));
    assert_eq!(passport.country_id, Some("147".to_string()));
    assert_eq!(passport.height, Some("183cm".to_string()));
}

#[test]
fn test_example() {
    let mut nr_valid = 0;
    let input_string = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm\n\niyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929\n\nhcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm\n\nhcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in";
    for passport_string in input_string.split("\n\n") {
        if Passport::from_string(passport_string).is_valid_part1() {
            nr_valid += 1;
        }
    }
    assert_eq!(nr_valid, 2);
}
