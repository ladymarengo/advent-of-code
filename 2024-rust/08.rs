use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    let input = read_to_string("input/08_test.txt").unwrap();
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut antinodes_part_one: HashSet<(i32, i32)> = HashSet::new();
    let mut antinodes_part_two: HashSet<(i32, i32)> = HashSet::new();
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    
    grid.iter().enumerate().for_each(|(row, line)| {
        line.iter().enumerate().for_each(|(col, &c)| {
            if c != '.' {
                let new_antenna = (row as i32, col as i32);
                if let Some(previous_antennas) = antennas.get(&c) {
                    previous_antennas.iter().for_each(|&antenna| {
                        let distance = (new_antenna.0 - antenna.0, new_antenna.1 - antenna.1);

                        antinodes_part_one.insert((antenna.0 - distance.0, antenna.1 - distance.1));
                        antinodes_part_one
                            .insert((new_antenna.0 + distance.0, new_antenna.1 + distance.1));

                        let mut new_node = antenna;
                        while new_node.0 >= 0 && new_node.1 >= 0 {
                            antinodes_part_two.insert(new_node);
                            new_node.0 -= distance.0;
                            new_node.1 -= distance.1;
                        }
                        new_node = new_antenna;
                        while new_node.0 < grid.len() as i32 && new_node.1 < grid[0].len() as i32 {
                            antinodes_part_two.insert(new_node);
                            new_node.0 += distance.0;
                            new_node.1 += distance.1;
                        }
                    });
                }
                let current_antennas = antennas.entry(c).or_default();
                current_antennas.push(new_antenna);
            }
        })
    });
    let result = antinodes_part_one
        .iter()
        .filter(|&&node| {
            node.0 >= 0
                && node.1 >= 0
                && node.0 < grid.len() as i32
                && node.1 < grid[0].len() as i32
        })
        .count();
    println!("First answer is {}", result);
    let result = antinodes_part_two
        .iter()
        .filter(|&&node| {
            node.0 >= 0
                && node.1 >= 0
                && node.0 < grid.len() as i32
                && node.1 < grid[0].len() as i32
        })
        .count();
    println!("Second answer is {}", result);
}
