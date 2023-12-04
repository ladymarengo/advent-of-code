use std::fs::read_to_string;

#[derive(Debug)]
struct Card {
    copies: usize,
    matching: usize,
}

fn main() {
    let input = read_to_string("input/04.txt").unwrap();

    let mut cards: Vec<Card> = input
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

            let matching_numbers = numbers[1]
                .iter()
                .filter(|number| numbers[0].contains(*number))
                .count();

            Card {
                copies: 1,
                matching: matching_numbers,
            }
        })
        .collect();

    let points: u32 = cards
        .iter()
        .map(|card| {
            if card.matching == 0 {
                return 0;
            }
            u32::pow(2, card.matching as u32 - 1)
        })
        .sum();

    println!("First answer is {points}");

    (0..cards.len()).for_each(|i| {
        (1..=cards[i].matching).for_each(|copy| {
            if i + copy < cards.len() {
                cards[i + copy].copies += cards[i].copies
            }
        })
    });
    let all_cards: usize = cards.iter().map(|card| card.copies).sum();

    println!("Second answer is {all_cards}");
}
