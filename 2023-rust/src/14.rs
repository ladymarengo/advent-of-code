use std::fs::read_to_string;

#[derive(PartialEq, Clone)]
enum Space {
    Empty,
    Round,
    Cube,
}

fn main() {
    let input = read_to_string("input/14.txt").unwrap();

    let mut map: Vec<Vec<Space>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Space::Empty,
                    'O' => Space::Round,
                    '#' => Space::Cube,
                    _ => panic!(),
                })
                .collect()
        })
        .collect();

    let mut first = map.clone();
    let mut second = map.clone();
    let mut cycle_start = 0;

    loop {
        run_cycle(&mut first);

        run_cycle(&mut second);
        run_cycle(&mut second);
        cycle_start += 1;
        if first == second {
            break;
        }
    }

    let mut steps = 0;
    loop {
        run_cycle(&mut first);

        run_cycle(&mut second);
        run_cycle(&mut second);
        steps += 1;
        if first == second {
            break;
        }
    }

    let cycles_amount = cycle_start + ((1000000000 - cycle_start) % steps);
    for _i in 0..cycles_amount {
        run_cycle(&mut map);
    }

    let load = map
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter().filter(|space| **space == Space::Round).count() * (map.len() - i)
        })
        .sum::<usize>();

    println!("Second answer is {load}");
}

fn run_cycle(map: &mut Vec<Vec<Space>>) {
    (0..map[0].len()).for_each(|column| {
        let mut bottom = 0;
        (0..map.len()).for_each(|row| {
            if map[row][column] == Space::Round {
                if bottom < row {
                    map[bottom][column] = Space::Round;
                    map[row][column] = Space::Empty;
                    while map[bottom][column] != Space::Empty && bottom < map.len() {
                        bottom += 1;
                    }
                } else {
                    bottom += 1;
                }
            } else if map[row][column] == Space::Cube {
                bottom = row + 1;
            }
        });
    });

    (0..map.len()).for_each(|row| {
        let mut bottom = 0;
        (0..map[0].len()).for_each(|column| {
            if map[row][column] == Space::Round {
                if bottom < column {
                    map[row][bottom] = Space::Round;
                    map[row][column] = Space::Empty;
                    while map[row][bottom] != Space::Empty && bottom < map[0].len() {
                        bottom += 1;
                    }
                } else {
                    bottom += 1;
                }
            } else if map[row][column] == Space::Cube {
                bottom = column + 1;
            }
        });
    });

    (0..map[0].len()).for_each(|column| {
        let mut bottom = map.len() - 1;
        (0..map.len()).rev().for_each(|row| {
            if map[row][column] == Space::Round {
                if bottom > row {
                    map[bottom][column] = Space::Round;
                    map[row][column] = Space::Empty;
                    while map[bottom][column] != Space::Empty && bottom > 0 {
                        bottom -= 1;
                    }
                } else {
                    bottom = bottom.saturating_sub(1);
                }
            } else if map[row][column] == Space::Cube {
                bottom = row - 1;
            }
        });
    });

    (0..map.len()).for_each(|row| {
        let mut bottom = map[0].len() - 1;
        (0..map[0].len()).rev().for_each(|column| {
            if map[row][column] == Space::Round {
                if bottom > column {
                    map[row][bottom] = Space::Round;
                    map[row][column] = Space::Empty;
                    while map[row][bottom] != Space::Empty && bottom > 0 {
                        bottom -= 1;
                    }
                } else {
                    bottom = bottom.saturating_sub(1);
                }
            } else if map[row][column] == Space::Cube {
                bottom = column - 1;
            }
        });
    });
}
