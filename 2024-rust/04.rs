use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/04.txt").unwrap();
    let word_search: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let result: usize = word_search
        .iter()
        .enumerate()
        .map(|(row, line)| {
            line.iter()
                .enumerate()
                .map(|(col, c)| {
                    if *c == 'X' {
                        count_xmas(&word_search, row, col)
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum();
    println!("First answer is {result}");

    let result: usize = word_search
        .iter()
        .enumerate()
        .map(|(row, line)| {
            line.iter()
                .enumerate()
                .filter(|(col, c)| **c == 'A' && is_xmas(&word_search, row, *col))
                .count()
        })
        .sum();
    println!("Second answer is {result}");
}

fn count_xmas(ws: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    let grid_height = ws.len();
    let grid_width = ws[0].len();
    let mut coords: Vec<Vec<(i32, i32)>> = Vec::new();
    if row >= 3 {
        coords.push(vec![(-1, 0), (-2, 0), (-3, 0)]);
        if col >= 3 {
            coords.push(vec![(-1, -1), (-2, -2), (-3, -3)]);
        }
        if col < grid_width - 3 {
            coords.push(vec![(-1, 1), (-2, 2), (-3, 3)]);
        }
    }
    if row < grid_height - 3 {
        coords.push(vec![(1, 0), (2, 0), (3, 0)]);
        if col >= 3 {
            coords.push(vec![(1, -1), (2, -2), (3, -3)]);
        }
        if col < grid_width - 3 {
            coords.push(vec![(1, 1), (2, 2), (3, 3)]);
        }
    }
    if col >= 3 {
        coords.push(vec![(0, -1), (0, -2), (0, -3)]);
    }
    if col < grid_width - 3 {
        coords.push(vec![(0, 1), (0, 2), (0, 3)]);
    }
    coords
        .iter()
        .filter(|&word| {
            ws[(row as i32 + word[0].0) as usize][(col as i32 + word[0].1) as usize] == 'M'
                && ws[(row as i32 + word[1].0) as usize][(col as i32 + word[1].1) as usize] == 'A'
                && ws[(row as i32 + word[2].0) as usize][(col as i32 + word[2].1) as usize] == 'S'
        })
        .count()
}

fn is_xmas(ws: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    row > 0
        && col > 0
        && row < ws.len() - 1
        && col < ws[0].len() - 1
        && ((ws[row - 1][col - 1] == 'M' && ws[row + 1][col + 1] == 'S')
            || (ws[row - 1][col - 1] == 'S' && ws[row + 1][col + 1] == 'M'))
        && ((ws[row - 1][col + 1] == 'M' && ws[row + 1][col - 1] == 'S')
            || (ws[row - 1][col + 1] == 'S' && ws[row + 1][col - 1] == 'M'))
}
