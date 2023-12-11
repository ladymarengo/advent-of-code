use std::{
    cmp::{max, min},
    fs::read_to_string,
};

fn main() {
    let input = read_to_string("input/11.txt").unwrap();

    let mut galaxies = Vec::new();
    input.lines().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(column, c)| {
            if c == '#' {
                galaxies.push((row, column))
            }
        })
    });

    let (rows, columns): (Vec<usize>, Vec<usize>) = galaxies.iter().cloned().unzip();
    let max_row = *rows.iter().max().unwrap();
    let max_column = *columns.iter().max().unwrap();
    let empty_rows: Vec<usize> = (0..max_row).filter(|row| !rows.contains(row)).collect();
    let empty_columns: Vec<usize> = (0..max_column)
        .filter(|column| !columns.contains(column))
        .collect();

    println!(
        "First answer is {}",
        count_lengths(&galaxies, &empty_rows, &empty_columns, 2)
    );
    println!(
        "Second answer is {}",
        count_lengths(&galaxies, &empty_rows, &empty_columns, 1000000)
    );
}

fn count_lengths(
    galaxies: &[(usize, usize)],
    empty_rows: &[usize],
    empty_columns: &[usize],
    expansion: usize,
) -> usize {
    let mut lengths_sum = 0;
    galaxies.iter().enumerate().for_each(|(i, first)| {
        galaxies[i + 1..].iter().for_each(|second| {
            let min = (min(first.0, second.0), min(first.1, second.1));
            let max = (max(first.0, second.0), max(first.1, second.1));
            let mut length = 0;
            (min.0..max.0).for_each(|row| {
                length += if empty_rows.contains(&row) {
                    expansion
                } else {
                    1
                }
            });
            (min.1..max.1).for_each(|column| {
                length += if empty_columns.contains(&column) {
                    expansion
                } else {
                    1
                }
            });
            lengths_sum += length;
        })
    });
    lengths_sum
}
