use std::{
    cmp::{max, min},
    fs::read_to_string,
};

fn main() {
    let input: Vec<Vec<(usize, usize)>> = read_to_string("input/14.txt")
        .unwrap()
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(|c| {
                    let coords = c.split_once(',').unwrap();
                    (
                        coords.0.parse::<usize>().unwrap(),
                        coords.1.parse::<usize>().unwrap(),
                    )
                })
                .collect()
        })
        .collect();

    let mut x: (usize, usize) = (1000, 0);
    let mut y: (usize, usize) = (0, 0);
    input.iter().for_each(|rock| {
        rock.iter().for_each(|part| {
            x = (min(x.0, part.0), max(x.1, part.0));
            y.1 = max(y.1, part.1);
        })
    });

    // Part one

    let mut cave = create_cave(&input, x, y);
    let mut sand: (usize, usize) = (500 - x.0, 0);
    println!("{}", solve(cave, sand));

    // Part two
    x = (min(x.0, 500 - y.1 - 3), max(x.1, 500 + y.1 + 3));
    cave = create_cave(&input, x, y);
    cave.push(vec![false; x.1 - x.0 + 1]);
    cave.push(vec![true; x.1 - x.0 + 1]);
    sand = (500 - x.0, 0);
    println!("{}", solve(cave, sand) + 1);
}

fn create_cave(
    input: &[Vec<(usize, usize)>],
    x: (usize, usize),
    y: (usize, usize),
) -> Vec<Vec<bool>> {
    let mut cave = vec![vec![false; x.1 - x.0 + 1]; y.1 + 1];
    input.iter().for_each(|rock| {
        (0..rock.len() - 1).for_each(|i| {
            if rock[i].0 == rock[i + 1].0 {
                (min(rock[i].1, rock[i + 1].1)..=max(rock[i].1, rock[i + 1].1))
                    .for_each(|part_y| cave[part_y][rock[i].0 - x.0] = true);
            } else {
                (min(rock[i].0, rock[i + 1].0)..=max(rock[i].0, rock[i + 1].0))
                    .for_each(|part_x| cave[rock[i].1][part_x - x.0] = true);
            };
        })
    });
    cave
}

fn solve(mut cave: Vec<Vec<bool>>, sand: (usize, usize)) -> i32 {
    let mut units = 0;

    loop {
        if fall_sand(&mut cave, sand) {
            units += 1;
        } else {
            break;
        }
    }
    units
}

fn fall_sand(cave: &mut [Vec<bool>], sand: (usize, usize)) -> bool {
    let mut current = sand;
    loop {
        if current.1 + 1 >= cave.len() {
            return false;
        } else if !cave[current.1 + 1][current.0] {
            current.1 += 1;
        } else if current.0 == 0 {
            return false;
        } else if !cave[current.1 + 1][current.0 - 1] {
            current = (current.0 - 1, current.1 + 1);
        } else if current.0 == cave[0].len() {
            return false;
        } else if !cave[current.1 + 1][current.0 + 1] {
            current = (current.0 + 1, current.1 + 1);
            continue;
        } else {
            cave[current.1][current.0] = true;
            return current != sand;
        }
    }
}
