use std::{collections::VecDeque, fs::read_to_string};

struct Cell {
    is_rock: bool,
    steps_from_start: u32,
}

fn main() {
    let input = read_to_string("input/21.txt").unwrap();

    let mut start = (0, 0);
    let mut map: Vec<Vec<Cell>> = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(column, c)| {
                    if c == 'S' {
                        start = (row, column);
                    }
                    Cell {
                        is_rock: c == '#',
                        steps_from_start: core::u32::MAX,
                    }
                })
                .collect()
        })
        .collect();

    fill_map(&mut map, start);
    let max_steps = 64;
    let result: usize = map
        .iter()
        .map(|row| {
            row.iter()
                .filter(|cell| {
                    !cell.is_rock
                        && cell.steps_from_start <= max_steps
                        && cell.steps_from_start % 2 == 0
                })
                .count()
        })
        .sum();
    println!("First answer is {result}");
}

fn fill_map(map: &mut [Vec<Cell>], start: (usize, usize)) {
    let mut open_list: VecDeque<(usize, usize)> = VecDeque::new();
    let mut checked: Vec<(usize, usize)> = Vec::new();
    map[start.0][start.1].steps_from_start = 0;
    open_list.push_back(start);
    while let Some(current) = open_list.pop_front() {
        let steps = map[current.0][current.1].steps_from_start;
        if current.0 > 0
            && !map[current.0 - 1][current.1].is_rock
            && !checked.contains(&(current.0 - 1, current.1))
            && !open_list.contains(&(current.0 - 1, current.1))
        {
            map[current.0 - 1][current.1].steps_from_start = steps + 1;
            open_list.push_back((current.0 - 1, current.1));
        }
        if current.0 < map.len() - 1
            && !map[current.0 + 1][current.1].is_rock
            && !checked.contains(&(current.0 + 1, current.1))
            && !open_list.contains(&(current.0 + 1, current.1))
        {
            map[current.0 + 1][current.1].steps_from_start = steps + 1;
            open_list.push_back((current.0 + 1, current.1));
        }
        if current.1 > 0
            && !map[current.0][current.1 - 1].is_rock
            && !checked.contains(&(current.0, current.1 - 1))
            && !open_list.contains(&(current.0, current.1 - 1))
        {
            map[current.0][current.1 - 1].steps_from_start = steps + 1;
            open_list.push_back((current.0, current.1 - 1));
        }
        if current.1 < map[0].len() - 1
            && !map[current.0][current.1 + 1].is_rock
            && !checked.contains(&(current.0, current.1 + 1))
            && !open_list.contains(&(current.0, current.1 + 1))
        {
            map[current.0][current.1 + 1].steps_from_start = steps + 1;
            open_list.push_back((current.0, current.1 + 1));
        }
        checked.push(current);
    }
}
