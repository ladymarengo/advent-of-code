use std::fs::read_to_string;

fn main() {
    let input = parse_input();
    let answers = solve(input, 100);
    println!(
        "First answer is {}, second answer is {}",
        answers.0, answers.1
    );
}

fn parse_input() -> Vec<Vec<u32>> {
    read_to_string("input/11")
        .unwrap()
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn solve(mut octopuses: Vec<Vec<u32>>, days: usize) -> (usize, usize) {
    let mut flashes: usize = 0;
    let size: usize = 10;
    let mut flashes_step: usize = 0;
    let mut steps: usize = 0;
    let mut first_answer: usize = 0;

    while flashes_step != 100 {
        flashes_step = 0;
        steps += 1;
        for row in 0..size {
            for place in 0..size {
                octopuses[row][place] += 1;
            }
        }
        for row in 0..size {
            for place in 0..size {
                check_octopus(&mut octopuses, row, place, size);
            }
        }
        for row in 0..size {
            for place in 0..size {
                if octopuses[row][place] == 0 {
                    flashes_step += 1;
                }
            }
        }

        flashes += flashes_step;
        if steps == days {
            first_answer = flashes;
        }
    }
    (first_answer, steps)
}

fn check_octopus(octopuses: &mut [Vec<u32>], row: usize, place: usize, size: usize) {
    let coordinates_to_check: Vec<(i32, i32)> = vec![
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, 1),
        (-1, -1),
        (1, -1),
        (1, 1),
    ];

    if octopuses[row][place] > 9 {
        octopuses[row][place] = 0;
        for coordinate in coordinates_to_check {
            let neighbour_row = row as i32 + coordinate.0;
            let neighbour_place = place as i32 + coordinate.1;
            if neighbour_row >= 0
                && neighbour_row < size as i32
                && neighbour_place >= 0
                && neighbour_place < size as i32
                && octopuses[neighbour_row as usize][neighbour_place as usize] != 0
            {
                octopuses[neighbour_row as usize][neighbour_place as usize] += 1;
                check_octopus(
                    octopuses,
                    neighbour_row as usize,
                    neighbour_place as usize,
                    size,
                );
            }
        }
    }
}
