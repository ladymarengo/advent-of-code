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

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(do\(\))|(don't\(\))").unwrap();
    let mut enabled = true;
    let mut result: i32 = 0;
    for cap in re.captures_iter(&input) {
        if cap.get(3).is_some() {
            enabled = true;
        } else if cap.get(4).is_some() {
            enabled = false;
        } else if enabled {
            result += cap.get(1).unwrap().as_str().parse::<i32>().unwrap()
                * cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
        }
    }
    println!("Second answer is {result}");
}
