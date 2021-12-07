use std::fs::read_to_string;
use statistical::median;

fn main() {
    let crabs: Vec<isize> = read_to_string("input/07").
        unwrap().split(',').map(|l| l.parse().unwrap()).collect();
    println!("First answer is {}, second answer is {}", solve(crabs.clone(), 1), solve(crabs, 2));
}

fn solve(crabs: Vec<isize>, part: usize) -> isize {
    let position = median(&crabs);
    check_position(&crabs, position, part)
}

fn check_position(crabs: &[isize], position: isize, part: usize) -> isize {
    let now = count_fuel(crabs, position, part);
    let up = count_fuel(crabs, position + 1, part);
    let down = count_fuel(crabs, position - 1, part);
    if up < now {
        check_position(crabs, position + 1, part)
    } else if down < now {
        check_position(crabs, position - 1, part)
    } else {
        now
    }
}

fn count_fuel(crabs: &[isize], position: isize, part: usize) -> isize {
    match part {
        1 => {
            let mut fuel: isize = 0;
            for crab in crabs {
                fuel += if crab < &position {position - crab} else {crab - position};
            }
            fuel
        },
        _ => {
            let mut fuel: isize = 0;
            for crab in crabs {
                let steps = if crab < &position {position - crab} else {crab - position};
                fuel += steps * (steps + 1) / 2;
            }
            fuel
        },
    }
}