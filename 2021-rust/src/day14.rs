use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let (template, pairs) = parse_input();
    println!("First answer is {}", solve(template.clone(), &pairs, 10));
    println!("Second answer is {}", solve(template, &pairs, 40));
}

fn parse_input() -> (String, HashMap<(char, char), char>) {
    let input: String = read_to_string("input/14").unwrap();
    let tuple: (&str, &str) = input.split_once("\r\n\r\n").unwrap();

    let template: String = tuple.0.to_string();

    let pairs: HashMap<(char, char), char> = tuple
        .1
        .to_string()
        .lines()
        .map(|l| {
            let (pair, insert) = l.split_once(" -> ").unwrap();
            let pair: Vec<char> = pair.chars().collect();
            ((pair[0], pair[1]), insert.chars().next().unwrap())
        })
        .collect();

    (template, pairs)
}

fn solve(template: String, pairs: &HashMap<(char, char), char>, steps: usize) -> usize {
    let first: char = template.chars().next().unwrap();
    let last: char = template.chars().last().unwrap();

    let mut elem_pairs: HashMap<(char, char), usize> = HashMap::new();
    let slice: Vec<char> = template.chars().collect();

    for a in slice.windows(2) {
        *elem_pairs.entry((a[0], a[1])).or_insert(0) += 1;
    }
    for _i in 0..steps {
        elem_pairs = step(elem_pairs, pairs);
    }
    count_quantity_two(elem_pairs, first, last)
}

fn step(
    elem_pairs: HashMap<(char, char), usize>,
    pairs: &HashMap<(char, char), char>,
) -> HashMap<(char, char), usize> {
    let mut new_pairs: HashMap<(char, char), usize> = HashMap::new();

    for elem in elem_pairs {
        let new_elem = pairs[&elem.0];
        *new_pairs.entry((elem.0 .0, new_elem)).or_insert(0) += elem.1;
        *new_pairs.entry((new_elem, elem.0 .1)).or_insert(0) += elem.1;
    }
    new_pairs
}

fn count_quantity_two(elem_pairs: HashMap<(char, char), usize>, first: char, last: char) -> usize {
    let mut chars: HashMap<char, usize> = HashMap::new();

    for pair in elem_pairs {
        *chars.entry(pair.0 .0).or_insert(0) += pair.1;
        *chars.entry(pair.0 .1).or_insert(0) += pair.1;
    }

    for value in chars.values_mut() {
        *value /= 2;
    }

    *chars.entry(first).or_insert(0) += 1;
    *chars.entry(last).or_insert(0) += 1;

    chars.values().max().unwrap() - chars.values().min().unwrap()
}
