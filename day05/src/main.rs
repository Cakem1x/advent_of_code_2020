pub fn split_str(input_str: &str) -> (&str, &str) {
    let rows_str = &input_str[..7];
    let columns_str = &input_str[7..];
    return (rows_str, columns_str);
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_split_str() {
    let input_str = "FBFBBFFRLR";
    assert_eq!(split_str(input_str), ("FBFBBFF", "RLR"));
}
