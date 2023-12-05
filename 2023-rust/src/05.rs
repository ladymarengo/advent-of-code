use std::fs::read_to_string;

#[derive(Debug)]
struct Range {
    source_start: i64,
    source_end: i64,
    offset: i64,
}

fn main() {
    let input = read_to_string("input/05.txt").unwrap();

    let almanac: Vec<&str> = input.split("\n\n").collect();

    let seeds = almanac[0]
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|seed| seed.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let maps = almanac[1..]
        .iter()
        .map(|map| {
            map.lines()
                .skip(1)
                .map(|line| {
                    let numbers: Vec<i64> = line
                        .split_whitespace()
                        .map(|number| number.parse::<i64>().unwrap())
                        .collect();
                    Range {
                        source_start: numbers[1],
                        source_end: numbers[1] + numbers[2] - 1,
                        offset: numbers[0] - numbers[1],
                    }
                })
                .collect::<Vec<Range>>()
        })
        .collect::<Vec<Vec<Range>>>();

    let mut locations = seeds
        .iter()
        .map(|seed| find_location(&maps, *seed))
        .collect::<Vec<i64>>();
    locations.sort();

    println!("First answer is {}", locations[0]);
}

fn find_location(maps: &[Vec<Range>], seed: i64) -> i64 {
    let mut current_number = seed;
    maps.iter().for_each(|map| {
        let range = map
            .iter()
            .find(|r| current_number >= r.source_start && current_number <= r.source_end);
        if let Some(r) = range {
            current_number += r.offset;
        }
    });
    current_number
}
