use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Scanner {
    number: usize,
    coordinates: (i32, i32, i32),
    beacons: Vec<Beacon>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Beacon {
    coords: [i32; 3],
}

struct Direction {
    x_sign: i32,
    y_sign: i32,
    z_sign: i32,
    x_index: usize,
    y_index: usize,
    z_index: usize,
}

fn main() {
    let input = parse_input();
    solve(input);
}

fn parse_input() -> Vec<Scanner> {
    let mut scanners: Vec<Scanner> = vec![];
    
    let input: Vec<String> = read_to_string("input/19")
        .unwrap()
        .split("\n\n")
        .map(|s| s.to_string())
        .collect();
    
    for (e, line) in input.iter().enumerate() {
        let mut beacons: Vec<Beacon> = vec![];
        for beacon in line.lines().map(|l| l.to_string()).skip(1) {
            let split: Vec<i32> = beacon
                .split(',')
                .map(|b| b.parse::<i32>().unwrap())
                .collect();
            beacons.push(Beacon {
                coords: [split[0], split[1], split[2]],
            });
        }
        scanners.push(Scanner {
            number: e,
            coordinates: (0, 0, 0),
            beacons,
        });
    }
    scanners
}

fn solve(mut input: Vec<Scanner>) {
    let rotated = rotate_all(&mut input);
    let mut beacons: Vec<Beacon> = vec![];
    
    for scanner in &rotated {
        for beacon in &scanner.beacons {
            if !beacons.contains(beacon) {
                beacons.push(beacon.clone());
            }
        }
    }
    println!("First answer is {}", beacons.len());
    println!("Second answer is {}", calculate_max_distance(rotated));
}

fn rotate_all(input: &mut Vec<Scanner>) -> Vec<Scanner> {
    let mut scanners: Vec<Scanner> = vec![input.first().unwrap().clone()];
    let mut checked: HashMap<usize, Vec<usize>> = HashMap::new();
    input.remove(0);
    
    let mut added = false;
    let mut index: usize = 0;

    while !input.is_empty() {
        'main: for scanner in scanners.clone() {
            checked.entry(scanner.number).or_default();
            
            for (i, other_scanner) in input.iter().enumerate() {
                if !checked[&scanner.number].contains(&other_scanner.number) {
                    checked
                        .entry(scanner.number)
                        .or_default()
                        .push(other_scanner.number);
                    
                        for direction in get_directions() {
                        let (result, new) = overlap(&scanner, other_scanner, &direction);
                        if result {
                            scanners.push(new);
                            added = true;
                            index = i;
                            break 'main;
                        }
                    }
                }
            }
        }
        if added {
            input.remove(index);
            added = false;
        }
    }
    scanners
}

fn overlap(
    first: &Scanner,
    second: &Scanner,
    direction: &Direction,
) -> (bool, Scanner) {
    
    for beacon in &first.beacons {
        for other_beacon in &second.beacons {
            let rotated = Beacon {
                coords: [
                    direction.x_sign * other_beacon.coords[direction.x_index],
                    direction.y_sign * other_beacon.coords[direction.y_index],
                    direction.z_sign * other_beacon.coords[direction.z_index],
                ],
            };
            let diff = (
                beacon.coords[0] - rotated.coords[0],
                beacon.coords[1] - rotated.coords[1],
                beacon.coords[2] - rotated.coords[2],
            );
            let new = rotate_scanner(second, &diff, direction);
            if fit_12(first, &new) {
                return (true, new);
            }
        }
    }
    (false, second.clone())
}

fn rotate_scanner(scanner: &Scanner, diff: &(i32, i32, i32), direction: &Direction) -> Scanner {
    let mut new_beacons: Vec<Beacon> = vec![];
    
    for beacon in &scanner.beacons {
        new_beacons.push(Beacon {
            coords: [
                direction.x_sign * beacon.coords[direction.x_index] + diff.0,
                direction.y_sign * beacon.coords[direction.y_index] + diff.1,
                direction.z_sign * beacon.coords[direction.z_index] + diff.2,
            ],
        });
    }
    Scanner {
        number: scanner.number,
        coordinates: *diff,
        beacons: new_beacons,
    }
}

