use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let input = read_to_string("input/06.txt").unwrap();
    let mut guard_pos = (0, 0);
    let map: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| {
                    if c == '^' {
                        guard_pos = (row as i32, col as i32)
                    }
                    c
                })
                .collect()
        })
        .collect();

    let positions = count_positions(&map, guard_pos);
    println!("First answer is {}", positions.len());

    let possible_obstacles = positions
        .iter()
        .filter(|position| check_obstacle(&map, guard_pos, **position))
        .count();
    println!("Second answer is {possible_obstacles}");
}

fn count_positions(map: &Vec<Vec<char>>, mut guard_pos: (i32, i32)) -> HashSet<(i32, i32)> {
    let mut positions: HashSet<(i32, i32)> = HashSet::from([guard_pos]);
    let mut next_pos;
    let mut direction = (-1, 0);
    loop {
        next_pos = (guard_pos.0 + direction.0, guard_pos.1 + direction.1);
        if next_pos.0 < 0
            || next_pos.0 == map.len() as i32
            || next_pos.1 < 0
            || next_pos.1 == map[0].len() as i32
        {
            break;
        } else if map[next_pos.0 as usize][next_pos.1 as usize] == '#' {
            direction = match direction {
                (-1, 0) => (0, 1),
                (0, 1) => (1, 0),
                (1, 0) => (0, -1),
                (0, -1) => (-1, 0),
                _ => (0, 0),
            }
        } else {
            guard_pos = next_pos;
            positions.insert(guard_pos);
        }
    }
    positions
}

fn check_obstacle(map: &Vec<Vec<char>>, mut guard_pos: (i32, i32), obstacle: (i32, i32)) -> bool {
    let mut direction = (-1, 0);
    let mut positions: HashSet<Position> = HashSet::from([Position {
        coords: guard_pos,
        direction,
    }]);
    let mut next_pos;
    loop {
        next_pos = (guard_pos.0 + direction.0, guard_pos.1 + direction.1);
        if next_pos.0 < 0
            || next_pos.0 == map.len() as i32
            || next_pos.1 < 0
            || next_pos.1 == map[0].len() as i32
        {
            return false;
        } else if map[next_pos.0 as usize][next_pos.1 as usize] == '#' || next_pos == obstacle {
            direction = match direction {
                (-1, 0) => (0, 1),
                (0, 1) => (1, 0),
                (1, 0) => (0, -1),
                (0, -1) => (-1, 0),
                _ => (0, 0),
            }
        } else {
            guard_pos = next_pos;
            let position = Position {
                coords: guard_pos,
                direction,
            };
            if positions.contains(&position) {
                return true;
            } else {
                positions.insert(position);
            }
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Position {
    coords: (i32, i32),
    direction: (i32, i32),
}
