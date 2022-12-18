use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/18.txt").unwrap();
    let mut cubes: HashSet<(i32, i32, i32)> = HashSet::new();
    input.lines().for_each(|l| {
        let coords: Vec<i32> = l.split(',').map(|c| c.parse::<i32>().unwrap()).collect();
        cubes.insert((coords[0], coords[1], coords[2]));
    });
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
}
