use regex::Regex;

pub fn parse_operation(contents: String, pattern: &'static str) -> Vec<(String, i32, i32)> {
    // Parse a valid instruction
    Regex::new(format!(r"({})\((\d\d*\d*),(\d\d*\d*)\)", pattern).as_str()).expect("Invalid RegEx")
        // Parse capture groups as well as match
        .captures_iter(contents.as_str())
            .map(|operation| {
                let (_, [op, a, b]) = operation.extract();
                (op.to_string(), a.parse().expect("Could not parse operand"), b.parse().expect("Could not parse operand"))
            }).collect::<Vec<(String, i32, i32)>>()
}

pub fn process(list: Vec<(String, i32, i32)>) -> i32 {
    list.iter()
        .map(|(op, a, b)| calculate(op, a, b))

        // Or fold if the next part requires something other than sum
        .sum()
}

fn calculate(op: &String, a: &i32, b: &i32) -> i32 {
    match op.as_str() {
        "add" => a + b,
        "sub" => a - b,
        "mul" => a * b,
        "div" => a / b,
            _ => 0,
    }
}
