use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/03.txt").unwrap();

    let schematic: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut part_sum = 0;
    schematic.iter().enumerate().for_each(|(i_line, line)| {
        let mut part_number = 0;
        let mut adjacent = false;
        line.iter().enumerate().for_each(|(i_symbol, symbol)| {
            if symbol.is_ascii_digit() {
                part_number = part_number * 10 + symbol.to_digit(10).unwrap();
                check_adjacent(&mut adjacent, &schematic, i_line as i32, i_symbol as i32);
            } else {
                if part_number != 0 && adjacent {
                    part_sum += part_number;
                }
                part_number = 0;
                adjacent = false;
            }
        });
        if part_number != 0 && adjacent {
            part_sum += part_number;
        }
    });
    println!("First answer is {part_sum}");
}

fn check_adjacent(
    adjacent: &mut bool,
    schematic: &[Vec<char>],
    line_index: i32,
    symbol_index: i32,
) {
    if *adjacent {
        return;
    }
    (line_index - 1..=line_index + 1).for_each(|y| {
        (symbol_index - 1..=symbol_index + 1).for_each(|x| {
            if x >= 0
                && y >= 0
                && y < schematic.len() as i32
                && x < schematic[0].len() as i32
                && schematic[y as usize][x as usize] != '.'
                && !schematic[y as usize][x as usize].is_ascii_digit()
            {
                *adjacent = true;
            }
        })
    })
}
