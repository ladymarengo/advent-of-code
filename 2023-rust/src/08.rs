use regex::Regex;
use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("input/08.txt").unwrap();

    let (instructions_input, network_input) = input.split_once("\n\n").unwrap();
    let instructions = instructions_input.chars().collect::<Vec<char>>();
    let mut network: HashMap<String, (String, String)> = HashMap::new();

    network_input.lines().for_each(|line| {
        let re = Regex::new(r"(\w{3}) = \((\w{3}), (\w{3})\)").unwrap();
        let caps = re.captures(line).unwrap();
        network.insert(
            caps[1].to_string(),
            (caps[2].to_string(), caps[3].to_string()),
        );
    });

    println!("First answer is {}", part_one(&network, &instructions));
}

fn part_one(network: &HashMap<String, (String, String)>, instructions: &[char]) -> usize {
    let mut current_node = "AAA";
    let mut instr_index = 0;
    let mut steps: usize = 0;

    while current_node != "ZZZ" {
        match instructions[instr_index] {
            'L' => current_node = &network.get(current_node).unwrap().0,
            'R' => current_node = &network.get(current_node).unwrap().1,
            _ => (),
        }
        instr_index += 1;
        if instr_index == instructions.len() {
            instr_index = 0;
        }
        steps += 1;
    }
    steps
}
