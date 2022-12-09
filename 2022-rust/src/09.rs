use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/09.txt").unwrap();
    part_one(&input);
    part_two(&input);
}

fn part_one(input: &str) {
    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);
    let mut positions: HashSet<(i32, i32)> = HashSet::new();

    input.lines().for_each(|l| {
        let (direction, steps) = l.split_once(' ').unwrap();
        (0..steps.parse().unwrap()).for_each(|_s| {
            move_head(&mut head, direction);
            move_tail(&head, &mut tail);
            positions.insert(tail);
        });
    });
    println!("{}", positions.len());
}

fn part_two(input: &str) {
    let mut head: (i32, i32) = (0, 0);
    let mut tails: Vec<(i32, i32)> = vec![(0, 0); 9];
    let mut positions: HashSet<(i32, i32)> = HashSet::new();

    input.lines().for_each(|l| {
        let (direction, steps) = l.split_once(' ').unwrap();
        (0..steps.parse().unwrap()).for_each(|_s| {
            move_head(&mut head, direction);
            move_tail(&head, &mut tails[0]);
            (0..8).for_each(|t| move_tail(&tails[t].clone(), &mut tails[t + 1]));
            positions.insert(tails[8]);
        });
    });
    println!("{}", positions.len());
}

fn move_head(head: &mut (i32, i32), direction: &str) {
    match direction {
        "R" => head.0 += 1,
        "U" => head.1 += 1,
        "L" => head.0 -= 1,
        "D" => head.1 -= 1,
        _ => (),
    }
}

fn move_tail(head: &(i32, i32), tail: &mut (i32, i32)) {
    match (head.0 - tail.0, head.1 - tail.1) {
        (2, 0) => tail.0 += 1,
        (0, 2) => tail.1 += 1,
        (-2, 0) => tail.0 -= 1,
        (0, -2) => tail.1 -= 1,
        (1 | -1, 1 | -1) => (),
        (1..=2, 1..=2) => {
            tail.0 += 1;
            tail.1 += 1
        }
        (-2..=-1, 1..=2) => {
            tail.0 -= 1;
            tail.1 += 1
        }
        (1..=2, -2..=-1) => {
            tail.0 += 1;
            tail.1 -= 1
        }
        (-2..=-1, -2..=-1) => {
            tail.0 -= 1;
            tail.1 -= 1
        }
        _ => (),
    }
}
