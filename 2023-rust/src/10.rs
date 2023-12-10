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

    println!("First answer is {}", part_one(&map, start));
}

fn part_one(map: &[Vec<char>], start: (usize, usize)) -> usize {
    let mut steps: usize = 1;
    let mut previous = start;
    let mut current = find_first_pipe(map, start);
    while current != start {
        let next = find_next(map, current, previous);
        previous = current;
        current = next;
        steps += 1;
    }
    steps / 2
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
