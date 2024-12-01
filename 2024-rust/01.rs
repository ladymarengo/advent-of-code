use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/01.txt").unwrap();

    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();
    input.lines().for_each(|line| {
        let (left_value, right_value) = line.split_once("   ").unwrap();
        left_list.push(left_value.parse().unwrap());
        right_list.push(right_value.parse().unwrap());
    });
    left_list.sort();
    right_list.sort();

    let total_distance: i32 = (0..left_list.len())
        .map(|i| (left_list[i] - right_list[i]).abs())
        .sum();
    println!("First answer is {total_distance}");

    let similarity_score: i32 = left_list
        .iter()
        .map(|left_value| {
            left_value
                * right_list
                    .iter()
                    .filter(|right_value| *right_value == left_value)
                    .count() as i32
        })
        .sum();
    println!("Second answer is {similarity_score}");
}
