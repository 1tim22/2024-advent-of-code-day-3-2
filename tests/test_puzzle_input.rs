use std::fs;
use day_3::*;

#[test]
fn test_parse_and_multiplication() {
    let table: Vec<(String, i32, i32)> = parse_operation(
        fs::read_to_string("input.txt").expect("Failed to open file."),
        "mul"
    );

    let total = process(table);

    // That's not the right answer; your answer is too low.
    assert_ne!(total, 56372201);

    // That's not the right answer; your answer is too high.
    assert_ne!(total, 175015740);

    // That's the right answer! You are one gold star closer to finding the Chief Historian.
    assert_eq!(total, 112272912);
}
