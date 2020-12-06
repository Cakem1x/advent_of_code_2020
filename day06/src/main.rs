use std::fs::File;
use std::io;
use std::io::prelude::Read;

fn main() -> io::Result<()> {
    let mut file = File::open("input").unwrap();
    let mut input_string = String::new();
    file.read_to_string(&mut input_string)?;

    let mut questions_yes = Vec::<usize>::new();
    for group_str in input_string.split("\n\n") {
        let group_str = group_str.split_whitespace().collect::<String>();
        let mut group_chars_sorted: Vec<char> = group_str.chars().collect();
        group_chars_sorted.sort();
        group_chars_sorted.dedup();
        questions_yes.push(group_chars_sorted.len());
    }

    println!(
        "Part1: Sum of 'yes' counts: {}",
        questions_yes.iter().sum::<usize>()
    );

    //part2
    let mut group_answers_bitwise_and = Vec::<u32>::new();

    for group_str in input_string.split("\n\n") {
        let mut answers_binary = Vec::<u32>::new();
        //answers_binary.push(2_u32.pow(26)-1); // start value for bitwise and; use for debug output

        for single_answer_str in group_str.split("\n") {
            if single_answer_str.is_empty() {
                continue;
            }
            // convert char iterator to u32, interpreting each existing char (answer) as setting the respective bit to 1.
            let single_answer_binary =
                single_answer_str
                    .chars()
                    .fold(0, |answer_bin, answer_char| {
                        answer_bin | (1 << answer_char as u32 - 'a' as u32) // lsb: a, msb: z
                    });
            answers_binary.push(single_answer_binary);
        }

        group_answers_bitwise_and.push(
            answers_binary
                .iter()
                .fold(2_u32.pow(26) - 1, |acc, answer| acc & answer),
        );
    }

    let group_yes = group_answers_bitwise_and
        .iter()
        .map(|group_answer| group_answer.count_ones())
        .collect::<Vec<u32>>();
    println!(
        "Part2: Sum of 'yes' counts: {}",
        group_yes.iter().sum::<u32>()
    );

    Ok(())
}

#[test]
fn test_example_part2() {
    let input_string = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";

    // this time, with bit fiddling!
    let mut group_answers_bitwise_and = Vec::<u32>::new();

    for group_str in input_string.split("\n\n") {
        println!("checking grp:\n{}", group_str);
        let mut answers_binary = Vec::<u32>::new();
        //answers_binary.push(2_u32.pow(26)-1); // start value for bitwise and; use for debug output

        for single_answer_str in group_str.split("\n") {
            if single_answer_str.is_empty() {
                continue;
            }
            println!("\tChecking answer: {}", single_answer_str);
            // convert char iterator to u32, interpreting each existing char (answer) as setting the respective bit to 1.
            let single_answer_binary =
                single_answer_str
                    .chars()
                    .fold(0, |answer_bin, answer_char| {
                        answer_bin | (1 << answer_char as u32 - 'a' as u32) // lsb: a, msb: z
                    });
            answers_binary.push(single_answer_binary);
        }
        for bit_answer in answers_binary.iter() {
            println!("Bitwise answer:       {:#032b}", bit_answer);
        }

        group_answers_bitwise_and.push(
            answers_binary
                .iter()
                .fold(2_u32.pow(26) - 1, |acc, answer| acc & answer),
        );
        println!(
            "Final group's answer: {:#032b}",
            group_answers_bitwise_and.last().unwrap()
        );
    }

    let group_yes = group_answers_bitwise_and
        .iter()
        .map(|group_answer| group_answer.count_ones())
        .collect::<Vec<u32>>();
    assert_eq!(group_yes, [3, 0, 1, 1, 1]);
}

#[test]
fn test_example_part1() {
    let input_str = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";
    let mut questions_yes = Vec::<usize>::new();
    for group_str in input_str.split("\n\n") {
        let group_str = group_str.split_whitespace().collect::<String>();
        println!("checking grp {}", group_str);
        let mut group_chars_sorted: Vec<char> = group_str.chars().collect();
        group_chars_sorted.sort();
        group_chars_sorted.dedup();
        questions_yes.push(group_chars_sorted.len());
    }
    assert_eq!(questions_yes, [3, 3, 3, 1, 1]);
}
