use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/19.txt").unwrap();
    let (patterns_input, designs) = input.split_once("\n\n").unwrap();
    let patterns: Vec<&str> = patterns_input.split(", ").collect();
    let result = designs
        .lines()
        .filter(|design| is_design_possible(&patterns, design))
        .count();
    println!("First answer is {result}");
}

fn is_design_possible(patterns: &Vec<&str>, design: &str) -> bool {
    if design.is_empty() {
        return true;
    }
    for pattern in patterns.iter() {
        if design.starts_with(pattern) && is_design_possible(patterns, &design[pattern.len()..]) {
            return true;
        }
    }
    false
}
