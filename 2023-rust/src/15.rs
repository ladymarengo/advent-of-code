use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/15.txt").unwrap();

    let results: u32 = input
        .split(',')
        .map(|string| {
            let mut value = 0;
            string.chars().for_each(|c| {
                value = (value + c as u32) * 17 % 256;
            });
            value
        })
        .sum();

    println!("First answer is {results}");
}
