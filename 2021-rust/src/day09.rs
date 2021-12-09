use std::fs::read_to_string;

#[derive(Debug, PartialEq, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
    value: u32,
}

fn main() {
    let input = parse_input();
    let low_points = count_low_points(&input);
    println!(
        "First answer is {}, second answer is {}",
        part_one(&low_points),
        part_two(&input, &low_points)
    );
}

fn parse_input() -> Vec<Vec<u32>> {
    read_to_string("input/09")
        .unwrap()
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn count_low_points(heightmap: &[Vec<u32>]) -> Vec<Point> {
    let mut low_points: Vec<Point> = vec![];
    let y = heightmap.len();
    let x = heightmap[0].len();

    for row in 0..y {
        for number in 0..x {
            let point = Point {
                x: number as i32,
                y: row as i32,
                value: heightmap[row][number],
            };
            if (row > 0 && heightmap[row - 1][number] <= point.value)
                || (row < y - 1 && heightmap[row + 1][number] <= point.value)
                || (number > 0 && heightmap[row][number - 1] <= point.value)
                || (number < x - 1 && heightmap[row][number + 1] <= point.value)
            {
                continue;
            } else {
                low_points.push(point);
            }
        }
    }
    low_points
}

fn part_one(low_points: &[Point]) -> u32 {
    let mut risk_level: u32 = 0;
    for point in low_points {
        risk_level += point.value + 1;
    }
    risk_level
}

fn part_two(heightmap: &[Vec<u32>], low_points: &[Point]) -> u32 {
    let mut basins: Vec<u32> = vec![];
    for point in low_points {
        let mut counted: Vec<Point> = vec![*point];
        basins.push(count_neighbours(heightmap, point, &mut counted));
    }
    basins.sort_by(|a, b| b.cmp(a));
    basins[0] * basins[1] * basins[2]
}

fn count_neighbours(heightmap: &[Vec<u32>], point: &Point, counted: &mut Vec<Point>) -> u32 {
    let mut neighbours: u32 = 1;
    let y = heightmap.len() as i32;
    let x = heightmap[0].len() as i32;
    let coordinates_to_check: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

    for coordinate in coordinates_to_check {
        let neighbour_x = point.x + coordinate.0;
        let neighbour_y = point.y + coordinate.1;
        if neighbour_x >= 0
            && neighbour_x < x
            && neighbour_y >= 0
            && neighbour_y < y
            && heightmap[neighbour_y as usize][neighbour_x as usize] != 9
        {
            let neighbour = Point {
                x: neighbour_x,
                y: neighbour_y,
                value: heightmap[neighbour_y as usize][neighbour_x as usize],
            };
            if !counted.contains(&neighbour) {
                counted.push(neighbour);
                neighbours += count_neighbours(heightmap, &neighbour, counted);
            }
        }
    }

    neighbours
}
