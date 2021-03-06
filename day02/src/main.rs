use std::fs::File;
use std::io::{self, BufRead};

type Password = String;

#[derive(Debug, PartialEq)]
struct Policy {
    required_char: char,
    required_min: usize,
    required_max: usize,
}

#[derive(Debug, PartialEq)]
pub struct PasswordWithPolicy {
    policy: Policy,
    password: Password,
}

impl PasswordWithPolicy {
    pub fn from_string(password_with_policy_string: &str) -> PasswordWithPolicy {
        let mut pw_with_pol_string_parts = password_with_policy_string.split(": ");
        PasswordWithPolicy {
            policy: Policy::from_string(pw_with_pol_string_parts.next().unwrap()),
            password: pw_with_pol_string_parts.next().unwrap().to_string(),
        }
    }

    pub fn is_valid_part2(&self) -> bool {
        let chars = self.password.chars().collect::<Vec<char>>();
        let req_char_at_min = chars[self.policy.required_min - 1] == self.policy.required_char;
        let req_char_at_max = chars[self.policy.required_max - 1] == self.policy.required_char;
        //println!("at chars[{}]: {:?} -> valid = {}", self.policy.required_min - 1, chars[self.policy.required_min - 1], valid);
        //println!("at chars[{}]: {:?} -> valid = {}", self.policy.required_max - 1, chars[self.policy.required_max - 1], valid);

        return (!req_char_at_max && req_char_at_min) || (req_char_at_max && !req_char_at_min)
    }

    pub fn is_valid_part1(&self) -> bool {
        let occurrence_count = self
            .password
            .split(self.policy.required_char)
            .collect::<Vec<&str>>()
            .len()
            - 1; // -1, because the one element will always exist even if there's no occurrence of the required_char
        return self.policy.required_min <= occurrence_count
            && occurrence_count <= self.policy.required_max;
    }
}

impl Policy {
    fn from_string(policy_string: &str) -> Policy {
        let mut policy_parts = policy_string.split(' ');
        let mut required_range = policy_parts.next().unwrap().split("-");
        Policy {
            required_char: policy_parts.next().unwrap().parse().unwrap(),
            required_min: required_range.next().unwrap().parse().unwrap(),
            required_max: required_range.next().unwrap().parse().unwrap(),
        }
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut valid_count_part1 = 0;
    let mut valid_count_part2 = 0;

    for entry in reader.lines() {
        let pw = PasswordWithPolicy::from_string(&entry.unwrap());
        if pw.is_valid_part1() {
            valid_count_part1 += 1;
        }
        if pw.is_valid_part2() {
            valid_count_part2 += 1;
        }
    }
    println!("Part 1 valid passwords: {}", valid_count_part1);
    println!("Part 2 valid passwords: {}", valid_count_part2);
}

#[test]
fn test_part1_simple_example() {
    let test_strings = ["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];
    let p0 = PasswordWithPolicy::from_string(test_strings[0]);
    let p1 = PasswordWithPolicy::from_string(test_strings[1]);
    let p2 = PasswordWithPolicy::from_string(test_strings[2]);
    assert_eq!(p0.is_valid_part1(), true);
    assert_eq!(p1.is_valid_part1(), false);
    assert_eq!(p2.is_valid_part1(), true);
}

#[test]
fn test_part2_simple_example() {
    let test_strings = ["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];
    let p0 = PasswordWithPolicy::from_string(test_strings[0]);
    let p1 = PasswordWithPolicy::from_string(test_strings[1]);
    let p2 = PasswordWithPolicy::from_string(test_strings[2]);
    assert_eq!(p0.is_valid_part2(), true);
    assert_eq!(p1.is_valid_part2(), false);
    assert_eq!(p2.is_valid_part2(), false);
}

#[test]
fn test_policy_from_string() {
    let p = Policy::from_string("1-3 a");
    assert_eq!(
        p,
        Policy {
            required_char: 'a',
            required_min: 1,
            required_max: 3
        }
    );
}

#[test]
fn test_password_with_policy_from_string() {
    let password_with_policy = PasswordWithPolicy::from_string("1-3 a: abcde");
    assert_eq!(
        password_with_policy,
        PasswordWithPolicy {
            policy: Policy {
                required_char: 'a',
                required_min: 1,
                required_max: 3
            },
            password: "abcde".to_string()
        }
    );

    let password_with_policy = PasswordWithPolicy::from_string("1-3 b: cdefg");
    assert_eq!(
        password_with_policy,
        PasswordWithPolicy {
            policy: Policy {
                required_char: 'b',
                required_min: 1,
                required_max: 3
            },
            password: "cdefg".to_string()
        }
    );

    let password_with_policy = PasswordWithPolicy::from_string("2-9 c: ccccccccc");
    assert_eq!(
        password_with_policy,
        PasswordWithPolicy {
            policy: Policy {
                required_char: 'c',
                required_min: 2,
                required_max: 9
            },
            password: "ccccccccc".to_string()
        }
    );
}
