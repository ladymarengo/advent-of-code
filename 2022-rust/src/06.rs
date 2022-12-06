use std::fs::read_to_string;
use std::collections::HashSet;

fn main() {
    let input: Vec<char> = read_to_string("input/06.txt").unwrap().chars().collect();
	tests();
	println!("{}", find_first(input.clone(), 4));
	println!("{}", find_first(input, 14));
}

fn find_first(datastream: Vec<char>, amount: usize) -> usize {
	let first = &datastream[..].windows(amount).enumerate().find(|(_i, s)| {let set: HashSet<&char> = s.iter().collect(); set.len() == s.len()}).unwrap();
	first.0 + amount
}

fn tests() {
	assert_eq!(find_first("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string().chars().collect(), 4), 7);
	assert_eq!(find_first("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string().chars().collect(), 4), 5);
	assert_eq!(find_first("nppdvjthqldpwncqszvftbrmjlhg".to_string().chars().collect(), 4), 6);
	assert_eq!(find_first("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string().chars().collect(), 4), 10);
	assert_eq!(find_first("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string().chars().collect(), 4), 11);

	assert_eq!(find_first("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string().chars().collect(), 14), 19);
	assert_eq!(find_first("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string().chars().collect(), 14), 23);
	assert_eq!(find_first("nppdvjthqldpwncqszvftbrmjlhg".to_string().chars().collect(), 14), 23);
	assert_eq!(find_first("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string().chars().collect(), 14), 29);
	assert_eq!(find_first("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string().chars().collect(), 14), 26);
}