use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("input/22.txt").unwrap();
    let mut first_result = 0;
    let mut full_seq_map: HashMap<Vec<i32>, i32> = HashMap::new();
    for (_, line) in input.lines().enumerate() {
        let mut number = line.parse::<usize>().unwrap();
        let mut previous = number as i32 % 10;
        let mut sequences: Vec<i32> = Vec::new();
        let mut seq_map: HashMap<Vec<i32>, i32> = HashMap::new();
        for _ in 0..2000 {
            let new = next_secret_number(number);
            number = new;
            let price = number as i32 % 10;
            sequences.push(price - previous);
            if sequences.len() >= 4 {
                let slice = &sequences[sequences.len() - 4..];
                if !seq_map.contains_key(slice) {
                    seq_map.insert(slice.to_vec(), price);
                }
            }
            previous = price;
        }
        first_result += number;
        for (seq, price) in seq_map {
            let entry = full_seq_map.entry(seq).or_default();
            *entry += price;
        }
    }
    let second_result = full_seq_map.values().max().unwrap();
    println!("First answer is {first_result}");
    println!("Second answer is {second_result}");
}

fn next_secret_number(secret_number: usize) -> usize {
    let mut new_number = secret_number * 64;
    new_number = new_number ^ secret_number;
    new_number = new_number % 16777216;
    new_number = new_number ^ (new_number / 32);
    new_number = new_number % 16777216;
    new_number = new_number ^ (new_number * 2048);
    new_number = new_number % 16777216;
    new_number
}