fn fit_12(first: &Scanner, new: &Scanner) -> bool {
    let mut count: usize = 0;

    for beacon in &first.beacons {
        for (e, other) in new.beacons.iter().enumerate() {
            if *beacon == *other {
                count += 1;
                if count >= 12 {
                    return true;
                }
            } else if new.beacons.len() * first.beacons.len() - e < 12 - count {
                return false;
            }
        }
    }
    if count >= 12 {
        return true;
    }
    false
}

fn calculate_max_distance(scanners: Vec<Scanner>) -> i32 {
    let mut max_distance: i32 = 0;
    
    for scanner in &scanners {
        for other_scanner in &scanners {
            let distance = (scanner.coordinates.0 - other_scanner.coordinates.0).abs()
                + (scanner.coordinates.1 - other_scanner.coordinates.1).abs()
                + (scanner.coordinates.2 - other_scanner.coordinates.2).abs();
            if distance > max_distance {
                max_distance = distance;
            }
        }
    }
    max_distance
}

#[rustfmt::skip]
fn get_directions() -> Vec<Direction> {   
    vec![Direction{x_sign: 1, x_index: 0, y_sign: 1, y_index: 2, z_sign: -1, z_index: 1},
        Direction{x_sign: 1, x_index: 0, y_sign: -1, y_index: 2, z_sign: 1, z_index: 1},
        Direction{x_sign: 1, x_index: 0, y_sign: 1, y_index: 1, z_sign: 1, z_index: 2},
        Direction{x_sign: 1, x_index: 0, y_sign: -1, y_index: 1, z_sign: -1, z_index: 2},
        Direction{x_sign: 1, x_index: 2, y_sign: 1, y_index: 0, z_sign: 1, z_index: 1},
        Direction{x_sign: -1, x_index: 2, y_sign: 1, y_index: 0, z_sign: -1, z_index: 1},
        Direction{x_sign: -1, x_index: 1, y_sign: 1, y_index: 0, z_sign: 1, z_index: 2},
        Direction{x_sign: 1, x_index: 1, y_sign: 1, y_index: 0, z_sign: -1, z_index: 2},
        Direction{x_sign: 1, x_index: 2, y_sign: -1, y_index: 1, z_sign: 1, z_index: 0},
        Direction{x_sign: -1, x_index: 2, y_sign: 1, y_index: 1, z_sign: 1, z_index: 0},
        Direction{x_sign: 1, x_index: 1, y_sign: 1, y_index: 2, z_sign: 1, z_index: 0},
        Direction{x_sign: -1, x_index: 1, y_sign: -1, y_index: 2, z_sign: 1, z_index: 0},
        Direction{x_sign: -1, x_index: 0, y_sign: 1, y_index: 2, z_sign: 1, z_index: 1},
        Direction{x_sign: -1, x_index: 0, y_sign: -1, y_index: 2, z_sign: -1, z_index: 1},
        Direction{x_sign: -1, x_index: 0, y_sign: -1, y_index: 1, z_sign: 1, z_index: 2},
        Direction{x_sign: -1, x_index: 0, y_sign: 1, y_index: 1, z_sign: -1, z_index: 2},
        Direction{x_sign: 1, x_index: 2, y_sign: -1, y_index: 0, z_sign: -1, z_index: 1},
        Direction{x_sign: -1, x_index: 2, y_sign: -1, y_index: 0, z_sign: 1, z_index: 1},
        Direction{x_sign: 1, x_index: 1, y_sign: -1, y_index: 0, z_sign: 1, z_index: 2},
        Direction{x_sign: -1, x_index: 1, y_sign: -1, y_index: 0, z_sign: -1, z_index: 2},
        Direction{x_sign: 1, x_index: 2, y_sign: 1, y_index: 1, z_sign: -1, z_index: 0},
        Direction{x_sign: -1, x_index: 2, y_sign: -1, y_index: 1, z_sign: -1, z_index: 0},
        Direction{x_sign: -1, x_index: 1, y_sign: 1, y_index: 2, z_sign: -1, z_index: 0},
        Direction{x_sign: 1, x_index: 1, y_sign: -1, y_index: 2, z_sign: -1, z_index: 0},]
}
