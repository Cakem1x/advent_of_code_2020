use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::Read;

type BagContent = Vec<(usize, String)>;
type Ruleset = HashMap<String, BagContent>;

pub fn ruleset_from_string(bag_rules_str: &str) -> Ruleset {
    let mut ruleset = Ruleset::new();

    for rule_str in bag_rules_str.split('\n') {
        if rule_str.is_empty() {
            continue;
        }

        let (bag_name, bag_content) = bag_content_from_string(rule_str);
        ruleset.insert(bag_name, bag_content);
    }

    return ruleset;
}

pub fn bag_content_from_string(bag_rule_str: &str) -> (String, BagContent) {
    let until_end_of_rules_bag_str_pos = bag_rule_str.find(" bags contain ").unwrap();
    let rules_bag_str = &bag_rule_str[..until_end_of_rules_bag_str_pos];

    let mut contained_bags_strs = Vec::new();

    if bag_rule_str.find("no other bags.").is_none() {
        for contained_bags_rule_str in
            bag_rule_str[until_end_of_rules_bag_str_pos + " bags contain ".len()..].split(", ")
        {
            let first_space_str_pos = contained_bags_rule_str.find(' ').unwrap();
            let nr_bags: usize = contained_bags_rule_str[..first_space_str_pos]
                .parse()
                .unwrap();
            let bag_type = &contained_bags_rule_str
                [first_space_str_pos + 1..contained_bags_rule_str.find(" bag").unwrap()];
            contained_bags_strs.push((nr_bags, bag_type.to_string()));
        }
    }

    return (rules_bag_str.to_string(), contained_bags_strs);
}

/// Returns (nr_contained_bags, already_checked_bags) after recursion
fn recursive_bag_expansion(
    current_bag: &str,
    ruleset: &Ruleset,
    mut already_checked_bags: HashSet<String>,
) -> (usize, HashSet<String>) {
    let rule_to_check = &ruleset[current_bag];

    let mut nr_other_bags_contained = 0;
    for (next_bag_amount, next_bag_name) in rule_to_check {
        already_checked_bags.insert(next_bag_name.to_owned());
        let recursion_result =
            recursive_bag_expansion(&next_bag_name, ruleset, already_checked_bags);
        nr_other_bags_contained += next_bag_amount * recursion_result.0;
        already_checked_bags = recursion_result.1;
    }

    return (1 + nr_other_bags_contained, already_checked_bags);
}

pub fn get_bags_contained_recursively_part1(bag: &str, ruleset: &Ruleset) -> HashSet<String> {
    return recursive_bag_expansion(bag, ruleset, HashSet::<String>::new()).1;
    // only return set, so I don't need to fix the part1 test
}

pub fn get_bags_contained_recursively_part2(bag: &str, ruleset: &Ruleset) -> usize {
    return recursive_bag_expansion(bag, ruleset, HashSet::<String>::new()).0 - 1;
    // subtract itself
}

fn main() {
    let mut file = File::open("input").unwrap();
    let mut input_string = String::new();
    file.read_to_string(&mut input_string).unwrap();

    let ruleset = ruleset_from_string(&input_string);
    let mut shiny_gold_counter = 0;

    for bag in ruleset.keys() {
        let contained_bags = get_bags_contained_recursively_part1(bag, &ruleset);
        if contained_bags.contains("shiny gold") {
            shiny_gold_counter += 1;
            println!("Bag {} can contain a shiny gold bag", bag);
        } else {
            println!("Bag {} cannot contain a shiny gold bag", bag);
        }
    }
    println!(
        "Part 1 - {} bags can contain a shiny gold bag",
        shiny_gold_counter
    );
    println!("Part 2 - A gold bag needs to contain {} other bags.",
        get_bags_contained_recursively_part2("shiny gold", &ruleset),
    );
}

#[test]
fn test_example_part2() {
    let example_str = "light red bags contain 1 bright white bag, 2 muted yellow bags.\ndark orange bags contain 3 bright white bags, 4 muted yellow bags.\nbright white bags contain 1 shiny gold bag.\nmuted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\nshiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\ndark olive bags contain 3 faded blue bags, 4 dotted black bags.\nvibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\nfaded blue bags contain no other bags.\ndotted black bags contain no other bags.";
    let ruleset = ruleset_from_string(example_str);

    assert_eq!(
        get_bags_contained_recursively_part2("shiny gold", &ruleset),
        32
    );
}

#[test]
fn test_example_part1() {
    let example_str = "light red bags contain 1 bright white bag, 2 muted yellow bags.\ndark orange bags contain 3 bright white bags, 4 muted yellow bags.\nbright white bags contain 1 shiny gold bag.\nmuted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\nshiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\ndark olive bags contain 3 faded blue bags, 4 dotted black bags.\nvibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\nfaded blue bags contain no other bags.\ndotted black bags contain no other bags.";
    let ruleset = ruleset_from_string(example_str);
    let bags_that_contain_shiny_gold = ["bright white", "muted yellow", "dark orange", "light red"];

    for bag_name in ruleset.keys() {
        assert_eq!(
            get_bags_contained_recursively_part1(bag_name, &ruleset).contains("shiny gold"),
            bags_that_contain_shiny_gold
                .iter()
                .any(|&bag_cotaining_shiny_gold| bag_cotaining_shiny_gold == bag_name)
        );
    }
}

#[test]
fn test_parse_rules() {
    let rules_str = "light red bags contain 1 bright white bag, 2 muted yellow bags.\nbright white bags contain 1 shiny gold bag.\nfaded blue bags contain no other bags.";
    let rules = ruleset_from_string(rules_str);
    let correct_rules: Ruleset = [
        (
            "light red".to_string(),
            [
                (1, "bright white".to_string()),
                (2, "muted yellow".to_string()),
            ]
            .to_vec(),
        ),
        (
            "bright white".to_string(),
            [(1, "shiny gold".to_string())].to_vec(),
        ),
        ("faded blue".to_string(), [].to_vec()),
    ]
    .iter()
    .cloned()
    .collect();
    assert_eq!(rules, correct_rules);
}
