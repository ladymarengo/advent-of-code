use std::fs::read_to_string;

#[derive(PartialEq)]
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

    let load = map
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter().filter(|space| **space == Space::Round).count() * (map.len() - i)
        })
        .sum::<usize>();

    println!("First answer is {load}");
}
