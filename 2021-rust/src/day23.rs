/*This code is very ugly because it's Christmas Eve tomorrow and I don't have any time to make it better. 
It works and that's enough.*/

use std::cmp::{max, min};
use std::fs::read_to_string;

#[derive(Debug)]
struct Move(usize, usize, usize);

fn main() {
    let (room, mut amphipods) = parse_input();
    amphipods.sort_by_key(|k| k.0);
    solve(room, amphipods);
}

fn parse_input() -> (Vec<Vec<char>>, Vec<(char, (usize, usize))>) {
    let room: Vec<Vec<char>> = read_to_string("input/23")
        .unwrap()
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut amphipods: Vec<(char, (usize, usize))> = vec![];
    for (e, line) in room.iter().enumerate() {
        for (i, char) in line.iter().enumerate() {
            if *char != '.' && *char != '#' {
                amphipods.push((*char, (i, e)));
            }
        }
    }
    (room, amphipods)
}

fn solve(mut room: Vec<Vec<char>>, mut amphipods: Vec<(char, (usize, usize))>) {
    let mut best_cost = 0;
    move_amphipod(&mut room, &mut amphipods, 0, &mut best_cost);
    println!("{}", best_cost);
}

fn move_amphipod(
    room: &mut Vec<Vec<char>>,
    amphipods: &mut Vec<(char, (usize, usize))>,
    cost: usize,
    best_cost: &mut usize,
) {
    if all_home(amphipods) {
        if cost < *best_cost || *best_cost == 0 {
            *best_cost = cost;
        }
        println!("found {}", *best_cost);
        return;
    }
    for a in 0..amphipods.len() {
        if is_home(&amphipods[a])
            && ((amphipods[a].1 .1 == 5
                || (amphipods[a].1 .1 == 4 && room[5][amphipods[a].1 .0] == amphipods[a].0))
                || (amphipods[a].1 .1 == 3
                    && room[5][amphipods[a].1 .0] == amphipods[a].0
                    && room[4][amphipods[a].1 .0] == amphipods[a].0)
                || (amphipods[a].1 .1 == 2
                    && room[5][amphipods[a].1 .0] == amphipods[a].0
                    && room[4][amphipods[a].1 .0] == amphipods[a].0
                    && room[3][amphipods[a].1 .0] == amphipods[a].0))
        {
            continue;
        }
        if amphipods[a].1.1 == 1 {
            let mut possible_moves = get_possible_moves(&room, &amphipods[a]);
            possible_moves.sort_by_key(|k| k.2);
            for possible_move in possible_moves {
                let mut new_cost = cost + possible_move.2;
                let old_x = amphipods[a].1 .0;
                let old_y = amphipods[a].1 .1;
                room[old_y][old_x] = '.';
                room[possible_move.1][possible_move.0] = amphipods[a].0;
                amphipods[a].1 .0 = possible_move.0;
                amphipods[a].1 .1 = possible_move.1;
                if new_cost < *best_cost || *best_cost == 0 {
                    move_amphipod(room, amphipods, new_cost, best_cost);
                }
                room[old_y][old_x] = amphipods[a].0;
                room[possible_move.1][possible_move.0] = '.';
                amphipods[a].1 .0 = old_x;
                amphipods[a].1 .1 = old_y;
            }
        }
    }
    for a in 0..amphipods.len() {
        if is_home(&amphipods[a])
            && ((amphipods[a].1 .1 == 5
                || (amphipods[a].1 .1 == 4 && room[5][amphipods[a].1 .0] == amphipods[a].0))
                || (amphipods[a].1 .1 == 3
                    && room[5][amphipods[a].1 .0] == amphipods[a].0
                    && room[4][amphipods[a].1 .0] == amphipods[a].0)
                || (amphipods[a].1 .1 == 2
                    && room[5][amphipods[a].1 .0] == amphipods[a].0
                    && room[4][amphipods[a].1 .0] == amphipods[a].0
                    && room[3][amphipods[a].1 .0] == amphipods[a].0))
        {
            continue;
        }
        if amphipods[a].1.1 != 1 {
            let mut possible_moves = get_possible_moves(&room, &amphipods[a]);
            possible_moves.sort_by_key(|k| k.2);
            for possible_move in possible_moves {
                let mut new_cost = cost + possible_move.2;
                let old_x = amphipods[a].1 .0;
                let old_y = amphipods[a].1 .1;
                room[old_y][old_x] = '.';
                room[possible_move.1][possible_move.0] = amphipods[a].0;
                amphipods[a].1 .0 = possible_move.0;
                amphipods[a].1 .1 = possible_move.1;
                if new_cost < *best_cost || *best_cost == 0 {
                    move_amphipod(room, amphipods, new_cost, best_cost);
                }
                room[old_y][old_x] = amphipods[a].0;
                room[possible_move.1][possible_move.0] = '.';
                amphipods[a].1 .0 = old_x;
                amphipods[a].1 .1 = old_y;
            }
        }
    }
}

