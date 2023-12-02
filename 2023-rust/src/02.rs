use std::{cmp::max, fs::read_to_string};

struct Set {
    red: usize,
    green: usize,
    blue: usize,
}

fn main() {
    let input = read_to_string("input/02.txt").unwrap();

    let mut powers: usize = 0;

    let possible_games: usize = input
        .lines()
        .enumerate()
        .map(|(id, line)| {
            // parse game
            let mut sets = line.split_once(": ").unwrap().1.split("; ").map(|cubes| {
                let mut set = Set {
                    red: 0,
                    green: 0,
                    blue: 0,
                };
                for pack in cubes.split(", ") {
                    let amount = pack.split_once(' ').unwrap().0.parse::<usize>().unwrap();
                    match pack.split_once(' ').unwrap().1 {
                        "red" => set.red = amount,
                        "green" => set.green = amount,
                        "blue" => set.blue = amount,
                        _ => (),
                    }
                }
                set
            });

            // calculate power for the second answer
            let mut max_set = Set {
                red: 0,
                green: 0,
                blue: 0,
            };
            sets.clone().for_each(|set| {
                max_set.red = max(max_set.red, set.red);
                max_set.green = max(max_set.green, set.green);
                max_set.blue = max(max_set.blue, set.blue);
            });
            powers += max_set.red * max_set.green * max_set.blue;

            // check if game is possible for the first answer
            if !sets.any(|set| set.red > 12 || set.green > 13 || set.blue > 14) {
                return id + 1;
            }
            0
        })
        .sum();

    println!("First answer is {possible_games}, second answer is {powers}");
}
