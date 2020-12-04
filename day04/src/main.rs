use std::fs::File;
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
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

    pub fn is_valid_part2(&self) -> bool {
        if !self.is_valid_birth_year() {
            println!("invalid birth year: {:?}", self.birth_year);
            return false;
        }
        if !self.is_valid_issue_year() {
            println!("invalid issue year: {:?}", self.issue_year);
            return false;
        }
        if !self.is_valid_expiration_year() {
            println!("invalid expiration year: {:?}", self.expiration_year);
            return false;
        }
        if !self.is_valid_height() {
            println!("invalid height: {:?}", self.height);
            return false;
        }
        if !self.is_valid_hair_color() {
            println!("invalid hair color{:?}", self.hair_color);
            return false;
        }
        if !self.is_valid_eye_color() {
            println!("invalid eye color: {:?}", self.eye_color);
            return false;
        }
        if !self.is_valid_passport_id() {
            println!("invalid passport id: {:?}", self.passport_id);
            return false;
        }
        true
    }

    fn is_valid_birth_year(&self) -> bool {
        // byr (Birth Year) - four digits; at least 1920 and at most 2002.
        return match &self.birth_year {
            Some(byr_str) => match byr_str.parse::<u32>() {
                Ok(byr_int) => 1920 <= byr_int && byr_int <= 2002,
                Err(_) => {
                    println!("invalid birth year: {:?}", self.birth_year);
                    false
                }
            },
            None => {
                println!("invalid birth year: {:?}", self.birth_year);
                false
            }
        };
    }

    fn is_valid_issue_year(&self) -> bool {
        // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        return match &self.issue_year {
            Some(iyr_str) => match iyr_str.parse::<u32>() {
                Ok(iyr_int) => 2010 <= iyr_int && iyr_int <= 2020,
                Err(_) => {
                    println!("invalid issue year: {:?}", self.issue_year);
                    false
                }
            },
            None => {
                println!("invalid issue year: {:?}", self.issue_year);
                false
            }
        };
    }

    fn is_valid_expiration_year(&self) -> bool {
        // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        return match &self.expiration_year {
            Some(eyr_str) => match eyr_str.parse::<u32>() {
                Ok(eyr_int) => 2020 <= eyr_int && eyr_int <= 2030,
                Err(_) => {
                    println!("invalid exp year: {:?}", self.expiration_year);
                    false
                }
            },
            None => {
                println!("invalid exp year: {:?}", self.expiration_year);
                false
            }
        };
    }

    fn is_valid_height(&self) -> bool {
        // hgt (Height) - a number followed by either cm or in:
        //     If cm, the number must be at least 150 and at most 193.
        //     If in, the number must be at least 59 and at most 76.
        match &self.height {
            Some(height_str) => {
                let unit = &height_str[height_str.len() - 2..];
                let value = &height_str[..height_str.len() - 2]
                    .parse::<u32>()
                    .unwrap_or(0);
                match unit {
                    "cm" => 150 <= *value && *value <= 193,
                    "in" => 59 <= *value && *value <= 76,
                    _ => {
                        println!("invalid height unit: {}", unit);
                        false
                    }
                }
            }
            None => {
                println!("invalid height: {:?}", self.height);
                false
            }
        }
    }

    fn is_valid_hair_color(&self) -> bool {
        // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        match &self.hair_color {
            Some(hcl_str) => match hcl_str.find("#") {
                Some(id) => match hcl_str[id + 1..].len() {
                    6 => match u32::from_str_radix(&hcl_str[id + 1..], 16) {
                        Ok(_color) => true,
                        Err(e) => {
                            println!("{}", e);
                            false
                        }
                    },
                    _ => {
                        println!("invalid haircolor: {:?}", self.hair_color);
                        false
                    }
                },
                None => false,
            },
            None => {
                println!("invalid haircolor: {:?}", self.hair_color);
                false
            }
        }
    }

    fn is_valid_eye_color(&self) -> bool {
        // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        match &self.eye_color {
            Some(ecl_str) => {
                if ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&(ecl_str as &str)) {
                    true
                } else {
                    println!("invalid eyecolor: {:?}", self.eye_color);
                    false
                }
            }
            None => {
                println!("invalid eyecolor: {:?}", self.eye_color);
                false
            }
        }
    }

    fn is_valid_passport_id(&self) -> bool {
        // pid (Passport ID) - a nine-digit number, including leading zeroes.
        match &self.passport_id {
            Some(pid_str) => {
                if pid_str.chars().collect::<Vec<char>>().len() == 9 {
                    match pid_str.parse::<u32>() {
                        Ok(_some_value) => true,
                        Err(_) => {
                            println!("invalid passport id: {:?}", self.passport_id);
                            false
                        }
                    }
                } else {
                    println!("invalid passport id: {:?}", self.passport_id);
                    false
                }
            }
            None => {
                println!("invalid passport id: {:?}", self.passport_id);
                false
            }
        }
    }
}

fn main() -> io::Result<()> {
    let mut file = File::open("input").unwrap();
    let mut input_string = String::new();
    file.read_to_string(&mut input_string)?;

    let mut nr_valid_passports_part1 = 0;
    let mut nr_valid_passports_part2 = 0;

    for passport_string in input_string.split("\n\n") {
        let passport = Passport::from_string(passport_string);
        if passport.is_valid_part1() {
            nr_valid_passports_part1 += 1;
        }
        if passport.is_valid_part2() {
            nr_valid_passports_part2 += 1;
        }
    }

    println!("Part1: Nr valid passports: {}", nr_valid_passports_part1);
    println!("Part2: Nr valid passports: {}", nr_valid_passports_part2);

    Ok(())
}

#[test]
fn test_part2_invalid_examples() {
    let invalid_passports_string = "eyr:1972 cid:100\nhcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926\n\niyr:2019\nhcl:#602927 eyr:1967 hgt:170cm\necl:grn pid:012533040 byr:1946\n\nhcl:dab227 iyr:2012\necl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277\n\nhgt:59cm ecl:zzz\neyr:2038 hcl:74454a iyr:2023\npid:3556412378 byr:2007";

    for invalid_passport_string in invalid_passports_string.split("\n\n") {
        let invalid_passport = Passport::from_string(invalid_passport_string);
        println!("Checking invalid passport {:?}", invalid_passport);
        assert_eq!(invalid_passport.is_valid_part2(), false);
    }
}

#[test]
fn test_part2_valid_examples() {
    let valid_passports_string = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\nhcl:#623a2f\n\neyr:2029 ecl:blu cid:129 byr:1989\niyr:2014 pid:896056539 hcl:#a97842 hgt:165cm\n\nhcl:#888785\nhgt:164cm byr:2001 iyr:2015 cid:88\npid:545766238 ecl:hzl\neyr:2022\n\niyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    for valid_passport_string in valid_passports_string.split("\n\n") {
        let valid_passport = Passport::from_string(valid_passport_string);
        println!("Checking valid passport {:?}", valid_passport);
        assert_eq!(valid_passport.is_valid_part2(), true);
    }
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
