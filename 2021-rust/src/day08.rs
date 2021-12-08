use std::fs::read_to_string;

struct Entry {
    patterns: Vec<String>,
    output: Vec<String>,
}

fn main() {
    let entries = parse_input();
    println!(
        "First answer is {}, second answer is {}",
        part_one(&entries),
        part_two(entries)
    );
}

fn parse_input() -> Vec<Entry> {
    let input: Vec<String> = read_to_string("input/08")
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect();
    let mut entries: Vec<Entry> = vec![];

    for line in input {
        let split_line: Vec<String> = line.split(" | ").map(|l| l.to_string()).collect();
        let entry = Entry {
            patterns: split_line[0].split(' ').map(|l| l.to_string()).collect(),
            output: split_line[1].split(' ').map(|l| l.to_string()).collect(),
        };
        entries.push(entry);
    }

    entries
}

fn part_one(entries: &[Entry]) -> usize {
    let mut easy_digits: usize = 0;

    for entry in entries {
        for digit in &entry.output {
            if digit.len() != 5 && digit.len() != 6 {
                easy_digits += 1;
            }
        }
    }

    easy_digits
}

fn part_two(entries: Vec<Entry>) -> usize {
    let mut sum: usize = 0;

    for entry in entries {
        sum += decode(entry);
    }

    sum
}

fn decode(entry: Entry) -> usize {
    let mut numbers: Vec<String> = vec![String::new(); 10];

    for digit in &entry.patterns {
        match digit.len() {
            2 => numbers[1] = digit.clone(),
            3 => numbers[7] = digit.clone(),
            4 => numbers[4] = digit.clone(),
            7 => numbers[8] = digit.clone(),
            _ => (),
        }
    }

    for digit in &entry.patterns {
        if digit.len() == 6 {
            if contains_all(digit, &numbers[4]) {
                numbers[9] = digit.clone();
            } else if contains_all(digit, &numbers[1]) {
                numbers[0] = digit.clone();
            } else {
                numbers[6] = digit.clone();
            }
        }
    }

    for digit in &entry.patterns {
        if digit.len() == 5 {
            if contains_all(digit, &numbers[1]) {
                numbers[3] = digit.clone();
            } else if contains_all(&numbers[6], digit) {
                numbers[5] = digit.clone();
            } else {
                numbers[2] = digit.clone();
            }
        }
    }

    let mut sum: String = String::new();

    for number in entry.output {
        for digit in &entry.patterns {
            if is_same(&number, digit) {
                let index = numbers.iter().position(|r| r == digit).unwrap();
                sum = format!("{}{}", sum, index.to_string());
            }
        }
    }

    sum.parse::<usize>().unwrap()
}

fn contains_all(first: &str, second: &str) -> bool {
    for char in second.chars() {
        if !first.contains(char) {
            return false;
        }
    }
    true
}

fn is_same(first: &str, second: &str) -> bool {
    if first.len() != second.len() {
        return false;
    }

    for char in second.chars() {
        if !first.contains(char) {
            return false;
        }
    }
    true
}
