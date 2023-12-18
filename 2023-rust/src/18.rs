use std::{
    cmp::{max, min},
    fs::read_to_string,
};

#[derive(Debug, PartialEq)]
struct Rectangle {
    top: i128,
    right: i128,
    down: i128,
    left: i128,
}

#[derive(Debug, PartialEq)]
struct Line {
    row: (i128, i128),
    column: (i128, i128),
    direction: Direction,
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn main() {
    let input = read_to_string("input/18.txt").unwrap();

    let mut trench_points: Vec<(i128, i128)> = vec![];
    let mut row = 0;
    let mut column = 0;

    let mut row_points = Vec::new();
    let mut column_points = Vec::new();
    let mut lines = Vec::new();

    input.lines().for_each(|line| {
        let info: Vec<&str> = line.split(' ').collect();
        let number = i128::from_str_radix(&info[2][2..7], 16).unwrap();
        let current = (row, column);
        let direction = match &info[2][7..8] {
            "3" => Direction::Up,
            "0" => Direction::Right,
            "1" => Direction::Down,
            _ => Direction::Left,
        };
        match &info[2][7..8] {
            "3" => row -= number,
            "0" => column += number,
            "1" => row += number,
            "2" => column -= number,
            _ => (),
        };
        trench_points.push((row, column));

        if !row_points.contains(&row) {
            row_points.push(row);
        }
        if !column_points.contains(&column) {
            column_points.push(column);
        }
        lines.push(Line {
            row: (min(current.0, row), max(current.0, row)),
            column: (min(current.1, column), max(current.1, column)),
            direction,
        });
    });

    row_points.sort();
    column_points.sort();

    let mut rectangles: Vec<Rectangle> = Vec::new();
    lines.iter().for_each(|line| {
        let step: i128 = match line.direction {
            Direction::Down | Direction::Left => -1,
            Direction::Up | Direction::Right => 1,
        };
        match line.direction {
            Direction::Right | Direction::Left => {
                let mut position = row_points
                    .iter()
                    .position(|row| *row == line.row.0)
                    .unwrap() as i128;
                loop {
                    position += step;

                    if position < 0 || position >= row_points.len() as i128 {
                        position -= step;
                        break;
                    } else if lines.iter().any(|other| {
                        other != line
                            && other.row.0 == row_points[position as usize]
                            && !(other.column.0 >= line.column.1 || other.column.1 <= line.column.0)
                    }) {
                        break;
                    }
                }
                let rectangle = Rectangle {
                    top: min(line.row.0, row_points[(position) as usize]),
                    right: max(line.column.0, line.column.1),
                    down: max(line.row.0, row_points[(position) as usize]),
                    left: min(line.column.0, line.column.1),
                };
                if !rectangles.contains(&rectangle) {
                    rectangles.push(rectangle);
                }
            }
            Direction::Up | Direction::Down => {
                let mut position = column_points
                    .iter()
                    .position(|column| *column == line.column.0)
                    .unwrap() as i128;
                loop {
                    position += step;
                    if position < 0 || position >= column_points.len() as i128 {
                        position -= step;
                        break;
                    } else if lines.iter().any(|other| {
                        other != line
                            && other.column.0 == column_points[position as usize]
                            && !(other.row.0 >= line.row.1 || other.row.1 <= line.row.0)
                    }) {
                        break;
                    }
                }
                let rectangle = Rectangle {
                    top: min(line.row.0, line.row.1),
                    right: max(line.column.0, column_points[(position) as usize]),
                    down: max(line.row.0, line.row.1),
                    left: min(line.column.0, column_points[(position) as usize]),
                };
                if !rectangles.contains(&rectangle) {
                    rectangles.push(rectangle);
                }
            }
        }
    });

    let amount = (0..row_points.len())
        .map(|row_pos| {
            (0..column_points.len())
                .map(|col_pos| {
                    let mut new_amount = 0;
                    new_amount += count_intersection(
                        &rectangles,
                        row_points[row_pos],
                        row_points[row_pos],
                        column_points[col_pos],
                        column_points[col_pos],
                    );
                    if row_pos < row_points.len() - 1
                        && col_pos < column_points.len() - 1
                        && row_points[row_pos + 1] - row_points[row_pos] > 1
                        && column_points[col_pos + 1] - column_points[col_pos] > 1
                    {
                        new_amount += count_intersection(
                            &rectangles,
                            row_points[row_pos] + 1,
                            row_points[row_pos + 1] - 1,
                            column_points[col_pos] + 1,
                            column_points[col_pos + 1] - 1,
                        );
                        new_amount += count_intersection(
                            &rectangles,
                            row_points[row_pos] + 1,
                            row_points[row_pos + 1] - 1,
                            column_points[col_pos],
                            column_points[col_pos],
                        );
                        new_amount += count_intersection(
                            &rectangles,
                            row_points[row_pos],
                            row_points[row_pos],
                            column_points[col_pos] + 1,
                            column_points[col_pos + 1] - 1,
                        );
                    } else if row_pos < row_points.len() - 1
                        && row_points[row_pos + 1] - row_points[row_pos] > 1
                    {
                        new_amount += count_intersection(
                            &rectangles,
                            row_points[row_pos] + 1,
                            row_points[row_pos + 1] - 1,
                            column_points[col_pos],
                            column_points[col_pos],
                        );
                    } else if col_pos < column_points.len() - 1
                        && column_points[col_pos + 1] - column_points[col_pos] > 1
                    {
                        new_amount += count_intersection(
                            &rectangles,
                            row_points[row_pos],
                            row_points[row_pos],
                            column_points[col_pos] + 1,
                            column_points[col_pos + 1] - 1,
                        );
                    }

                    new_amount
                })
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("{amount}");
}

fn count_intersection(
    rectangles: &[Rectangle],
    top: i128,
    down: i128,
    left: i128,
    right: i128,
) -> usize {
    if rectangles.iter().any(|rectangle| {
        top >= rectangle.top
            && down <= rectangle.down
            && left >= rectangle.left
            && right <= rectangle.right
    }) {
        return ((down - top + 1) * (right - left + 1)) as usize;
    }
    0
}
