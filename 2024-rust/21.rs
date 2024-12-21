use itertools::Itertools;
use std::cmp::min;
use std::collections::HashMap;
use std::usize::MAX;

fn main() {
    let input = vec!["540A", "839A", "682A", "826A", "974A"];
    let num_keypad_input = "789\n456\n123\nX0A";
    let dir_keypad_input = "X^A\n<v>";

    let num_keypad = parse_keypad(num_keypad_input, (3, 0));
    let dir_keypad = parse_keypad(dir_keypad_input, (0, 0));

    let mut moves_map: HashMap<((char, char), usize), usize> = HashMap::new();
    let result: usize = input
        .iter()
        .map(|code| {
            let input_line: Vec<char> = code.chars().collect();
            let code_result =
                make_code(&input_line, &num_keypad, &dir_keypad, 0, &mut moves_map, 2);
            code_result * code[..3].parse::<usize>().unwrap()
        })
        .sum();
    println!("First answer is {result}");

    moves_map.clear();
    let result: usize = input
        .iter()
        .map(|code| {
            let input_line: Vec<char> = code.chars().collect();
            let code_result =
                make_code(&input_line, &num_keypad, &dir_keypad, 0, &mut moves_map, 25);
            code_result * code[..3].parse::<usize>().unwrap()
        })
        .sum();
    println!("Second answer is {result}");
}

fn make_code(
    input_line: &Vec<char>,
    num_keypad: &HashMap<(char, char), Vec<Vec<char>>>,
    dir_keypad: &HashMap<(char, char), Vec<Vec<char>>>,
    depth: usize,
    moves_map: &mut HashMap<((char, char), usize), usize>,
    keypads_amount: usize,
) -> usize {
    let mut length = 0;
    let keypad = if depth == 0 { &num_keypad } else { &dir_keypad };
    let mut current = 'A';

    for c in input_line.iter() {
        if let Some(new) = moves_map.get(&((current, *c), depth)) {
            length += new;
        } else {
            let new_variants = keypad.get(&(current, *c)).unwrap();
            let mut new_length = MAX;
            if depth == keypads_amount {
                new_length = new_variants[0].len();
            } else {
                for variant in new_variants.iter() {
                    let variant_length = make_code(
                        variant,
                        num_keypad,
                        dir_keypad,
                        depth + 1,
                        moves_map,
                        keypads_amount,
                    );
                    new_length = min(new_length, variant_length);
                }
            }
            moves_map.insert(((current, *c), depth), new_length);
            length += new_length;
        }
        current = *c;
    }
    length
}

fn parse_keypad(input: &str, x_coord: (i32, i32)) -> HashMap<(char, char), Vec<Vec<char>>> {
    let mut keypad: HashMap<(char, char), Vec<Vec<char>>> = HashMap::new();
    let mut keypad_list: Vec<(char, (i32, i32))> = Vec::new();
    input.lines().enumerate().for_each(|(row, line)| {
        line.chars()
            .enumerate()
            .for_each(|(col, c)| keypad_list.push((c, (row as i32, col as i32))))
    });

    keypad_list.iter().for_each(|(first_symbol, first_coord)| {
        keypad_list
            .iter()
            .for_each(|(second_symbol, second_coord)| {
                if *first_symbol != 'X' && *second_symbol != 'X' {
                    let diff = (
                        second_coord.0 - first_coord.0,
                        second_coord.1 - first_coord.1,
                    );
                    let entry = keypad.entry((*first_symbol, *second_symbol)).or_default();
                    let mut options: Vec<Vec<char>> = Vec::new();
                    let mut symbols: Vec<char> = Vec::new();
                    let symbol = if diff.0 > 0 { 'v' } else { '^' };
                    for _ in 0..diff.0.abs() {
                        symbols.push(symbol);
                    }
                    let symbol = if diff.1 > 0 { '>' } else { '<' };
                    for _ in 0..diff.1.abs() {
                        symbols.push(symbol);
                    }

                    let symbols_permutations = symbols.iter().permutations(symbols.len()).unique();
                    for perm in symbols_permutations {
                        let mut valid = true;
                        let mut coord = *first_coord;
                        let mut new: Vec<char> = Vec::new();
                        for c in perm.iter() {
                            match c {
                                '^' => coord.0 -= 1,
                                'v' => coord.0 += 1,
                                '>' => coord.1 += 1,
                                _ => coord.1 -= 1,
                            };
                            if coord == x_coord {
                                valid = false;
                                break;
                            }
                            new.push(**c);
                        }
                        if valid {
                            new.push('A');
                            options.push(new);
                        }
                    }
                    *entry = options;
                }
            })
    });
    keypad
}
