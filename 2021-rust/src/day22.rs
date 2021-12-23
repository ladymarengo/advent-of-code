use regex::Regex;
use std::cmp::{max, min};
use std::fs::read_to_string;

#[derive(Debug)]
struct Zone {
    on: bool,
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    z_min: i32,
    z_max: i32,
}

#[derive(Debug, Clone)]
struct X {
    value: Vec<((i32, i32), bool, Y)>,
}

#[derive(Debug, Clone)]
struct Y {
    value: Vec<((i32, i32), bool, Z)>,
}

#[derive(Debug, Clone)]
struct Z {
    value: Vec<((i32, i32), bool)>,
}

fn main() {
    let instructions = parse_input();
    println!("First answer is {}", part_one(&instructions));
    println!("Second answer is {}", part_two(&instructions));
}

fn parse_input() -> Vec<Zone> {
    let re =
        Regex::new(r"(on|off) x=(-*\d+)..(-*\d+),y=(-*\d+)..(-*\d+),z=(-*\d+)..(-*\d+)").unwrap();
    let input: Vec<Zone> = read_to_string("input/22")
        .unwrap()
        .lines()
        .map(|d| {
            let caps = re.captures(d).unwrap();
            Zone {
                on: caps[1].to_string() == "on",
                x_min: caps[2].parse::<i32>().unwrap(),
                x_max: caps[3].parse::<i32>().unwrap(),
                y_min: caps[4].parse::<i32>().unwrap(),
                y_max: caps[5].parse::<i32>().unwrap(),
                z_min: caps[6].parse::<i32>().unwrap(),
                z_max: caps[7].parse::<i32>().unwrap(),
            }
        })
        .collect();
    input
}

fn part_one(instructions: &[Zone]) -> usize {
    let z: Vec<bool> = vec![false; 101];
    let y: Vec<Vec<bool>> = vec![z; 101];
    let mut grid: Vec<Vec<Vec<bool>>> = vec![y; 101];
    
    for instr in instructions {
        if instr.x_min >= -50
            && instr.x_max <= 50
            && instr.y_min >= -50
            && instr.y_max <= 50
            && instr.z_min >= -50
            && instr.z_max <= 50
        {
            for x in instr.x_min..=instr.x_max {
                for y in instr.y_min..=instr.y_max {
                    for z in instr.z_min..=instr.z_max {
                        grid[(x + 50) as usize][(y + 50) as usize][(z + 50) as usize] = instr.on;
                    }
                }
            }
        }
    }
    let mut lighted: usize = 0;
    
    for row in &grid {
        for column in row {
            for depth in column {
                if *depth {
                    lighted += 1;
                }
            }
        }
    }
    lighted
}

fn part_two(instructions: &[Zone]) -> usize {
    let mut grid: X = X { value: vec![] };
    
    for instruction in instructions {
        slice_x(
            &mut grid,
            instruction.x_min,
            instruction.x_max,
            instruction.on,
        );
        
        for e in 0..grid.value.len() {
            if (grid.value[e].0 .0 >= instruction.x_min && grid.value[e].0 .0 <= instruction.x_max)
                || (grid.value[e].0 .1 >= instruction.x_min
                    && grid.value[e].0 .1 <= instruction.x_max)
            {
                grid.value[e].2 = slice_y(
                    &grid.value[e].2,
                    instruction.y_min,
                    instruction.y_max,
                    instruction.on,
                );
                
                for i in 0..grid.value[e].2.value.len() {
                    if (grid.value[e].2.value[i].0 .0 >= instruction.y_min
                        && grid.value[e].2.value[i].0 .0 <= instruction.y_max)
                        || (grid.value[e].2.value[i].0 .1 >= instruction.y_min
                            && grid.value[e].2.value[i].0 .1 <= instruction.y_max)
                    {
                        grid.value[e].2.value[i].2 = slice_z(
                            grid.value[e].2.value[i].2.clone(),
                            instruction.z_min,
                            instruction.z_max,
                            instruction.on,
                        );
                    }
                }
            }
        }
    }
    calculate_lighted(&grid)
}

fn slice_x(grid: &mut X, mut x_min: i32, x_max: i32, on: bool) {
    let mut new: X = X { value: vec![] };
    let mut done = false;
    
    if grid.value.is_empty() || grid.value.last().unwrap().0 .1 < x_min {
        grid.value.push(((x_min, x_max), on, Y { value: vec![] }));
        return;
    } else if grid.value[0].0 .0 > x_max {
        grid.value
            .insert(0, ((x_min, x_max), on, Y { value: vec![] }));
        return;
    }
    
    for i in 0..grid.value.len() {
        let old_min: i32 = grid.value[i].0 .0;
        let old_max: i32 = grid.value[i].0 .1;
        if old_max < x_min {
            new.value.push(grid.value[i].clone());
        } else if old_min > x_max {
            if !done {
                new.value.push(((x_min, x_max), on, Y { value: vec![] }));
                done = true;
            }
            new.value.push(grid.value[i].clone());
        } else if old_min == x_min && old_max == x_max {
            new.value.push(((grid.value[i].0.0, grid.value[i].0.1), on, grid.value[i].2.clone()));
            done = true;
        } else {
            
            if old_min < x_min {
                new.value.push((
                    (old_min, x_min - 1),
                    grid.value[i].1,
                    grid.value[i].2.clone(),
                ));
            } else if old_min > x_min {
                new.value
                    .push(((x_min, old_min - 1), on, Y { value: vec![] }));
            }
            
            new.value.push((
                (max(old_min, x_min), min(old_max, x_max)),
                on,
                grid.value[i].2.clone(),
            ));
            
            if old_max > x_max {
                new.value.push((
                    (x_max + 1, old_max),
                    grid.value[i].1,
                    grid.value[i].2.clone(),
                ));
            }
            if old_max < x_max {
                x_min = old_max + 1;
            } else {
                done = true;
            }
        }
    }
    if !done {
        new.value.push(((x_min, x_max), on, Y { value: vec![] }));
    }
    *grid = new;
}

