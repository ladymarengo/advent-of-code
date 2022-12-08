use std::fs::read_to_string;

fn main() {
    let trees = read_to_string("input/08.txt")
        .unwrap()
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect::<Vec<Vec<i32>>>();

    let mut visible = vec![vec![false; trees[0].len()]; trees.len()];
    trees.iter().enumerate().for_each(|(i, line)| {
        let mut highest = -1;
        line.iter().enumerate().for_each(|(e, tree)| {
            if *tree > highest {
                highest = *tree;
                visible[i][e] = true;
            }
        });
        highest = -1;
        line.iter().enumerate().rev().for_each(|(e, tree)| {
            if *tree > highest {
                highest = *tree;
                visible[i][e] = true;
            }
        });
    });
    (0..trees[0].len()).for_each(|i| {
        let mut highest = -1;
        (0..trees.len()).for_each(|e| {
            if trees[e][i] > highest {
                highest = trees[e][i];
                visible[e][i] = true;
            }
        });
        highest = -1;
        (0..trees.len()).rev().for_each(|e| {
            if trees[e][i] > highest {
                highest = trees[e][i];
                visible[e][i] = true;
            }
        });
    });
    let sum = visible
        .iter()
        .map(|line| line.iter().filter(|t| **t).count())
        .sum::<usize>();
    println!("{sum}");

    let mut scenic_scores: Vec<Vec<usize>> = trees
        .iter()
        .enumerate()
        .map(|(i, l)| {
            l.iter()
                .enumerate()
                .map(|(e, _t)| calculate_scenic_score(&trees, i, e))
                .collect()
        })
        .collect();

    let mut best = scenic_scores
        .iter_mut()
        .map(|s| {
            s.sort();
            *s.last().unwrap()
        })
        .collect::<Vec<usize>>();
    best.sort();
    println!("{}", best.last().unwrap());
}

fn calculate_scenic_score(trees: &Vec<Vec<i32>>, row: usize, column: usize) -> usize {
    let mut scenic_score = 1;
    let tree = trees[row][column];
    let mut direction = 0;
    (row + 1..trees.len()).try_for_each(|r| {
        if trees[r][column] < tree {
            direction += 1;
            Some(())
        } else {
            direction += 1;
            None
        }
    });
    scenic_score *= direction;
    direction = 0;
    (0..row).rev().try_for_each(|r| {
        if trees[r][column] < tree {
            direction += 1;
            Some(())
        } else {
            direction += 1;
            None
        }
    });
    scenic_score *= direction;
    direction = 0;
    (column + 1..trees[0].len()).try_for_each(|c| {
        if trees[row][c] < tree {
            direction += 1;
            Some(())
        } else {
            direction += 1;
            None
        }
    });
    scenic_score *= direction;
    direction = 0;
    (0..column).rev().try_for_each(|c| {
        if trees[row][c] < tree {
            direction += 1;
            Some(())
        } else {
            direction += 1;
            None
        }
    });
    scenic_score *= direction;
    scenic_score
}
