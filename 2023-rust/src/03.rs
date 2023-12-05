use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    let input = read_to_string("input/03.txt").unwrap();

    let schematic: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut part_sum = 0;
    let mut all_gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    schematic.iter().enumerate().for_each(|(i_line, line)| {
        let mut part_number = 0;
        let mut adjacent = false;
        let mut gears: HashSet<(usize, usize)> = HashSet::new();

        line.iter().enumerate().for_each(|(i_symbol, symbol)| {
            if symbol.is_ascii_digit() {
                part_number = part_number * 10 + symbol.to_digit(10).unwrap();
                check_adjacent(
                    &mut adjacent,
                    &schematic,
                    i_line as i32,
                    i_symbol as i32,
                    &mut gears,
                );
            } else {
                if part_number != 0 {
                    if adjacent {
                        part_sum += part_number;
                    }
                    gears.iter().for_each(|gear| {
                        if all_gears.get(gear).is_none() {
                            all_gears.insert(*gear, Vec::new());
                        }
                        all_gears.get_mut(gear).unwrap().push(part_number);
                    });
                }
                part_number = 0;
                adjacent = false;
                gears = HashSet::new();
            }
        });
        if part_number != 0 {
            if adjacent {
                part_sum += part_number;
            }
            gears.iter().for_each(|gear| {
                if all_gears.get(gear).is_none() {
                    all_gears.insert(*gear, Vec::new());
                }
                all_gears.get_mut(gear).unwrap().push(part_number);
            });
        }
    });

    let mut gear_ratio = 0;
    all_gears.iter().for_each(|gear| {
        let numbers = gear.1;
        if numbers.len() == 2 {
            gear_ratio += numbers[0] * numbers[1];
        }
    });
    println!("First answer is {part_sum}, second answer is {gear_ratio}");
}

fn check_adjacent(
    adjacent: &mut bool,
    schematic: &[Vec<char>],
    line_index: i32,
    symbol_index: i32,
    gears: &mut HashSet<(usize, usize)>,
) {
    (line_index - 1..=line_index + 1).for_each(|y| {
        (symbol_index - 1..=symbol_index + 1).for_each(|x| {
            if x >= 0 && y >= 0 && y < schematic.len() as i32 && x < schematic[0].len() as i32 {
                if schematic[y as usize][x as usize] != '.'
                    && !schematic[y as usize][x as usize].is_ascii_digit()
                {
                    *adjacent = true;
                }
                if schematic[y as usize][x as usize] == '*' {
                    gears.insert((y as usize, x as usize));
                }
            }
        })
    })
}
