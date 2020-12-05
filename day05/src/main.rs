#[cfg(test)]
use itertools::izip;
use std::fs::File;
use std::io;
use std::io::prelude::*;

pub fn parse_boarding_pass_str(boarding_pass_str: &str) -> usize {
    usize::from_str_radix(
        &boarding_pass_str
            .replace("F", "0")
            .replace("B", "1")
            .replace("L", "0")
            .replace("R", "1"),
        2,
    )
    .unwrap()
}

fn main() -> io::Result<()> {
    let mut file = File::open("input").unwrap();
    let mut input_string = String::new();
    file.read_to_string(&mut input_string)?;

    // for part 1
    let mut highest_seat_id = 0;
    // for part 2
    let mut seats_occupied = [false; 1024];

    for line in input_string.lines() {
        let seat_id = parse_boarding_pass_str(line);

        // part 1
        if seat_id > highest_seat_id {
            highest_seat_id = seat_id;
        }
        // part 2
        seats_occupied[seat_id as usize] = true;
    }

    // part 1
    println!("Part 1 - highest seat id: {}", highest_seat_id);

    // part 2
    for seat_id in 8..1016 {
        if !seats_occupied[seat_id] && seats_occupied[seat_id - 1] && seats_occupied[seat_id + 1] {
            println!("Part 2 - found seat: {}", seat_id);
        }
    }

    Ok(())
}

#[test]
fn test_part1_examples() {
    let input_strs = ["BFFFBBFRRR", "FFFBBBFRRR", "BBFFBBFRLL"];
    let seat_ids = [567, 119, 820];
    for (input_str, seat_id) in izip!(&input_strs, &seat_ids) {
        assert_eq!(parse_boarding_pass_str(input_str), *seat_id);
    }
}
