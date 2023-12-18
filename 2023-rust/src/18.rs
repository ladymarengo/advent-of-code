use std::{
    cmp::{max, min},
    collections::HashSet,
    fs::read_to_string,
};

fn main() {
    let input = read_to_string("input/18.txt").unwrap();

    let mut trench_points: Vec<(i32, i32)> = vec![];
    let mut trench_path: HashSet<(i32, i32)> = HashSet::new();
    let mut row = 0;
    let mut column = 0;
    input.lines().for_each(|line| {
        let info: Vec<&str> = line.split(' ').collect();
        let number = info[1].parse::<i32>().unwrap();
        let current = (row, column);
        match info[0] {
            "U" => row -= number,
            "R" => column += number,
            "D" => row += number,
            "L" => column -= number,
            _ => (),
        };
        trench_points.push((row, column));
        for row in min(current.0, row)..=max(current.0, row) {
            for column in min(current.1, column)..=max(current.1, column) {
                trench_path.insert((row, column));
            }
        }
    });

    let row_boundaries = (
        trench_points.iter().map(|p| p.0).min().unwrap(),
        trench_points.iter().map(|p| p.0).max().unwrap(),
    );
    let column_boundaries = (
        trench_points.iter().map(|p| p.1).min().unwrap(),
        trench_points.iter().map(|p| p.1).max().unwrap(),
    );

    let map_size = (
        max(row_boundaries.0, row_boundaries.1) - min(row_boundaries.0, row_boundaries.1) + 1,
        max(column_boundaries.0, column_boundaries.1)
            - min(column_boundaries.0, column_boundaries.1)
            + 1,
    );

    // for row in (row_boundaries.0..=row_boundaries.1) {
    // 	for column in (column_boundaries.0..=column_boundaries.1) {
    // 		if trench_path.contains(&(row, column)) {
    // 			print!("#");
    // 		} else {
    // 			print!(".");
    // 		}
    // 	}
    // 	println!();
    // }

    let amount = (map_size.0 + 2) * (map_size.1 + 2)
        - count_neighbors(&trench_path, row_boundaries, column_boundaries) as i32;
    println!("First answer is {amount}");
}

fn count_neighbors(
    path: &HashSet<(i32, i32)>,
    row_boundaries: (i32, i32),
    column_boundaries: (i32, i32),
) -> usize {
    let mut checked: Vec<(i32, i32)> = Vec::new();
    let mut amount = 0;
    let mut open = vec![(row_boundaries.0 - 1, column_boundaries.0 - 1)];
    while let Some(current) = open.pop() {
        (-1..=1).for_each(|row| {
            (-1..=1).for_each(|column| {
                if current.0 + row >= row_boundaries.0 - 1
                    && current.1 + column >= column_boundaries.0 - 1
                    && current.0 + row <= row_boundaries.1 + 1
                    && current.1 + column <= column_boundaries.1 + 1
                {
                    let new = (current.0 + row, current.1 + column);
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
