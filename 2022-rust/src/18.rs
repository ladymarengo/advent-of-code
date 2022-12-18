use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/18.txt").unwrap();
    let mut cubes: HashSet<(i32, i32, i32)> = HashSet::new();
    input.lines().for_each(|l| {
        let coords: Vec<i32> = l.split(',').map(|c| c.parse::<i32>().unwrap()).collect();
        cubes.insert((coords[0], coords[1], coords[2]));
    });
    let x_range = (
        cubes.iter().map(|c| c.0).min().unwrap(),
        cubes.iter().map(|c| c.0).max().unwrap(),
    );
    let y_range = (
        cubes.iter().map(|c| c.1).min().unwrap(),
        cubes.iter().map(|c| c.1).max().unwrap(),
    );
    let z_range = (
        cubes.iter().map(|c| c.2).min().unwrap(),
        cubes.iter().map(|c| c.2).max().unwrap(),
    );
    let neighbours = vec![
        (1, 0, 0),
        (0, 1, 0),
        (0, 0, 1),
        (-1, 0, 0),
        (0, -1, 0),
        (0, 0, -1),
    ];
    let sides = cubes
        .iter()
        .map(|c| {
            neighbours
                .iter()
                .filter(|&&n| !cubes.contains(&(c.0 + n.0, c.1 + n.1, c.2 + n.2)))
                .count()
        })
        .sum::<usize>();
    dbg!(sides);

    let trapped = cubes
        .iter()
        .map(|c| {
            neighbours
                .iter()
                .filter(|&&n| {
                    !cubes.contains(&(c.0 + n.0, c.1 + n.1, c.2 + n.2))
                        && get_out(
                            &cubes,
                            x_range,
                            y_range,
                            z_range,
                            (c.0 + n.0, c.1 + n.1, c.2 + n.2),
                            &neighbours,
                        )
                })
                .count()
        })
        .sum::<usize>();

    dbg!(trapped);
}

fn get_out(
    cubes: &HashSet<(i32, i32, i32)>,
    x_range: (i32, i32),
    y_range: (i32, i32),
    z_range: (i32, i32),
    current: (i32, i32, i32),
    neighbours: &Vec<(i32, i32, i32)>,
) -> bool {
    let mut visited: Vec<(i32, i32, i32)> = Vec::new();
    let mut queue: Vec<(i32, i32, i32)> = vec![current];
    while !queue.is_empty() {
        let next = queue.pop().unwrap();
        if next.0 < x_range.0
            || next.0 > x_range.1
            || next.1 < y_range.0
            || next.1 > y_range.1
            || next.2 < z_range.0
            || next.2 > z_range.1
        {
            return true;
        }
        for n in neighbours {
            let neighbour = (next.0 + n.0, next.1 + n.1, next.2 + n.2);
            if !cubes.contains(&neighbour) && !visited.contains(&neighbour) {
                visited.push(neighbour);
                queue.push(neighbour);
            }
        }
    }
    false
}
