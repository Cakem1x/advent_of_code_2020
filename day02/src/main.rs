type Password = String;

#[derive(Debug, PartialEq)]
struct Policy {
    required_char: char,
    required_min: u32,
    required_max: u32,
}

#[derive(Debug, PartialEq)]
struct PasswordWithPolicy {
    policy: Policy,
    password: Password,
}

impl PasswordWithPolicy {
    fn from_string(password_with_policy_string: &'static str) -> PasswordWithPolicy {
        let mut pw_with_pol_string_parts = password_with_policy_string.split(": ");
        PasswordWithPolicy {
            policy: Policy::from_string(pw_with_pol_string_parts.next().unwrap()),
            password: pw_with_pol_string_parts.next().unwrap().to_string(),
        }
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
    println!("Hello, world!");
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
fn test_part1_simple_example() {
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
