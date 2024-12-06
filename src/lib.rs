use regex::Regex;

pub fn parse_operation(contents: String, pattern: &'static str) -> Vec<(String, i32, i32)> {
    // Remove arbitrary whitespace
    let mut enabled_content = Regex::new(r"[\n\r\s]").expect("Invalid RegEx")
        .replace_all(contents.as_str(), "")
        .to_string();

    // Add structure to split
    enabled_content = Regex::new(r"do\(\)").expect("Invalid RegEx")
        .replace_all(&enabled_content, "$0\n")
        .to_string();

    // Split each line
    enabled_content = Regex::new(r"\n").expect("Invalid RegEx")
        .split(&enabled_content)
        .collect::<Vec<&str>>()
        .iter()
        .fold(String::from(""), |acc, value|
            acc + &Regex::new(r"don't\(\).*?do\(\)").expect("Invalid RegEx")
            .replace_all(&value, "")
        );

    // Remove disabled instructions
    enabled_content = Regex::new(r"don't\(\).*?do\(\)").expect("Invalid RegEx")
        .replace_all(&enabled_content, "")
        .to_string();

    // Parse a valid instruction
    Regex::new(format!(r"({})\((\d\d*\d*),(\d\d*\d*)\)", pattern).as_str()).expect("Invalid RegEx")
        // Parse capture groups as well as match
        .captures_iter(&enabled_content)
            .map(|operation| {
                let (_, [op, a, b]) = operation.extract();
                (op.to_string(), a.parse().expect("Could not parse operand"), b.parse().expect("Could not parse operand"))
            }).collect::<Vec<(String, i32, i32)>>()
}

pub fn process(list: Vec<(String, i32, i32)>) -> i32 {
    list.iter()
        .map(|(op, a, b)| calculate(op, a, b))
        .sum()
}

fn calculate(op: &String, a: &i32, b: &i32) -> i32 {
    match op.as_str() {
        "mul" => a * b,
            _ => 0,
    }
}
