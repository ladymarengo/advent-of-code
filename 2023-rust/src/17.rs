use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/17.txt").unwrap();

    let map: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    println!("Fisrt answer is {}", find_path(&map));
}

fn find_path(map: &[Vec<usize>]) -> usize {
    let end = (map.len() - 1, map[0].len() - 1);
    let mut heat = core::usize::MAX;

    let mut closed: Vec<Block> = Vec::new();
    let mut open: Vec<Block> = vec![
        Block {
            row: 0,
            column: 0,
            direction: Direction::Right,
            straight_line: 1,
            heat: 0,
        },
        Block {
            row: 0,
            column: 0,
            direction: Direction::Down,
            straight_line: 1,
            heat: 0,
        },
    ];

    while let Some(current) = open.pop() {
        if current.row == end.0 && current.column == end.1 {
            if current.heat < heat {
                heat = current.heat;
                println!("{:?}", current);
            }
        } else {
            if current.row > 0
                && current.direction != Direction::Down
                && !(current.direction == Direction::Up && current.straight_line >= 3)
            {
                let straight_line = if current.direction == Direction::Up {
                    current.straight_line + 1
                } else {
                    1
                };
                let new = Block {
                    row: current.row - 1,
                    column: current.column,
                    direction: Direction::Up,
                    straight_line,
                    heat: current.heat + map[current.row - 1][current.column],
                };
                if valid_step(&new, &mut open, &closed) {
                    open.push(new);
                }
            }
            if current.row < map.len() - 1
                && current.direction != Direction::Up
                && !(current.direction == Direction::Down && current.straight_line >= 3)
            {
                let straight_line = if current.direction == Direction::Down {
                    current.straight_line + 1
                } else {
                    1
                };
                let new = Block {
                    row: current.row + 1,
                    column: current.column,
                    direction: Direction::Down,
                    straight_line,
                    heat: current.heat + map[current.row + 1][current.column],
                };
                if valid_step(&new, &mut open, &closed) {
                    open.push(new);
                }
            }
            if current.column > 0
                && current.direction != Direction::Right
                && !(current.direction == Direction::Left && current.straight_line >= 3)
            {
                let straight_line = if current.direction == Direction::Left {
                    current.straight_line + 1
                } else {
                    1
                };
                let new = Block {
                    row: current.row,
                    column: current.column - 1,
                    direction: Direction::Left,
                    straight_line,
                    heat: current.heat + map[current.row][current.column - 1],
                };
                if valid_step(&new, &mut open, &closed) {
                    open.push(new);
                }
            }
            if current.column < map[0].len() - 1
                && current.direction != Direction::Left
                && !(current.direction == Direction::Right && current.straight_line >= 3)
            {
                let straight_line = if current.direction == Direction::Right {
                    current.straight_line + 1
                } else {
                    1
                };
                let new = Block {
                    row: current.row,
                    column: current.column + 1,
                    direction: Direction::Right,
                    straight_line,
                    heat: current.heat + map[current.row][current.column + 1],
                };
                if valid_step(&new, &mut open, &closed) {
                    open.push(new);
                }
            }
        }
        closed.push(current);
    }

    heat
}

fn valid_step(current: &Block, open: &mut [Block], closed: &[Block]) -> bool {
    if let Some(position) = open.iter_mut().find(|block| {
        block.row == current.row
            && block.column == current.column
            && ((block.straight_line == 1 && current.straight_line == 1)
                || (block.direction == current.direction
                    && block.straight_line == current.straight_line))
    }) {
        if position.heat > current.heat {
            position.heat = current.heat;
        }
        return false;
    }
    if closed.iter().any(|block| {
        block.row == current.row
            && block.column == current.column
            && block.heat < current.heat
            && ((block.straight_line == 1 && current.straight_line == 1)
                || (block.direction == current.direction
                    && block.straight_line == current.straight_line))
    }) {
        return false;
    }
    true
}

#[derive(PartialEq, Copy, Clone, Debug)]
struct Block {
    row: usize,
    column: usize,
    direction: Direction,
    straight_line: usize,
    heat: usize,
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
