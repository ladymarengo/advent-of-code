use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let instructions: Vec<String> = read_to_string("input/02").
        unwrap().lines().map(|l| l.parse().unwrap()).collect();
    let mut depth1: i32 = 0;
    let mut depth2: i32 = 0;
    let mut aim: i32 = 0;
    let mut position: i32 = 0;
    let re = Regex::new(r"(\w+) (\d+)").unwrap();
    
    for line in instructions {
        let instruction = re.captures(&line).unwrap();
        let value = instruction.get(2).unwrap().as_str().parse::<i32>().unwrap();
        match instruction.get(1).unwrap().as_str() {
            "forward" => {
                position = position + value;
                depth2 = depth2 + (aim * value);
            },
            "up" => {
                depth1 = depth1 - value;
                aim = aim - value;
            },
            "down" => {
                depth1 = depth1 + value;
                aim = aim + value;
            },
            _ => println!("error"),
        }
    }
    
    println!("First answer is {}\nSecond answer is {}", depth1 * position, depth2 * position);
}
