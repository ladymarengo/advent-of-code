use std::fs::read_to_string;

#[derive(Debug, Copy, Clone)]
enum Symbol {
    Open,
    Close,
    Value(i32),
}

fn main() {
    let input = parse_input();
    println!("First answer is {}", part_one(input.clone()));
    println!("Second answer is {}", part_two(input));
}

fn parse_input() -> Vec<Vec<Symbol>> {
    let input: Vec<String> = read_to_string("input/18")
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect();
    let mut numbers: Vec<Vec<Symbol>> = vec![];
    for line in input {
        numbers.push(parse_number(line));
    }
    numbers
}

fn parse_number(line: String) -> Vec<Symbol> {
    let mut number: Vec<Symbol> = vec![];
    let mut value: String = String::new();
    for char in line.chars().collect::<Vec<char>>() {
        match char {
            '[' => {
                if !value.is_empty() {
                    number.push(Symbol::Value(value.parse::<i32>().unwrap()));
                    value.clear();
                }
                number.push(Symbol::Open)
            }
            ']' => {
                if !value.is_empty() {
                    number.push(Symbol::Value(value.parse::<i32>().unwrap()));
                    value.clear();
                }
                number.push(Symbol::Close)
            }
            ',' => {
                if !value.is_empty() {
                    number.push(Symbol::Value(value.parse::<i32>().unwrap()));
                    value.clear();
                }
            }
            _ => value.push(char),
        }
    }
    number
}

fn part_one(numbers: Vec<Vec<Symbol>>) -> i32 {
    let mut result: Vec<Symbol> = vec![];
    for number in &numbers {
        if result.is_empty() {
            result = number.clone();
        } else {
            result = add(&result, number);
        }
    }
    count_magnitude(&result)
}

fn part_two(numbers: Vec<Vec<Symbol>>) -> i32 {
    let mut max_magnitude: i32 = 0;
    for number1 in &numbers {
        for number2 in &numbers {
            let result = add(number1, number2);
            let magnitude = count_magnitude(&result);
            if magnitude > max_magnitude {
                max_magnitude = magnitude;
            }
        }
    }
    max_magnitude
}

fn add(first: &[Symbol], second: &[Symbol]) -> Vec<Symbol> {
    let mut new: Vec<Symbol> = vec![Symbol::Open];
    new.extend_from_slice(first);
    new.extend_from_slice(second);
    new.push(Symbol::Close);
    let mut reduced = true;
    while reduced {
        reduced = action(&mut new);
    }
    new
}

fn action(number: &mut Vec<Symbol>) -> bool {
    if explode(number) || split(number) {
        return true;
    }
    false
}

fn explode(number: &mut Vec<Symbol>) -> bool {
    let mut opened: usize = 0;
    let mut index: usize = 0;
    for (i, symbol) in number.iter().enumerate() {
        match symbol {
            Symbol::Open => opened += 1,
            Symbol::Close => opened -= 1,
            _ => (),
        }
        if opened > 4 {
            index = i;
            break;
        }
    }
    if index == 0 {
        false
    } else {
        let mut left: i32 = 0;
        if let Symbol::Value(a) = number[index + 1] {
            left = a
        }
        for i in (0..index).rev() {
            if let Symbol::Value(n) = &mut number[i] {
                *n += left;
                break;
            }
        }
        let mut right: i32 = 0;
        if let Symbol::Value(a) = number[index + 2] {
            right = a
        }
        for symbol in &mut number[index + 4..] {
            if let Symbol::Value(n) = symbol {
                *n += right;
                break;
            }
        }
        for _i in 0..4 {
            number.remove(index);
        }
        number.insert(index, Symbol::Value(0));
        true
    }
}

fn split(number: &mut Vec<Symbol>) -> bool {
    let mut index: usize = 0;
    let mut value: i32 = 0;
    for (i, symbol) in number.iter().enumerate() {
        match symbol {
            Symbol::Value(a) if *a > 9 => {
                index = i;
                value = *a;
                break;
            }
            _ => (),
        }
    }
    if index == 0 {
        false
    } else {
        number.remove(index);
        number.insert(index, Symbol::Close);
        number.insert(index, Symbol::Value(value / 2 + value % 2));
        number.insert(index, Symbol::Value(value / 2));
        number.insert(index, Symbol::Open);
        true
    }
}

fn count_magnitude(number: &[Symbol]) -> i32 {
    let mut opened: usize = 1;
    let left: i32;
    let right: i32;
    let mut index: usize = 2;
    if let Symbol::Value(n) = number[1] {
        left = n;
    } else {
        while index < number.len() {
            match number[index] {
                _ if opened == 0 => break,
                Symbol::Open => opened += 1,
                Symbol::Close => opened -= 1,
                _ => (),
            }
            index += 1;
        }
        left = count_magnitude(&number[1..index].to_vec());
    }
    if let Symbol::Value(n) = number[index] {
        right = n;
    } else {
        right = count_magnitude(&number[index..number.len() - 1].to_vec());
    }
    3 * left + 2 * right
}
