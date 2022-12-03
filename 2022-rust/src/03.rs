use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/03.txt").unwrap();
    let common_items = input
        .lines()
        .map(|l| check_rucksack(l.chars().collect()))
        .sum::<u32>();
    let badges = input
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<String>>()
        .chunks(3)
        .map(check_badge)
        .sum::<u32>();
    println!("{common_items} {badges}");
}

fn check_rucksack(rucksack: Vec<char>) -> u32 {
    let first = &rucksack[0..rucksack.len() / 2];
    let second = &rucksack[rucksack.len() / 2..rucksack.len()];
    item_type(first.iter().find(|i| second.contains(*i)).unwrap())
}

fn item_type(item: &char) -> u32 {
    if *item as u32 >= 'a' as u32 {
        *item as u32 - 'a' as u32 + 1
    } else {
        *item as u32 - 'A' as u32 + 27
    }
}

fn check_badge(elves: &[String]) -> u32 {
    item_type(
        &elves[0]
            .chars()
            .find(|i| elves[1].contains(*i) && elves[2].contains(*i))
            .unwrap(),
    )
}
