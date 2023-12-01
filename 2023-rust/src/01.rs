use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/01.txt").unwrap();

	let first_values = input.lines().map(|line| {
		let mut value = String::new();
		value.push(line.chars().find(|c| c.is_ascii_digit()).unwrap());
		value.push(line.chars().rev().find(|c| c.is_ascii_digit()).unwrap());
		value.parse::<i32>().unwrap()
	});

	println!("First answer is {}", first_values.sum::<i32>());

	let second_values = input.lines().map(|line| {
		let mut value = String::new();
		for (i, c) in line.chars().enumerate() {
			if c.is_ascii_digit() {
				value.push(c);
				break;
			} else if let Some(digit) = letters_to_digit(&line[i..]) {
				value.push((digit + 1).to_string().chars().next().unwrap());
				break;
			}
		}
		for i in (0..line.len()).rev() {
			let c = line.chars().collect::<Vec<char>>()[i];
			if c.is_ascii_digit() {
				value.push(c);
				break;
			} else if let Some(digit) = letters_to_digit(&line[i..]) {
				value.push((digit + 1).to_string().chars().next().unwrap());
				break;
			}
		}
		value.parse::<usize>().unwrap()
	});

	println!("Second answer is {}", second_values.sum::<usize>());
}

fn letters_to_digit(str: &str) -> Option<usize> {
	let digits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
	if let Some(value) = digits.iter().find(|digit| str.starts_with(*digit)) {
		return digits.iter().position(|&r| r == *value)
	}
	None
}