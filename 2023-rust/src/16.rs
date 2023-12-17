use std::fs::read_to_string;

#[derive(PartialEq, Clone)]
struct Tile {
    symbol: char,
    directions: Vec<(Direction, Direction)>,
}

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn main() {
    let input = read_to_string("input/16.txt").unwrap();

    let tiles: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Tile {
                    symbol: c,
                    directions: Vec::new(),
                })
                .collect::<Vec<Tile>>()
        })
        .collect();

    let mut answer = 0;
    for i in 0..tiles.len() {
        let mut new_tiles = tiles.clone();
        run_beam(&mut new_tiles, (0, i), Direction::Left);
        let new: usize = new_tiles
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|tile| !tile.directions.is_empty())
                    .count()
            })
            .sum();
        if new > answer {
            answer = new;
        }

        let mut new_tiles = tiles.clone();
        run_beam(&mut new_tiles, (tiles[0].len() - 1, i), Direction::Right);
        let new: usize = new_tiles
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|tile| !tile.directions.is_empty())
                    .count()
            })
            .sum();
        if new > answer {
            answer = new;
        }
    }

    for i in 0..tiles[0].len() {
        let mut new_tiles = tiles.clone();
        run_beam(&mut new_tiles, (i, 0), Direction::Up);
        let new: usize = new_tiles
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|tile| !tile.directions.is_empty())
                    .count()
            })
            .sum();
        if new > answer {
            answer = new;
        }

        let mut new_tiles = tiles.clone();
        run_beam(&mut new_tiles, (i, tiles.len() - 1), Direction::Down);
        let new: usize = new_tiles
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|tile| !tile.directions.is_empty())
                    .count()
            })
            .sum();
        if new > answer {
            answer = new;
        }
    }

    println!("{answer}");
}

