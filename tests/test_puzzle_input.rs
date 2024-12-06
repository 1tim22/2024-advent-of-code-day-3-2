use std::fs;
use day_3::*;

#[test]
fn test_parse_and_multiplication() {
    let table: Vec<(String, i32, i32)> = parse_operation(
        fs::read_to_string("input.txt").expect("Failed to open file."),
        "mul"
    );

    let total = process(table);

    assert_eq!(total, 175015740);
}
