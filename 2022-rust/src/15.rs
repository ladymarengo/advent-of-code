use regex::Regex;
use std::collections::HashSet;
use std::{
    cmp::{max, min},
    fs::read_to_string,
};

fn main() {
    let input = read_to_string("input/15.txt").unwrap();
    let re =
        Regex::new(r"Sensor at x=(-*\d+), y=(-*\d+): closest beacon is at x=(-*\d+), y=(-*\d+)")
            .unwrap();
    let sensors_beacons: Vec<((i32, i32), (i32, i32))> = input
        .lines()
        .map(|l| {
            let caps = re.captures(l).unwrap();
            let sensor_x = (&caps[1]).parse::<i32>().unwrap();
            let sensor_y = (&caps[2]).parse::<i32>().unwrap();
            let beacon_x = (&caps[3]).parse::<i32>().unwrap();
            let beacon_y = (&caps[4]).parse::<i32>().unwrap();
            ((sensor_x, sensor_y), (beacon_x, beacon_y))
        })
        .collect();

    part_one(sensors_beacons.clone());
    part_two(sensors_beacons);
}

fn part_one(sensors_beacons: Vec<((i32, i32), (i32, i32))>) {
    let y: i32 = 2000000;
    let mut covered_zones: Vec<(i32, i32)> = Vec::new();
    let mut beacons: HashSet<i32> = HashSet::new();
    sensors_beacons.iter().for_each(|(sensor, beacon)| {
        if beacon.1 == y {
            beacons.insert(beacon.0);
        }
        let distance = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();
        let y_distance = (sensor.1 - y).abs();
        if y_distance <= distance {
            let x_range = distance - y_distance;
            let (min_x, max_x) = (sensor.0 - x_range, sensor.0 + x_range);
            covered_zones.push((min_x, max_x));
        }
    });

    loop {
        if covered_zones.len() == 1 {
            break;
        }
        let zone = covered_zones.pop().unwrap();
        #[allow(clippy::needless_range_loop)]
        for i in 0..covered_zones.len() {
            if covered_zones[i].0 <= zone.1 && zone.0 <= covered_zones[i].1 {
                covered_zones[i] = (
                    min(covered_zones[i].0, zone.0),
                    max(covered_zones[i].1, zone.1),
                );
                break;
            }
        }
    }
    let mut positions = covered_zones[0].1 - covered_zones[0].0 + 1;
    beacons.iter().for_each(|b| {
        if (covered_zones[0].0..=covered_zones[0].1).contains(b) {
            positions -= 1;
        }
    });
    println!("Part one {positions}");
}

fn part_two(sensors_beacons: Vec<((i32, i32), (i32, i32))>) {
    let mut covered_zones: Vec<Vec<(i32, i32)>> = vec![Vec::new(); 4000001];
    let mut beacons: Vec<i32> = Vec::new();
    for y_i32 in 0..=4000000 {
        let y = y_i32 as usize;
        sensors_beacons.iter().for_each(|(sensor, beacon)| {
            if beacon.1 == y as i32 {
                beacons.push(beacon.0);
            }
            let distance = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();
            let y_distance = (sensor.1 - y as i32).abs();
            if y_distance <= distance {
                let x_range = distance - y_distance;
                let (min_x, max_x) = (sensor.0 - x_range, sensor.0 + x_range);
                if max_x >= 0 && min_x <= 4000000 {
                    covered_zones[y as usize].push((max(min_x, 0), min(max_x, 4000000)));
                }
            }
        });
        loop {
            if covered_zones[y].len() == 1 {
                break;
            }
            let zone = covered_zones[y].pop().unwrap();
            let mut found = false;
            for i in 0..covered_zones[y].len() {
                if covered_zones[y][i].0 <= zone.1 && zone.0 <= covered_zones[y][i].1 {
                    covered_zones[y][i] = (
                        min(covered_zones[y][i].0, zone.0),
                        max(covered_zones[y][i].1, zone.1),
                    );
                    found = true;
                    break;
                }
            }
            if !found {
                covered_zones[y].insert(0, zone);
                if covered_zones[y].len() == 2 {
                    println!(
                        "Part two {}",
                        (covered_zones[y][0].1 as usize + 1) * 4000000 + y
                    );
                    return;
                }
            }
        }
    }
}