fn all_home(amphipods: &[(char, (usize, usize))]) -> bool {
    for amphipod in amphipods {
        if !is_home(amphipod) {
            return false;
        }
    }
    true
}

fn is_home(amphipod: &(char, (usize, usize))) -> bool {
    match amphipod.0 {
        'A' if amphipod.1 .0 == 3 && (amphipod.1 .1 >= 2) => true,
        'B' if amphipod.1 .0 == 5 && (amphipod.1 .1 >= 2) => true,
        'C' if amphipod.1 .0 == 7 && (amphipod.1 .1 >= 2) => true,
        'D' if amphipod.1 .0 == 9 && (amphipod.1 .1 >= 2) => true,
        _ => false,
    }
}

fn get_possible_moves(room: &Vec<Vec<char>>, amphipod: &(char, (usize, usize))) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    if !is_home(amphipod) {
        match amphipod.0 {
            'A' => {
                if way_is_clear(room, amphipod.1 .0, amphipod.1 .1, 3, 5) {
                    moves.push(Move(3, 5, calculate_cost(amphipod, 3, 5)))
                } else if way_is_clear(room, amphipod.1 .0, amphipod.1 .1, 3, 4)
                    && room[5][3] == amphipod.0
                {
                    moves.push(Move(3, 4, calculate_cost(amphipod, 3, 4)))
                } else if way_is_clear(room, amphipod.1 .0, amphipod.1 .1, 3, 3)
                    && room[5][3] == amphipod.0
                    && room[4][3] == amphipod.0
                {
                    moves.push(Move(3, 3, calculate_cost(amphipod, 3, 3)))
                } else if way_is_clear(room, amphipod.1 .0, amphipod.1 .1, 3, 2)
                    && room[5][3] == amphipod.0
                    && room[4][3] == amphipod.0
                    && room[3][3] == amphipod.0
                {
                    moves.push(Move(3, 2, calculate_cost(amphipod, 3, 2)))
                }
            }
            'B' => {
                if way_is_clear(room, amphipod.1 .0, amphipod.1 .1, 5, 5) {
                    moves.push(Move(5, 5, calculate_cost(amphipod, 5, 5)))
                } else if way_is_clear(room, amphipod.1 .0, amphipod.1 .1, 5, 4)
                    && room[5][5] == amphipod.0
                {
                    moves.push(Move(5, 4, calculate_cost(amphipod, 5, 4)))
                } else if way_is_clear(room, amphipod.1 .0, amphipod.1 .1, 5, 3)
                    && room[5][5] == amphipod.0
                    && room[4][5] == amphipod.0
                {
                    moves.push(Move(5, 3, calculate_cost(amphipod, 5, 3)))
                } else if way_is_clear(room, amphipod.1 .0, amphipod.1 .1, 5, 2)
                    && room[5][5] == amphipod.0
                    && room[4][5] == amphipod.0
                    && room[3][5] == amphipod.0
                {
                    moves.push(Move(5, 2, calculate_cost(amphipod, 5, 2)))
                }
            }
            'C' => {
                if way_is_clear(room, amphipod.1 .0, amphipod.1 .1, 7, 5) {
                    moves.push(Move(7, 5, calculate_cost(amphipod, 7, 5)))
                } else if way_is_clear(room, amphipod.1 .0, amphipod.1 .1, 7, 4)
                    && room[5][7] == amphipod.0
                {
                    moves.push(Move(7, 4, calculate_cost(amphipod, 7, 4)))
                } else if way_is_clear(room, amphipod.1 .0, amphipod.1 .1, 7, 3)
                    && room[5][7] == amphipod.0
                    && room[4][7] == amphipod.0
                {
                    moves.push(Move(7, 3, calculate_cost(amphipod, 7, 3)))
                } else if way_is_clear(room, amphipod.1 .0, amphipod.1 .1, 7, 2)
                    && room[5][7] == amphipod.0
                    && room[4][7] == amphipod.0
                    && room[3][7] == amphipod.0
                {
                    moves.push(Move(7, 2, calculate_cost(amphipod, 7, 2)))
                }
            }
            'D' => {
                if way_is_clear(room, amphipod.1 .0, amphipod.1 .1, 9, 5) {
                    moves.push(Move(9, 5, calculate_cost(amphipod, 9, 5)))
                } else if way_is_clear(room, amphipod.1 .0, amphipod.1 .1, 9, 4)
                    && room[5][9] == amphipod.0
                {
                    moves.push(Move(9, 4, calculate_cost(amphipod, 9, 4)))
                } else if way_is_clear(room, amphipod.1 .0, amphipod.1 .1, 9, 3)
                    && room[5][9] == amphipod.0
                    && room[4][9] == amphipod.0
                {
                    moves.push(Move(9, 3, calculate_cost(amphipod, 9, 3)))
                } else if way_is_clear(room, amphipod.1 .0, amphipod.1 .1, 9, 2)
                    && room[5][9] == amphipod.0
                    && room[4][9] == amphipod.0
                    && room[3][9] == amphipod.0
                {
                    moves.push(Move(9, 2, calculate_cost(amphipod, 9, 2)))
                }
            }
            _ => (),
        }
    }
    if amphipod.1 .1 != 1 {
        for x in [1, 2, 4, 6, 8, 10, 11] {
            if way_is_clear(room, amphipod.1 .0, amphipod.1 .1, x, 1) {
                moves.push(Move(x, 1, calculate_cost(amphipod, x, 1)))
            }
        }
    }
    moves
}

