use std::fs::File;
use std::io;
use std::io::prelude::*;
#[cfg(test)]
use itertools::izip;

pub fn split_str(input_str: &str) -> (&str, &str) {
    let rows_str = &input_str[..7];
    let columns_str = &input_str[7..];
    (rows_str, columns_str)
}

pub fn rows_to_binary_str(rows_str: &str) -> String {
    rows_str.replace("F", "0").replace("B", "1")
}

pub fn columns_to_binary_str(columns_str: &str) -> String {
    columns_str.replace("L", "0").replace("R", "1")
}

pub fn parse_binary_str(binary_str: &str) -> u32 {
    u32::from_str_radix(binary_str, 2).unwrap()
}

fn main() -> io::Result<()> {
    let mut file = File::open("input").unwrap();
    let mut input_string = String::new();
    file.read_to_string(&mut input_string)?;

    // for part 1
    let mut highest_seat_id = 0;
    // for part 2
    let mut seats_occupied = [false; 1024];

    for line in input_string.lines()
    {
        let (row_str, column_str) = split_str(line);
        let row = parse_binary_str(&rows_to_binary_str(row_str));
        let column = parse_binary_str(&columns_to_binary_str(column_str));
        let seat_id = row * 8 + column;

        // part 1
        if seat_id > highest_seat_id
        {
            highest_seat_id = seat_id;
        }

        // part 2
        seats_occupied[seat_id as usize] = true;
    }

    println!("Part 1 - highest seat id: {}", highest_seat_id);

    // part 2
    for seat_id in 8..1016 {
        if !seats_occupied[seat_id] && seats_occupied[seat_id-1] && seats_occupied[seat_id+1]
        {
            println!("Part 2 - found seat: {}", seat_id);
        }
    }

    Ok(())
}

#[test]
fn test_part1_examples() {
    let input_strs = ["BFFFBBFRRR", "FFFBBBFRRR", "BBFFBBFRLL"];
    let row_results = [70, 14, 102];
    let column_results = [7, 7, 4];
    let seat_ids = [567, 119, 820];
    for (input_str, row_result, column_result, seat_id) in
        izip!(&input_strs, &row_results, &column_results, &seat_ids)
    {
        println!("Checking input {}", input_str);
        let (row_str, column_str) = split_str(input_str);
        let row = parse_binary_str(&rows_to_binary_str(row_str));
        assert_eq!(row, *row_result);
        let column = parse_binary_str(&columns_to_binary_str(column_str));
        assert_eq!(column, *column_result);
        assert_eq!(row * 8 + column, *seat_id);
    }
}

#[test]
fn test_parse_binary_str() {
    let binary_str = "1000110";
    assert_eq!(parse_binary_str(binary_str), 70);
}

#[test]
fn test_columns_to_binary_str() {
    let columns_str = "RLL";
    assert_eq!(columns_to_binary_str(columns_str), "100");
}

#[test]
fn test_rows_to_binary_str() {
    let rows_str = "BFFFBBF";
    assert_eq!(rows_to_binary_str(rows_str), "1000110");
}

#[test]
fn test_split_str() {
    let input_str = "FBFBBFFRLR";
    assert_eq!(split_str(input_str), ("FBFBBFF", "RLR"));
}
