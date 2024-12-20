use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    usize::MAX,
};

fn main() {
    let input = read_to_string("input/20.txt").unwrap();
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut grid: Vec<Vec<Tile>> = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| {
                    if c == 'S' {
                        start = (row, col);
                    } else if c == 'E' {
                        end = (row, col);
                    }
                    if c != '#' {
                        Tile {
                            tile_type: TileType::Empty,
                            score: MAX,
                        }
                    } else {
                        Tile {
                            tile_type: TileType::Wall,
                            score: MAX,
                        }
                    }
                })
                .collect()
        })
        .collect();

    populate_grid(&mut grid, end);

    let mut cheats: HashMap<usize, HashSet<Cheat>> = HashMap::new();
    find_cheats(&grid, &mut cheats, 2);
    let result: usize = cheats
        .iter()
        .map(|(save, cheats)| if *save >= 100 { cheats.len() } else { 0 })
        .sum();
    println!("First answer is {result}");

    cheats.clear();
    find_cheats(&grid, &mut cheats, 20);
    let result: usize = cheats
        .iter()
        .map(|(save, cheats)| if *save >= 100 { cheats.len() } else { 0 })
        .sum();
    println!("Second answer is {result}");
}

#[derive(PartialEq, Debug)]
struct Tile {
    tile_type: TileType,
    score: usize,
}

#[derive(PartialEq, Debug)]
enum TileType {
    Wall,
    Empty,
}

#[derive(Eq, Hash, PartialEq, Debug)]
struct Cheat {
    start: (usize, usize),
    end: (usize, usize),
}

fn populate_grid(grid: &mut Vec<Vec<Tile>>, start_point: (usize, usize)) {
    let mut next: Vec<(usize, usize)> = Vec::from([start_point]);
    grid[start_point.0][start_point.1].score = 0;
    let mut distance = 0;
    while !next.is_empty() {
        distance += 1;
        let mut new: Vec<(usize, usize)> = Vec::new();
        next.iter().for_each(|(row, col)| {
            if *row > 0
                && grid[row - 1][*col].tile_type == TileType::Empty
                && grid[row - 1][*col].score == MAX
            {
                grid[row - 1][*col].score = distance;
                new.push((row - 1, *col));
            }
            if *row < grid.len() - 1
                && grid[row + 1][*col].tile_type == TileType::Empty
                && grid[row + 1][*col].score == MAX
            {
                grid[row + 1][*col].score = distance;
                new.push((row + 1, *col));
            }
            if *col > 0
                && grid[*row][col - 1].tile_type == TileType::Empty
                && grid[*row][col - 1].score == MAX
            {
                grid[*row][col - 1].score = distance;
                new.push((*row, col - 1));
            }
            if *col < grid[0].len() - 1
                && grid[*row][col + 1].tile_type == TileType::Empty
                && grid[*row][col + 1].score == MAX
            {
                grid[*row][col + 1].score = distance;
                new.push((*row, col + 1));
            }
        });
        next = new;
    }
}

fn find_cheats(
    grid: &Vec<Vec<Tile>>,
    cheats: &mut HashMap<usize, HashSet<Cheat>>,
    max_distance: i32,
) {
    grid.iter().enumerate().for_each(|(row, line)| {
        line.iter().enumerate().for_each(|(col, tile)| {
            if tile.tile_type == TileType::Empty {
                for y in -max_distance..=max_distance {
                    for x in -max_distance..=max_distance {
                        let point = (row as i32 + y, col as i32 + x);
                        let distance = (point.0 - row as i32).abs() + (point.1 - col as i32).abs();
                        if distance <= max_distance
                            && point.0 >= 0
                            && point.1 >= 0
                            && point.0 < grid.len() as i32
                            && point.1 < grid[0].len() as i32
                            && grid[point.0 as usize][point.1 as usize].tile_type == TileType::Empty
                        {
                            let save = grid[row][col].score as i32
                                - grid[point.0 as usize][point.1 as usize].score as i32;
                            if save > distance {
                                let cheat = Cheat {
                                    start: (row, col),
                                    end: (point.0 as usize, point.1 as usize),
                                };
                                let entry =
                                    cheats.entry(save as usize - distance as usize).or_default();
                                entry.insert(cheat);
                            }
                        }
                    }
                }
            }
        })
    });
}
