use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/03.txt").unwrap();

    let re = Regex::new(r"mul\((\d{1,3},\d{1,3})\)").unwrap();
    let result: i32 = re
        .captures_iter(&input)
        .map(|instr| {
            let (left, right) = instr.get(1).unwrap().as_str().split_once(",").unwrap();
            left.parse::<i32>().unwrap() * right.parse::<i32>().unwrap()
        })
        .sum();
    println!("First answer is {result}");

    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)").unwrap();
    let mut enabled = true;
    let mut result: i32 = 0;
    for cap in re.captures_iter(&input) {
        let instruction = cap.get(0).unwrap().as_str();
        match cap.get(0).unwrap().as_str() {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                if enabled {
                    let instr_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
                    let caps = instr_re.captures(&instruction).unwrap();
                    result += &caps[1].parse::<i32>().unwrap() * &caps[2].parse::<i32>().unwrap();
                }
            }
        }
    }
    println!("Second answer is {result}");
}
