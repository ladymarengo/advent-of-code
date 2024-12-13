use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/13.txt").unwrap();

    println!("First answer is {}", count_tokens(&input, 0));
    println!("Second answer is {}", count_tokens(&input, 10000000000000));
}

#[derive(Debug)]
struct Coords {
    x: i64,
    y: i64,
}

fn count_tokens(input: &String, addition: i64) -> i64 {
    let re = Regex::new(r"X[+|=](?<x>\d+), Y[+|=](?<y>\d+)").unwrap();

    input
        .split("\n\n")
        .map(|lines| {
            let machine: Vec<String> = lines.lines().map(|line| line.to_string()).collect();
            let input_button_a = re.captures(&machine[0]).unwrap();
            let input_button_b = re.captures(&machine[1]).unwrap();
            let input_prize = re.captures(&machine[2]).unwrap();
            let button_a = Coords {
                x: input_button_a["x"].parse::<i64>().unwrap(),
                y: input_button_a["y"].parse::<i64>().unwrap(),
            };
            let button_b = Coords {
                x: input_button_b["x"].parse::<i64>().unwrap(),
                y: input_button_b["y"].parse::<i64>().unwrap(),
            };
            let prize = Coords {
                x: input_prize["x"].parse::<i64>().unwrap() + addition,
                y: input_prize["y"].parse::<i64>().unwrap() + addition,
            };

            let press_b = (button_a.x * prize.y - button_a.y * prize.x)
                / (button_a.x * button_b.y - button_a.y * button_b.x);
            let press_a = (prize.x - button_b.x * press_b) / button_a.x;
            
            if press_b * button_b.x + press_a * button_a.x == prize.x
                && press_b * button_b.y + press_a * button_a.y == prize.y
            {
                return press_b + press_a * 3;
            }
            0
        })
        .sum()
}