fn slice_y(grid: &Y, mut y_min: i32, y_max: i32, on: bool) -> Y {
    let mut new: Y = Y { value: vec![] };
    let mut done = false;
    
    if grid.value.is_empty() || grid.value.last().unwrap().0 .1 < y_min {
        new = grid.clone();
        new.value.push(((y_min, y_max), on, Z { value: vec![] }));
        return new;
    } else if grid.value[0].0 .0 > y_max {
        new = grid.clone();
        new.value
            .insert(0, ((y_min, y_max), on, Z { value: vec![] }));
        return new;
    }
    
    for i in 0..grid.value.len() {
        let old_min: i32 = grid.value[i].0 .0;
        let old_max: i32 = grid.value[i].0 .1;
        
        if old_max < y_min {
            new.value.push(grid.value[i].clone());
        } else if old_min > y_max {
            if !done {
                new.value.push(((y_min, y_max), on, Z { value: vec![] }));
                done = true;
            }
            new.value.push(grid.value[i].clone());
        } else if old_min == y_min && old_max == y_max {
            new.value.push(((grid.value[i].0.0, grid.value[i].0.1), on, grid.value[i].2.clone()));
            done = true;
        } else {
            
            if old_min < y_min {
                new.value.push((
                    (old_min, y_min - 1),
                    grid.value[i].1,
                    grid.value[i].2.clone(),
                ));
            } else if old_min > y_min {
                new.value
                    .push(((y_min, old_min - 1), on, Z { value: vec![] }));
            }
            
            new.value.push((
                (max(old_min, y_min), min(old_max, y_max)),
                on,
                grid.value[i].2.clone(),
            ));
            
            if old_max > y_max {
                new.value.push((
                    (y_max + 1, old_max),
                    grid.value[i].1,
                    grid.value[i].2.clone(),
                ));
            }
            if old_max < y_max {
                y_min = old_max + 1;
            } else {
                done = true;
            }
        }
    }
    if !done {
        new.value.push(((y_min, y_max), on, Z { value: vec![] }));
    }
    new
}

fn slice_z(mut grid: Z, mut z_min: i32, z_max: i32, on: bool) -> Z {
    let mut new: Z = Z { value: vec![] };
    let mut done = false;
    
    if grid.value.is_empty() || grid.value.last().unwrap().0 .1 < z_min {
        grid.value.push(((z_min, z_max), on));
        return grid;
    } else if grid.value[0].0 .0 > z_max {
        grid.value.insert(0, ((z_min, z_max), on));
        return grid;
    }
    
    for i in 0..grid.value.len() {
        let old_min: i32 = grid.value[i].0 .0;
        let old_max: i32 = grid.value[i].0 .1;
        
        if old_max < z_min {
            new.value.push(grid.value[i]);
        } else if old_min > z_max {
            if !done {
                new.value.push(((z_min, z_max), on));
                done = true;
            }
            new.value.push(grid.value[i]);
        } else if old_min == z_min && old_max == z_max {
            new.value.push(((grid.value[i].0.0, grid.value[i].0.1), on));
            done = true;
        } else {
            
            if old_min < z_min {
                new.value.push(((old_min, z_min - 1), grid.value[i].1));
            } else if old_min > z_min {
                new.value.push(((z_min, old_min - 1), on));
            }
            
            new.value
                .push(((max(old_min, z_min), min(old_max, z_max)), on));
            if old_max > z_max {
                new.value.push(((z_max + 1, old_max), grid.value[i].1));
            }
            if old_max < z_max {
                z_min = old_max + 1;
            } else {
                done = true;
            }
        }
    }
    if !done {
        new.value.push(((z_min, z_max), on));
    }
    new
}

fn calculate_lighted(grid: &X) -> usize {
    let mut lighted: usize = 0;
    for row in &grid.value {
        for column in &row.2.value {
            for slice in &column.2.value {
                if slice.1 {
                    lighted += (row.0.1 - row.0.0 + 1) as usize * (column.0.1 - column.0.0 + 1) as usize * (slice.0.1 - slice.0.0 + 1) as usize;
                }
            }
        }
    }
    lighted
}