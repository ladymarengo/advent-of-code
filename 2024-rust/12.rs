use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/12.txt").unwrap();
    let grid: Vec<Vec<Plot>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Plot {
                    plant: c,
                    processed: false,
                })
                .collect()
        })
        .collect();

    println!("First answer is {}", count_price(&grid, false));
    println!("Second answer is {}", count_price(&grid, true));
}

#[derive(Copy, Clone, Debug)]
struct Plot {
    plant: char,
    processed: bool,
}

#[derive(Copy, Clone, Debug)]
struct Wall {
    start_row: i32,
    start_col: i32,
    end_row: i32,
    end_col: i32,
    orientation: Orientation,
    processed: bool,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Orientation {
    Top,
    Bottom,
    Right,
    Left,
}

fn count_price(grid: &Vec<Vec<Plot>>, part_two: bool) -> usize {
    let mut grid = grid.clone();
    let mut result: usize = 0;
    (0..grid.len()).for_each(|row| {
        (0..grid[0].len()).for_each(|col| {
            if !grid[row][col].processed {
                result += count_region(&mut grid, row, col, part_two);
            }
        })
    });
    result
}

fn count_region(grid: &mut Vec<Vec<Plot>>, row: usize, col: usize, part_two: bool) -> usize {
    let mut area: usize = 0;
    let mut perimeter: usize = 0;
    let mut walls: Vec<Wall> = Vec::new();

    let plant = grid[row][col].plant;
    let mut plots: Vec<(usize, usize)> = Vec::new();
    plots.push((row, col));
    while !plots.is_empty() {
        let plot = plots.pop().unwrap();
        let mut neighbors: Vec<(usize, usize)> = Vec::new();
        if plot.0 > 0 && grid[plot.0 - 1][plot.1].plant == plant {
            neighbors.push((plot.0 - 1, plot.1));
        } else if part_two {
            walls.push(Wall {
                start_row: plot.0 as i32,
                start_col: plot.1 as i32,
                end_row: plot.0 as i32,
                end_col: plot.1 as i32,
                orientation: Orientation::Top,
                processed: false,
            });
        }
        if plot.1 > 0 && grid[plot.0][plot.1 - 1].plant == plant {
            neighbors.push((plot.0, plot.1 - 1));
        } else if part_two {
            walls.push(Wall {
                start_row: plot.0 as i32,
                start_col: plot.1 as i32,
                end_row: plot.0 as i32,
                end_col: plot.1 as i32,
                orientation: Orientation::Left,
                processed: false,
            });
        }
        if plot.0 < grid.len() - 1 && grid[plot.0 + 1][plot.1].plant == plant {
            neighbors.push((plot.0 + 1, plot.1));
        } else if part_two {
            walls.push(Wall {
                start_row: plot.0 as i32,
                start_col: plot.1 as i32,
                end_row: plot.0 as i32,
                end_col: plot.1 as i32,
                orientation: Orientation::Bottom,
                processed: false,
            });
        }
        if plot.1 < grid[0].len() - 1 && grid[plot.0][plot.1 + 1].plant == plant {
            neighbors.push((plot.0, plot.1 + 1));
        } else if part_two {
            walls.push(Wall {
                start_row: plot.0 as i32,
                start_col: plot.1 as i32,
                end_row: plot.0 as i32,
                end_col: plot.1 as i32,
                orientation: Orientation::Right,
                processed: false,
            });
        }
        area += 1;
        if !part_two {
            perimeter += 4 - neighbors.len();
        }
        grid[plot.0][plot.1].processed = true;
        neighbors.iter().for_each(|neighbor| {
            if !plots.contains(&neighbor) && !grid[neighbor.0][neighbor.1].processed {
                plots.push(*neighbor)
            }
        });
    }

    if part_two {
        let mut final_walls: Vec<Wall> = Vec::new();
        (0..walls.len()).for_each(|i| {
            if !walls[i].processed {
                walls[i].processed = true;
                let mut wall = walls[i];
                while let Some(next) = walls.iter_mut().find(|next| {
                    !next.processed
                        && next.orientation == wall.orientation
                        && ((next.start_col == wall.start_col
                            && (next.start_row == wall.start_row - 1
                                || next.end_row == wall.end_row + 1))
                            || (next.start_row == wall.start_row
                                && (next.start_col == wall.start_col - 1
                                    || next.end_col == wall.end_col + 1)))
                }) {
                    if next.start_row == wall.start_row - 1 {
                        wall.start_row = next.start_row;
                    } else if next.end_row == wall.end_row + 1 {
                        wall.end_row = next.end_row;
                    } else if next.start_col == wall.start_col - 1 {
                        wall.start_col = next.start_col;
                    } else if next.end_col == wall.end_col + 1 {
                        wall.end_col = next.end_col;
                    }
                    next.processed = true;
                }
                final_walls.push(wall);
            }
        });
        perimeter = final_walls.len();
    }

    area * perimeter
}
