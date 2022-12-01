use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/01.txt").unwrap();
    let mut elves: Vec<i32> = input
        .split("\n\n")
        .map(|e| e.lines().map(|l| l.parse::<i32>().unwrap()).sum::<i32>())
        .collect::<Vec<i32>>();
    elves.sort_by(|a, b| b.cmp(a));
    println!("Max is {}", elves[0]);
    println!("Top three is {}", &elves[0..3].iter().sum::<i32>())
}
