use std::fs::read_to_string;

struct Card {
    winning: Vec<u32>,
    have: Vec<u32>,
}

fn main() {
    let input = read_to_string("input/04.txt").unwrap();

    let cards: Vec<Card> = input
        .lines()
        .map(|line| {
            let numbers = line
                .split_once(": ")
                .unwrap()
                .1
                .split(" | ")
                .map(|numbers| {
                    numbers
                        .split_whitespace()
                        .map(|number| number.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>()
                })
                .collect::<Vec<Vec<u32>>>();
            Card {
                winning: numbers[0].clone(),
                have: numbers[1].clone(),
            }
        })
        .collect();
    let points: u32 = cards
        .iter()
        .map(|card| {
            let winning_numbers = card
                .have
                .iter()
                .filter(|number| card.winning.contains(*number))
                .count();
            if winning_numbers == 0 {
                return 0;
            }
            u32::pow(2, winning_numbers as u32 - 1)
        })
        .sum();
    println!("{points}");
}
