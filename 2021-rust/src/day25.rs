use std::fs::read_to_string;

fn main() {
    let input: Vec<Vec<char>> = read_to_string("input/25")
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    part_one(input);
}

fn part_one(mut input: Vec<Vec<char>>) {
    let mut moved: bool = true;
    let mut steps = 0;
    while moved {
        steps += 1;
        moved = false;
        let mut new = input.clone();
        for (i, row) in input.iter().enumerate() {
            for (e, _column) in row.iter().enumerate() {
                if input[i][e] == '>' {
                    if e + 1 < input[i].len() && input[i][e + 1] == '.' {
                        new[i][e + 1] = '>';
                        new[i][e] = '.';
                        moved = true;
                    } else if e + 1 == input[i].len() && input[i][0] == '.' {
                        new[i][0] = '>';
                        new[i][e] = '.';
                        moved = true;
                    }
                }
            }
        }
        input = new;
        new = input.clone();
        for (i, row) in input.iter().enumerate() {
            for (e, _column) in row.iter().enumerate() {
                if input[i][e] == 'v' {
                    if i + 1 < input.len() && input[i + 1][e] == '.' {
                        new[i + 1][e] = 'v';
                        new[i][e] = '.';
                        moved = true;
                    } else if i + 1 == input.len() && input[0][e] == '.' {
                        new[0][e] = 'v';
                        new[i][e] = '.';
                        moved = true;
                    }
                }
            }
        }
        input = new;
    }
    println!("{}", steps);
}