use std::{
    cmp::{max, min},
    fs::read_to_string,
};
use regex::Regex;
use std::collections::HashSet;

fn main() {
    let input = read_to_string("input/15_test.txt").unwrap();
	let re = Regex::new(r"Sensor at x=(-*\d+), y=(-*\d+): closest beacon is at x=(-*\d+), y=(-*\d+)").unwrap();
	let sensors_beacons: Vec<((i32, i32), (i32, i32))> = input.lines().map(|l| {
		let caps = re.captures(l).unwrap();
        let sensor_x = (&caps[1]).parse::<i32>().unwrap();
		let sensor_y = (&caps[2]).parse::<i32>().unwrap();
		let beacon_x = (&caps[3]).parse::<i32>().unwrap();
		let beacon_y = (&caps[4]).parse::<i32>().unwrap();
		((sensor_x, sensor_y), (beacon_x, beacon_y))
	}).collect();

	let y: i32 = 10;
	let mut covered_zones: Vec<(i32, i32)> = Vec::new();
	let mut beacons: Vec<i32> = Vec::new();
	sensors_beacons.iter().for_each(|(sensor, beacon)| {
		if beacon.1 == y {
			beacons.push(beacon.0);
		}
		let distance = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();
		let y_distance = (sensor.1 - y).abs();
		if y_distance <= distance {
			let x_range = distance - y_distance;
			let (min_x, max_x) = (sensor.0 - x_range, sensor.0 + x_range);
			covered_zones.push((min_x, max_x));
		}
	});

	let mut positions: HashSet<i32> = HashSet::new();
	covered_zones.iter().for_each(|z| {
		(z.0..=z.1).for_each(|x| {
			if !beacons.contains(&x) {
				positions.insert(x);
			}
		});
	});
	let mut pos: Vec<i32> = positions.iter().map(|v| *v).collect();
	pos.sort();
	println!("{}", pos.len());

}
