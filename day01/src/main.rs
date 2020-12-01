use std::fs::read_to_string;

fn main() {
    let input_string = read_to_string("input.txt").unwrap();

    let mut expense_report = Vec::new();
    for entry in input_string.split('\n') {
        match entry.parse::<i32>() {
            Ok(entry_value) => expense_report.push(entry_value),
            Err(e) => println!("Ignoring entry, because of conversion error: {}", e),
        };
    }

    println!(
        "Got {} entries. All entries:\n{:?}",
        expense_report.len(),
        expense_report
    );

    for (entry_outer_id, entry_outer) in expense_report.iter().enumerate() {
        for (entry_middle_id, entry_middle) in expense_report[entry_outer_id..].iter().enumerate() {
            for entry_inner in &expense_report[entry_middle_id..] {
                if entry_inner + entry_middle + entry_outer == 2020 {
                    println!(
                        "Found valid entry triplet 2020 = {} + {} + {}",
                        entry_inner, entry_middle, entry_outer
                    );
                    println!(
                        "{} * {} * {} = {}",
                        entry_inner,
                        entry_middle,
                        entry_outer,
                        entry_outer * entry_middle * entry_inner
                    );
                }
            }
        }
    }
}
