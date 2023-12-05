use std::{
    cmp::{max, min},
    fs::read_to_string,
};

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
            let mut ranges = map
                .lines()
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
                .collect::<Vec<Range>>();
            ranges.sort_by_key(|r| r.source_start);
            ranges
        })
        .collect::<Vec<Vec<Range>>>();

    let mut locations = seeds
        .iter()
        .map(|seed| find_location(&maps, *seed))
        .collect::<Vec<i64>>();
    locations.sort();

    println!("First answer is {}", locations[0]);

    let mut available_ranges = (0..seeds.len())
        .step_by(2)
        .map(|i| (seeds[i], seeds[i] + seeds[i + 1] - 1))
        .collect::<Vec<(i64, i64)>>();
    available_ranges.sort_by_key(|r| r.0);

    maps.iter()
        .for_each(|map| find_next_ranges(&mut available_ranges, map));

    println!("Second_answer is {}", available_ranges[0].0);
}

fn find_next_ranges(available_ranges: &mut Vec<(i64, i64)>, map: &[Range]) {
    let mut new_ranges: Vec<(i64, i64)> = Vec::new();

    available_ranges.iter().for_each(|range| {
        let mut start = range.0;
        let end = range.1;
        map.iter().for_each(|next_range| {
            if start <= end {
                if start < next_range.source_start {
                    new_ranges.push((start, min(next_range.source_start - 1, end)));
                    start = min(next_range.source_start - 1, end) + 1;
                }
                if end >= next_range.source_start && start <= next_range.source_end {
                    new_ranges.push((
                        max(start, next_range.source_start) + next_range.offset,
                        min(end, next_range.source_end) + next_range.offset,
                    ));
                    start = min(end, next_range.source_end) + 1;
                }
            }
        });
        if start <= end {
            new_ranges.push((start, end));
        }
    });

    new_ranges.sort_by_key(|r| r.0);
    *available_ranges = new_ranges;
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
