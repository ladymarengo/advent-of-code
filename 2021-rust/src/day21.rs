use std::{
    cmp::{max, min},
    collections::HashMap,
};

fn main() {
    let input: Vec<usize> = vec![4, 2];
    println!("First answer is {}", part_one(input.clone()));
    println!("Second answer is {}", part_two(input));
}

fn part_one(mut positions: Vec<usize>) -> usize {
    let mut scores: Vec<usize> = vec![0, 0];
    let mut rolls: usize = 0;
    let mut dice_value: usize = 0;
    let mut turn: usize = 0;

    while scores[0] < 1000 && scores[1] < 1000 {
        for _i in 0..3 {
            dice_value += 1;
            if dice_value > 100 {
                dice_value = 1;
            }
            rolls += 1;
            positions[turn] += dice_value % 10;
            if positions[turn] > 10 {
                positions[turn] -= 10;
            }
        }
        scores[turn] += positions[turn];
        if turn == 1 {
            turn = 0;
        } else {
            turn = 1;
        }
    }
    rolls * min(scores[0], scores[1])
}

fn part_two(position: Vec<usize>) -> usize {

    let mut players: Vec<HashMap<usize, HashMap<usize, usize>>> = vec![
        HashMap::from([(position[0], HashMap::from([(0, 1)]))]),
        HashMap::from([(position[1], HashMap::from([(0, 1)]))]),
    ];
    let mut turn: usize = 0;
    let mut wins: Vec<usize> = vec![0, 0];

    while !players[0].is_empty() && !players[1].is_empty() {
        let mut new: HashMap<usize, HashMap<usize, usize>> = HashMap::new();
        
        for position in players[turn].keys() {
            for a in 1..=3 {
                for b in 1..=3 {
                    for c in 1..=3 {
                        let current = if position + a + b + c > 10 {
                            position + a + b + c - 10
                        } else {
                            position + a + b + c
                        };
                        for score in players[turn][position].keys() {
                            *new.entry(current)
                                .or_default()
                                .entry(score + current)
                                .or_default() += players[turn][position][score];
                        }
                    }
                }
            }
        }
        players[turn] = new;

        let other_player: usize = if turn == 1 { 0 } else { 1 };

        new = HashMap::new();
        for position in players[turn].keys() {
            for score in players[turn][position].keys() {
                if *score >= 21 {
                    wins[turn] +=
                        count_wins(players[turn][position][score], &players[other_player]);
                } else {
                    *new.entry(*position).or_default().entry(*score).or_default() +=
                        players[turn][position][score];
                }
            }
        }
        players[turn] = new;
        
        turn = other_player;
    }
    
    max(wins[0], wins[1])
}

fn count_wins(amount: usize, player: &HashMap<usize, HashMap<usize, usize>>) -> usize {
    let mut versions: usize = 0;
    for position in player.keys() {
        for score in player[position].keys() {
            versions += amount * player[position][score];
        }
    }
    versions
}