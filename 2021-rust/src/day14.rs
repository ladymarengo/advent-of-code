use std::fs::read_to_string;
use std::collections::HashMap;
use std::str;

fn main() {
    let (template, pairs) = parse_input();
    println!("First answer is {}", solve(template.clone(), &pairs, 10));
    println!("Second answer is {}", solve(template, &pairs, 40));
}

fn parse_input() -> (String, HashMap<String, char>) {
    let input: String = read_to_string("input/14").unwrap();
    let tuple: (&str, &str) = input.split_once("\n\n").unwrap();
    
    let template: String = tuple.0.to_string();
    
    let pairs: HashMap<String, char> = tuple.1.to_string().lines().map(|l| {
        let mut spl = l.split(" -> ");
            (
                spl.next().unwrap().to_string(),
                spl.next().unwrap().to_string().chars().next().unwrap(),
            )
        }).collect();
    
    (template, pairs)
}

fn solve(template: String, pairs: &HashMap<String, char>, steps: usize) -> usize {
    let first: char = template.chars().collect::<Vec<char>>()[0];
    let last: char = template.chars().collect::<Vec<char>>()[template.len()-1];
    let mut elem_pairs: HashMap<String, usize> = HashMap::new();
    let slice = template.as_bytes();
    let iter = slice.windows(2);
    for a in iter {
        let quantity = elem_pairs
            .entry(str::from_utf8(&a.to_vec()).unwrap().to_string())
            .or_insert(0);
        *quantity += 1;
    }
    for _i in 0..steps {
        elem_pairs = step(elem_pairs, pairs);
    }
    count_quantity_two(elem_pairs, first, last)
}

fn step(elem_pairs: HashMap<String, usize>, pairs: &HashMap<String, char>) -> HashMap<String, usize> {
    
    let mut new_pairs: HashMap<String, usize> = HashMap::new();
    for elem in elem_pairs {
        let new_elem = pairs[&elem.0];
        let quantity = new_pairs
            .entry(str::from_utf8(&[elem.0.chars().collect::<Vec<char>>()[0] as u8, new_elem as u8]).unwrap().to_string())
            .or_insert(0);
        *quantity += elem.1;
        let quantity = new_pairs
            .entry(str::from_utf8(&[new_elem as u8, elem.0.chars().collect::<Vec<char>>()[1] as u8]).unwrap().to_string())
            .or_insert(0);
        *quantity += elem.1;
    }
    new_pairs
}

fn count_quantity_two(elem_pairs: HashMap<String, usize>, first: char, last: char) -> usize {
    let mut chars: HashMap<char, usize> = HashMap::new();
    for pair in elem_pairs {
        let quantity = chars
            .entry(pair.0.chars().collect::<Vec<char>>()[0])
            .or_insert(0);
        *quantity += pair.1;
        let quantity = chars
            .entry(pair.0.chars().collect::<Vec<char>>()[1])
            .or_insert(0);
        *quantity += pair.1;
    }
    let chars_vec: Vec<char> = chars.keys().copied().collect();
    for char in chars_vec {
        let quantity = chars
            .entry(char)
            .or_insert(0);
        *quantity /= 2;
    }
    let quantity = chars
        .entry(first)
        .or_insert(0);
    *quantity += 1;
    let quantity = chars
        .entry(last)
        .or_insert(0);
    *quantity += 1;
    chars.values().max().unwrap() - chars.values().min().unwrap()
}