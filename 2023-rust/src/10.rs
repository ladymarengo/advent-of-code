use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/10.txt").unwrap();

    let mut map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    map.iter_mut().for_each(|line| {
        line.insert(0, '.');
        line.push('.');
    });
    map.insert(0, vec!['.'; map[0].len()]);
    map.push(vec!['.'; map[0].len()]);

    let mut start = (0, 0);
    map.iter().enumerate().for_each(|(y, line)| {
        if let Some(x) = line.iter().position(|c| *c == 'S') {
            start = (x, y)
        }
    });

    let mut path = find_path(&map, start);

    change_start_cell(&mut map, start);

    println!("First answer is {}", path.len() / 2);

    println!("Second answer is {}", count_enclosed(&map, &mut path));
}

fn change_start_cell(map: &mut [Vec<char>], start: (usize, usize)) {
    let north = map[start.1 - 1][start.0] == '|'
        || map[start.1 - 1][start.0] == '7'
        || map[start.1 - 1][start.0] == 'F';
    let east = map[start.1][start.0 + 1] == '-'
        || map[start.1][start.0 + 1] == '7'
        || map[start.1][start.0 + 1] == 'J';
    let south = map[start.1 + 1][start.0] == '|'
        || map[start.1 + 1][start.0] == 'J'
        || map[start.1 + 1][start.0] == 'L';
    let west = map[start.1][start.0 - 1] == '-'
        || map[start.1][start.0 - 1] == 'L'
        || map[start.1][start.0 - 1] == 'F';

    map[start.1][start.0] = if north && east {
        'L'
    } else if north && south {
        '|'
    } else if north && west {
        'J'
    } else if east && south {
        'F'
    } else if east && west {
        '-'
    } else {
        '7'
    };
}

fn find_path(map: &[Vec<char>], start: (usize, usize)) -> Vec<(usize, usize)> {
    let mut previous = start;
    let mut current = find_first_pipe(map, start);
    let mut path = vec![current];
    while current != start {
        let next = find_next(map, current, previous);
        previous = current;
        current = next;
        path.push(current);
    }
    path
}

fn count_enclosed(map: &[Vec<char>], path: &mut [(usize, usize)]) -> usize {
    let mut checked: Vec<(usize, usize)> = Vec::new();
    let mut amount = 0;
    let previous = find_first_corner(map, path);
    let mut previous_index = path.iter().position(|c| *c == previous).unwrap();
    let mut current_index;
    if previous_index + 1 < path.len() && path[previous_index + 1] == (previous.0 + 1, previous.1) {
        current_index = previous_index + 1;
    } else if previous_index + 1 == path.len() && path[0] == (previous.0 + 1, previous.1) {
        current_index = 0;
    } else {
        path.reverse();
        previous_index = path.iter().position(|c| *c == previous).unwrap();
        current_index = previous_index + 1;
    }
    let start_index = previous_index;
    while current_index != start_index {
        let cells_on_right = find_cells_on_right(map, path[previous_index], path[current_index]);
        cells_on_right.iter().for_each(|cell| {
            amount += count_neighbors(map, path, *cell, &mut checked);
        });
        previous_index = if previous_index < path.len() - 1 {
            previous_index + 1
        } else {
            0
        };
        current_index = if current_index < path.len() - 1 {
            current_index + 1
        } else {
            0
        };
    }
    amount
}

fn find_cells_on_right(
    map: &[Vec<char>],
    previous: (usize, usize),
    current: (usize, usize),
) -> Vec<(usize, usize)> {
    match map[current.1][current.0] {
        '|' => {
            if (current.0, current.1 - 1) == previous {
                vec![(current.0 - 1, current.1)]
            } else {
                vec![(current.0 + 1, current.1)]
            }
        }
        '-' => {
            if (current.0 - 1, current.1) == previous {
                vec![(current.0, current.1 + 1)]
            } else {
                vec![(current.0, current.1 - 1)]
            }
        }
        'L' => {
            if (current.0, current.1 - 1) == previous {
                vec![
                    (current.0 - 1, current.1),
                    (current.0 - 1, current.1 + 1),
                    (current.0, current.1 + 1),
                ]
            } else {
                vec![]
            }
        }
        'J' => {
            if (current.0, current.1 - 1) == previous {
                vec![]
            } else {
                vec![
                    (current.0 + 1, current.1),
                    (current.0 + 1, current.1 + 1),
                    (current.0, current.1 + 1),
                ]
            }
        }
        '7' => {
            if (current.0 - 1, current.1) == previous {
                vec![]
            } else {
                vec![
                    (current.0, current.1 - 1),
                    (current.0 + 1, current.1 - 1),
                    (current.0 + 1, current.1),
                ]
            }
        }
        'F' => {
            if (current.0 + 1, current.1) == previous {
                vec![
                    (current.0, current.1 - 1),
                    (current.0 - 1, current.1 - 1),
                    (current.0 - 1, current.1),
                ]
            } else {
                vec![]
            }
        }
        _ => vec![],
    }
}

