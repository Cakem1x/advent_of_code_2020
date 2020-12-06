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

    println!("Part1: Sum of 'yes' counts: {}", questions_yes.iter().sum::<usize>());

    Ok(())
}

#[test]
fn test_example() {
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
    assert_eq!(questions_yes, [3,3,3,1,1]);
}
