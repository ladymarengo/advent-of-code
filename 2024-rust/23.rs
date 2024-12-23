use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    let input = read_to_string("input/23.txt").unwrap();
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    input.lines().for_each(|line| {
        let (first, second) = line.split_once("-").unwrap();
        let entry = graph.entry(first.to_string()).or_default();
        entry.push(second.to_string());
        let entry: &mut Vec<String> = graph.entry(second.to_string()).or_default();
        entry.push(first.to_string());
    });

    let mut comp_sets: HashSet<Vec<&String>> = HashSet::new();
    for first in graph.iter() {
        for second in first.1.iter() {
            if let Some(second_vec) = graph.get(second) {
                second_vec.iter().for_each(|c| {
                    if first.1.contains(c)
                        && (first.0.starts_with("t")
                            || second.starts_with("t")
                            || c.starts_with("t"))
                    {
                        let mut new = vec![first.0, second, c];
                        new.sort();
                        comp_sets.insert(new);
                    }
                });
            }
        }
    }
    println!("First answer is {}", comp_sets.len());

    let mut biggest_party: Vec<String> = Vec::new();
    for node in graph.iter() {
        check_party(node.0, &node.1, &graph, &mut biggest_party);
    }
    biggest_party.sort();
    println!("Second answer is '{}'", biggest_party.join(","));
}

fn check_party(
    current: &String,
    computers: &Vec<String>,
    graph: &HashMap<String, Vec<String>>,
    biggest_party: &mut Vec<String>,
) {
    let mut party_options: Vec<Vec<&String>> = Vec::new();
    party_options.push(Vec::from([current]));
    for comp in computers {
        for i in 0..party_options.len() {
            let mut connected: Vec<&String> = party_options[i]
                .iter()
                .filter(|&&c| graph.get(c).unwrap().contains(comp))
                .map(|c| *c)
                .collect();
            if connected.len() == party_options[i].len() {
                party_options[i].push(comp);
            } else {
                connected.push(comp);
                party_options.push(connected);
            }
        }
    }
    party_options.sort_by(|a, b| b.len().cmp(&a.len()));
    if party_options[0].len() > biggest_party.len() {
        biggest_party.clear();
        for comp in party_options[0].iter() {
            biggest_party.push(comp.to_string());
        }
    }
}
