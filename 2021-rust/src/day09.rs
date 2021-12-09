use std::fs::read_to_string;

fn main() {
    let input = parse_input();
    println!("First answer is {}", count_low_points(&input));
}

fn parse_input() -> Vec<Vec<u32>> {
    read_to_string("input/09")
        .unwrap()
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn count_low_points(heightmap: &[Vec<u32>]) -> u32 {
    let mut risk_level: u32 = 0;
    let y = heightmap.len();
    let x = heightmap[0].len();

    for row in 0..y {
        for number in 0..x {
            let point = heightmap[row][number];
            if (row > 0 && heightmap[row - 1][number] <= point) ||
            (row < y - 1 && heightmap[row + 1][number] <= point) ||
            (number > 0 && heightmap[row][number - 1] <= point) ||
            (number < x - 1 && heightmap[row][number + 1] <= point) {
                continue;
            } else {
                risk_level += point + 1;
            }
        }
    }
    risk_level
}