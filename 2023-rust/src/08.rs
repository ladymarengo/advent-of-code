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
    println!("Second answer is {}", part_two(&network, &instructions));
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

fn part_two(network: &HashMap<String, (String, String)>, instructions: &[char]) -> usize {
    let mut current_nodes = Vec::new();
    network.keys().for_each(|node| {
        if node.chars().collect::<Vec<char>>()[2] == 'A' {
            current_nodes.push(node)
        }
    });

    let mut instr_index;
    let mut steps: Vec<usize> = Vec::new();

    for node in &mut current_nodes {
        instr_index = 0;
        let mut step = 0;
        while node.chars().collect::<Vec<char>>()[2] != 'Z' {
            match instructions[instr_index] {
                'L' => *node = &network.get(*node).unwrap().0,
                'R' => *node = &network.get(*node).unwrap().1,
                _ => (),
            }
            instr_index += 1;
            if instr_index == instructions.len() {
                instr_index = 0;
            }
            step += 1;
        }

        steps.push(step);
    }

    let mut answer = 1;
    steps.iter().for_each(|step| answer *= *step);

    let min = *steps.iter().min().unwrap();
    for i in (min..=answer).step_by(min) {
        if steps.iter().all(|number| i % number == 0) {
            return i;
        }
    }
    0
}
