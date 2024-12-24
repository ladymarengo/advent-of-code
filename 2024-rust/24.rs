use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("input/24.txt").unwrap();
    let (start_values, gates) = input.split_once("\n\n").unwrap();

    let mut wires: HashMap<String, bool> = HashMap::new();
    start_values.lines().for_each(|line| {
        let (wire, value) = line.split_once(": ").unwrap();
        wires.insert(wire.to_string(), if value == "1" { true } else { false });
    });

    let mut gates: Vec<Gate> = gates
        .lines()
        .map(|line| {
            let values: Vec<String> = line.split(" ").map(|part| part.to_string()).collect();
            let operation = match &values[1][..] {
                "AND" => Operation::And,
                "OR" => Operation::Or,
                _ => Operation::Xor,
            };
            Gate {
                first: values[0].clone(),
                second: values[2].clone(),
                operation,
                result: values[4].clone(),
                value: None,
            }
        })
        .collect();

    let mut z_wires: Vec<String> = Vec::new();

    while gates.iter().any(|gate| gate.value.is_none()) {
        gates.iter_mut().for_each(|gate| {
            if gate.value.is_none()
                && wires.contains_key(&gate.first)
                && wires.contains_key(&gate.second)
            {
                let first = wires.get(&gate.first).unwrap();
                let second = wires.get(&gate.second).unwrap();
                let result_value = match gate.operation {
                    Operation::And => *first && *second,
                    Operation::Or => *first || *second,
                    Operation::Xor => *first ^ *second,
                };
                wires.insert(gate.result.clone(), result_value);
                gate.value = Some(result_value);
                if gate.result.starts_with("z") {
                    z_wires.push(gate.result.clone());
                }
            }
        });
    }
    z_wires.sort();
    z_wires.reverse();

    let mut result = String::new();
    for wire in z_wires {
        let value = wires.get(&wire).unwrap();
        result.push(if *value { '1' } else { '0' });
    }
    println!(
        "First answer is {}",
        isize::from_str_radix(&result, 2).unwrap()
    );
}

struct Gate {
    first: String,
    second: String,
    operation: Operation,
    result: String,
    value: Option<bool>,
}

enum Operation {
    And,
    Or,
    Xor,
}
