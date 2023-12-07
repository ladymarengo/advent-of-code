use std::{cmp::Ordering, collections::HashMap, fs::read_to_string};

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    bid: usize,
}

fn main() {
    let input = read_to_string("input/07.txt").unwrap();
    let mut hands = input
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(' ').unwrap();
            Hand {
                cards: cards.chars().collect::<Vec<char>>(),
                bid: bid.parse::<usize>().unwrap(),
            }
        })
        .collect::<Vec<Hand>>();

    hands.sort_by(compare_cards);
    let mut total_winnings: usize = 0;
    hands
        .iter()
        .enumerate()
        .for_each(|(i, hand)| total_winnings += (i + 1) * hand.bid);

    println!("First answer is {total_winnings}");
}

fn compare_cards(first: &Hand, second: &Hand) -> Ordering {
    let cards_order = [
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ];
    let first_type = find_type(first);
    let second_type = find_type(second);

    match first_type.cmp(&second_type) {
        Ordering::Less => return Ordering::Less,
        Ordering::Equal => {
            for i in 0..5 {
                match cards_order
                    .iter()
                    .position(|&e| e == first.cards[i])
                    .unwrap()
                    .cmp(
                        &cards_order
                            .iter()
                            .position(|&e| e == second.cards[i])
                            .unwrap(),
                    ) {
                    Ordering::Less => return Ordering::Greater,
                    Ordering::Equal => (),
                    Ordering::Greater => return Ordering::Less,
                }
            }
        }
        Ordering::Greater => return Ordering::Greater,
    }

    Ordering::Equal
}

fn find_type(hand: &Hand) -> usize {
    let mut elements: HashMap<char, usize> = HashMap::new();
    for e in &hand.cards {
        *elements.entry(*e).or_insert(0) += 1;
    }
    let amounts = elements.values().cloned().collect::<Vec<usize>>();
    if amounts.contains(&5) {
        return 6;
    } else if amounts.contains(&4) {
        return 5;
    } else if amounts.contains(&3) && amounts.contains(&2) {
        return 4;
    } else if amounts.contains(&3) {
        return 3;
    } else if amounts.iter().filter(|e| **e == 2).count() == 2 {
        return 2;
    } else if amounts.contains(&2) {
        return 1;
    }
    0
}
