use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("input/10.txt").unwrap();
    let grid: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let mut result_part_one: usize = 0;
    let mut result_part_two: usize = 0;

    grid.iter().enumerate().for_each(|(row, line)| {
        line.iter().enumerate().for_each(|(col, &height)| {
            if height == 0 {
                result_part_one += count_score(&grid, row, col, false);
                result_part_two += count_score(&grid, row, col, true);
            }
        })
    });

    println!("First answer is {result_part_one}");
    println!("Second answer is {result_part_two}");
}

fn count_score(grid: &Vec<Vec<usize>>, row: usize, col: usize, part_two: bool) -> usize {
    let mut heights_coords: HashMap<(usize, usize), usize> = HashMap::new();
    heights_coords.insert((row, col), 1);
    (0..9).for_each(|height| {
        let mut new_coords: HashMap<(usize, usize), usize> = HashMap::new();
        heights_coords.iter().for_each(|(coord, amount)| {
            if coord.0 > 0 && grid[coord.0 - 1][coord.1] == height + 1 {
                let next = new_coords.entry((coord.0 - 1, coord.1)).or_default();
                *next += amount;
            }
            if coord.1 > 0 && grid[coord.0][coord.1 - 1] == height + 1 {
                let next = new_coords.entry((coord.0, coord.1 - 1)).or_default();
                *next += amount;
            }
            if coord.0 < grid.len() - 1 && grid[coord.0 + 1][coord.1] == height + 1 {
                let next = new_coords.entry((coord.0 + 1, coord.1)).or_default();
                *next += amount;
            }
            if coord.1 < grid[0].len() - 1 && grid[coord.0][coord.1 + 1] == height + 1 {
                let next = new_coords.entry((coord.0, coord.1 + 1)).or_default();
                *next += amount;
            }
        });
        heights_coords = new_coords;
    });
    if part_two {
        heights_coords.values().sum()
    } else {
        heights_coords.len()
    }
}