fn run_beam(tiles: &mut Vec<Vec<Tile>>, mut start: (usize, usize), mut direction: Direction) {
    loop {
        match tiles[start.1][start.0].symbol {
            '.' => match direction {
                Direction::Up => {
                    tiles[start.1][start.0]
                        .directions
                        .push((direction, Direction::Down));
                    if start.1 < tiles.len() - 1
                        && !tiles[start.1 + 1][start.0]
                            .directions
                            .iter()
                            .any(|direction| direction.0 == Direction::Up)
                    {
                        start.1 += 1;
                    } else {
                        break;
                    }
                }
                Direction::Right => {
                    tiles[start.1][start.0]
                        .directions
                        .push((direction, Direction::Left));
                    if start.0 > 0
                        && !tiles[start.1][start.0 - 1]
                            .directions
                            .iter()
                            .any(|direction| direction.0 == Direction::Right)
                    {
                        start.0 -= 1;
                    } else {
                        break;
                    }
                }
                Direction::Down => {
                    tiles[start.1][start.0]
                        .directions
                        .push((direction, Direction::Up));
                    if start.1 > 0
                        && !tiles[start.1 - 1][start.0]
                            .directions
                            .iter()
                            .any(|direction| direction.0 == Direction::Down)
                    {
                        start.1 -= 1;
                    } else {
                        break;
                    }
                }
                Direction::Left => {
                    tiles[start.1][start.0]
                        .directions
                        .push((direction, Direction::Right));
                    if start.0 < tiles[0].len() - 1
                        && !tiles[start.1][start.0 + 1]
                            .directions
                            .iter()
                            .any(|direction| direction.0 == Direction::Left)
                    {
                        start.0 += 1;
                    } else {
                        break;
                    }
                }
            },
            '-' => match direction {
                Direction::Up | Direction::Down => {
                    tiles[start.1][start.0]
                        .directions
                        .push((direction, Direction::Left));
                    if start.0 > 0
                        && !tiles[start.1][start.0 - 1]
                            .directions
                            .iter()
                            .any(|direction| direction.0 == Direction::Right)
                    {
                        run_beam(tiles, (start.0 - 1, start.1), Direction::Right);
                    }
                    tiles[start.1][start.0]
                        .directions
                        .push((direction, Direction::Right));
                    if start.0 < tiles[0].len() - 1
                        && !tiles[start.1][start.0 + 1]
                            .directions
                            .iter()
                            .any(|direction| direction.0 == Direction::Left)
                    {
                        start.0 += 1;
                        direction = Direction::Left;
                    } else {
                        break;
                    }
                }
                Direction::Right => {
                    tiles[start.1][start.0]
                        .directions
                        .push((direction, Direction::Left));
                    if start.0 > 0
                        && !tiles[start.1][start.0 - 1]
                            .directions
                            .iter()
                            .any(|direction| direction.0 == Direction::Right)
                    {
                        start.0 -= 1;
                    } else {
                        break;
                    }
                }
                Direction::Left => {
                    tiles[start.1][start.0]
                        .directions
                        .push((direction, Direction::Right));
                    if start.0 < tiles[0].len() - 1
                        && !tiles[start.1][start.0 + 1]
                            .directions
                            .iter()
                            .any(|direction| direction.0 == Direction::Left)
                    {
                        start.0 += 1;
                    } else {
                        break;
                    }
                }
            },
            '|' => match direction {
                Direction::Up => {
                    tiles[start.1][start.0]
                        .directions
                        .push((direction, Direction::Down));
                    if start.1 < tiles.len() - 1
                        && !tiles[start.1 + 1][start.0]
                            .directions
                            .iter()
                            .any(|direction| direction.0 == Direction::Up)
                    {
                        start.1 += 1;
                    } else {
                        break;
                    }
                }
                Direction::Right | Direction::Left => {
                    tiles[start.1][start.0]
                        .directions
                        .push((direction, Direction::Down));
                    if start.1 < tiles.len() - 1
                        && !tiles[start.1 + 1][start.0]
                            .directions
                            .iter()
                            .any(|direction| direction.0 == Direction::Up)
                    {
                        run_beam(tiles, (start.0, start.1 + 1), Direction::Up);
                    }
                    tiles[start.1][start.0]
                        .directions
                        .push((direction, Direction::Up));
                    if start.1 > 0
                        && !tiles[start.1 - 1][start.0]
                            .directions
                            .iter()
                            .any(|direction| direction.0 == Direction::Down)
                    {
                        start.1 -= 1;
                        direction = Direction::Down;
                    } else {
                        break;
                    }
                }
                Direction::Down => {
                    tiles[start.1][start.0]
                        .directions
                        .push((direction, Direction::Up));
                    if start.1 > 0
                        && !tiles[start.1 - 1][start.0]
                            .directions
                            .iter()
                            .any(|direction| direction.0 == Direction::Down)
                    {
                        start.1 -= 1;
                    } else {
                        break;
                    }
                }
            },
            '/' => match direction {
                Direction::Right => {
                    tiles[start.1][start.0]
                        .directions
                        .push((direction, Direction::Down));
                    if start.1 < tiles.len() - 1
                        && !tiles[start.1 + 1][start.0]
                            .directions
                            .iter()
                            .any(|direction| direction.0 == Direction::Up)
                    {
                        start.1 += 1;
                        direction = Direction::Up;
                    } else {
                        break;
                    }
                }
                Direction::Up => {
                    tiles[start.1][start.0]
                        .directions
                        .push((direction, Direction::Left));
                    if start.0 > 0
                        && !tiles[start.1][start.0 - 1]
                            .directions
                            .iter()
                            .any(|direction| direction.0 == Direction::Right)
                    {
                        start.0 -= 1;
                        direction = Direction::Right;
                    } else {
                        break;
                    }
                }
                Direction::Left => {
                    tiles[start.1][start.0]
                        .directions
                        .push((direction, Direction::Up));
                    if start.1 > 0
                        && !tiles[start.1 - 1][start.0]
                            .directions
                            .iter()
                            .any(|direction| direction.0 == Direction::Down)
                    {
                        start.1 -= 1;
                        direction = Direction::Down;
                    } else {
                        break;
                    }
                }
                Direction::Down => {
                    tiles[start.1][start.0]
                        .directions
                        .push((direction, Direction::Right));
                    if start.0 < tiles[0].len() - 1
                        && !tiles[start.1][start.0 + 1]
                            .directions
                            .iter()
                            .any(|direction| direction.0 == Direction::Left)
                    {
                        start.0 += 1;
                        direction = Direction::Left;
                    } else {
                        break;
                    }
                }
            },
            '\\' => match direction {
                Direction::Left => {
                    tiles[start.1][start.0]
                        .directions
                        .push((direction, Direction::Down));
                    if start.1 < tiles.len() - 1
                        && !tiles[start.1 + 1][start.0]
                            .directions
                            .iter()
                            .any(|direction| direction.0 == Direction::Up)
                    {
                        start.1 += 1;
                        direction = Direction::Up;
                    } else {
                        break;
                    }
                }
                Direction::Down => {
                    tiles[start.1][start.0]
                        .directions
                        .push((direction, Direction::Left));
                    if start.0 > 0
                        && !tiles[start.1][start.0 - 1]
                            .directions
                            .iter()
                            .any(|direction| direction.0 == Direction::Right)
                    {
                        start.0 -= 1;
                        direction = Direction::Right;
                    } else {
                        break;
                    }
                }
                Direction::Right => {
                    tiles[start.1][start.0]
                        .directions
                        .push((direction, Direction::Up));
                    if start.1 > 0
                        && !tiles[start.1 - 1][start.0]
                            .directions
                            .iter()
                            .any(|direction| direction.0 == Direction::Down)
                    {
                        start.1 -= 1;
                        direction = Direction::Down;
                    } else {
                        break;
                    }
                }
                Direction::Up => {
                    tiles[start.1][start.0]
                        .directions
                        .push((direction, Direction::Right));
                    if start.0 < tiles[0].len() - 1
                        && !tiles[start.1][start.0 + 1]
                            .directions
                            .iter()
                            .any(|direction| direction.0 == Direction::Left)
                    {
                        start.0 += 1;
                        direction = Direction::Left;
                    } else {
                        break;
                    }
                }
            },
            _ => (),
        }
    }
}