fn find_first_corner(map: &[Vec<char>], path: &[(usize, usize)]) -> (usize, usize) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if path.contains(&(x, y)) {
                return (x, y);
            }
        }
    }
    (0, 0)
}

fn count_neighbors(
    map: &[Vec<char>],
    path: &[(usize, usize)],
    cell: (usize, usize),
    checked: &mut Vec<(usize, usize)>,
) -> usize {
    if path.contains(&cell) || checked.contains(&cell) {
        return 0;
    }
    let mut amount = 0;
    let mut open = vec![cell];
    while let Some(current) = open.pop() {
        (-1..=1).for_each(|x| {
            (-1..=1).for_each(|y| {
                if current.0 as i32 + x >= 0
                    && current.1 as i32 + y >= 0
                    && current.0 as i32 + x < map[0].len() as i32
                    && current.1 as i32 + y < map.len() as i32
                {
                    let new = (
                        (current.0 as i32 + x) as usize,
                        (current.1 as i32 + y) as usize,
                    );
                    if !path.contains(&new)
                        && !checked.contains(&new)
                        && !open.contains(&new)
                        && new != current
                    {
                        open.push(new);
                    }
                }
            })
        });
        checked.push(current);
        amount += 1;
    }
    amount
}

fn find_next(
    map: &[Vec<char>],
    current: (usize, usize),
    previous: (usize, usize),
) -> (usize, usize) {
    match map[current.1][current.0] {
        '|' => {
            if (current.0, current.1 - 1) != previous {
                return (current.0, current.1 - 1);
            } else {
                return (current.0, current.1 + 1);
            }
        }
        '-' => {
            if (current.0 - 1, current.1) != previous {
                return (current.0 - 1, current.1);
            } else {
                return (current.0 + 1, current.1);
            }
        }
        'L' => {
            if (current.0, current.1 - 1) != previous {
                return (current.0, current.1 - 1);
            } else {
                return (current.0 + 1, current.1);
            }
        }
        'J' => {
            if (current.0, current.1 - 1) != previous {
                return (current.0, current.1 - 1);
            } else {
                return (current.0 - 1, current.1);
            }
        }
        '7' => {
            if (current.0 - 1, current.1) != previous {
                return (current.0 - 1, current.1);
            } else {
                return (current.0, current.1 + 1);
            }
        }
        'F' => {
            if (current.0 + 1, current.1) != previous {
                return (current.0 + 1, current.1);
            } else {
                return (current.0, current.1 + 1);
            }
        }
        _ => (),
    }
    (0, 0)
}

fn find_first_pipe(map: &[Vec<char>], start: (usize, usize)) -> (usize, usize) {
    if map[start.1][start.0 - 1] == '-'
        || map[start.1][start.0 - 1] == 'L'
        || map[start.1][start.0 - 1] == 'F'
    {
        return (start.0 - 1, start.1);
    } else if map[start.1][start.0 + 1] == '-'
        || map[start.1][start.0 - 1] == 'J'
        || map[start.1][start.0 - 1] == '7'
    {
        return (start.0 + 1, start.1);
    } else if map[start.1 - 1][start.0] == '|'
        || map[start.1 - 1][start.0] == '7'
        || map[start.1 - 1][start.0] == 'F'
    {
        return (start.0 - 1, start.1);
    } else if map[start.1 + 1][start.0] == '|'
        || map[start.1 + 1][start.0] == 'L'
        || map[start.1 + 1][start.0] == 'J'
    {
        return (start.0 + 1, start.1);
    }
    (0, 0)
}
