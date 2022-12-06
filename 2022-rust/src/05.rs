use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/05.txt").unwrap();
    let (crates_input, moves) = input.split_once("\n\n").unwrap();
    let mut crates: Vec<Vec<char>> = vec![Vec::new(); 9];

    crates_input.lines().for_each(|l| {
        l.chars().enumerate().for_each(|(i, c)| {
            if c.is_alphabetic() && (i - 1) % 4 == 0 {
                crates[(i - 1) / 4].push(c);
            }
        })
    });

    crates.iter_mut().for_each(|c| c.reverse());

    rearrange_crates(crates.clone(), moves, true);
    rearrange_crates(crates, moves, false);
}

fn rearrange_crates(mut crates: Vec<Vec<char>>, moves: &str, part_one: bool) {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    moves.lines().for_each(|line| {
        let caps = re.captures(line).unwrap();
        let amount = (&caps[1]).parse::<usize>().unwrap();
        let from = (&caps[2]).parse::<usize>().unwrap() - 1;
        let to = (&caps[3]).parse::<usize>().unwrap() - 1;

        let left = crates[from].len() - amount;
        let mut crates_to_move = crates[from][left..].to_vec();
        if part_one {
            crates_to_move.reverse();
        }
        crates[to].extend_from_slice(&crates_to_move);
        crates[from].truncate(left);
    });

    crates.iter().for_each(|s| print!("{}", s.last().unwrap()));
    println!();
}
