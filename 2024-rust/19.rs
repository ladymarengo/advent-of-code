use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("input/19.txt").unwrap();
    let (patterns_input, designs) = input.split_once("\n\n").unwrap();
    let patterns: Vec<String> = patterns_input.split(", ").map(|p| p.to_string()).collect();

    let result_part_one = designs
        .lines()
        .filter(|design| is_design_possible(&patterns, design))
        .count();
    println!("First answer is {result_part_one}");

    let mut result_part_two = 0;
    for design in designs.lines() {
        let mut result = 1;
        let mut ways: HashMap<String, usize> = HashMap::new();
        result *= all_possible_ways(&patterns, design, &mut ways);
        result_part_two += result;
    }
    println!("Second answer is {result_part_two}");
}

fn is_design_possible(patterns: &Vec<String>, design: &str) -> bool {
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

fn all_possible_ways(
    patterns: &Vec<String>,
    design: &str,
    ways: &mut HashMap<String, usize>,
) -> usize {
    if design.is_empty() {
        return 1;
    }
    let mut result = 0;
    for pattern in patterns.iter() {
        if design.starts_with(pattern) {
            if let Some(amount) = ways.get(&design[pattern.len()..]) {
                result += amount;
            } else {
                result += all_possible_ways(patterns, &design[pattern.len()..], ways);
            }
        }
    }
    if !ways.contains_key(design) {
        ways.insert(design.to_string(), result);
    }
    result
}