fn way_is_clear(
    room: &[Vec<char>],
    current_x: usize,
    mut current_y: usize,
    new_x: usize,
    new_y: usize,
) -> bool {

    if current_y > 1 {
        for y in 1..current_y {
            if room[y][current_x] != '.' {
                return false;
            }
        }
    }
    if current_x < new_x {
        for x in current_x + 1..=new_x {
            if room[1][x] != '.' {
                return false;
            }
        }
    } else {
        for x in new_x..current_x {
            if room[1][x] != '.' {
                return false;
            }
        }
    }

    if new_y > 1 {
        for y in 1..=new_y {
            if room[y][new_x] != '.' {
                return false;
            }
        }
    }
    true
}

fn calculate_cost(amphipod: &(char, (usize, usize)), new_x: usize, new_y: usize) -> usize {
    let mut steps: usize = 0;
    if amphipod.1 .1 > 1 {
        for _y in 1..amphipod.1 .1 {
            steps += 1;
        }
    }
    for _x in min(amphipod.1 .0, new_x)..max(amphipod.1 .0, new_x) {
        steps += 1;
    }
    if new_y > 1 {
        for _y in 1..new_y {
            steps += 1;
        }
    }
    match amphipod.0 {
        'A' => steps,
        'B' => steps * 10,
        'C' => steps * 100,
        'D' => steps * 1000,
        _ => 0,
    }
}
