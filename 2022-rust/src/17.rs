use std::fs::read_to_string;

fn main() {
    let input: Vec<char> = read_to_string("input/17.txt").unwrap().chars().collect();
    let shapes: Vec<Vec<Vec<bool>>> = read_to_string("input/17_shapes.txt")
        .unwrap()
        .split("\n\n")
        .map(|s| {
            s.lines()
                .map(|l| l.chars().map(|c| c == '#').collect())
                .collect()
        })
        .collect();

    solve(&input, &shapes, 2022);
}

fn solve(pattern: &[char], shapes: &[Vec<Vec<bool>>], max_rocks: usize) {
    let mut chamber = vec![vec![true; 7]; 1];
    let mut rocks: usize = 1;
    let mut step: usize = 0;
    let mut rock = Rock {
        shape: 0,
        x: 2,
        y: 4,
    };

    loop {
        move_rock(&mut rock, pattern[step % pattern.len()], shapes, &chamber);
        step += 1;
        if !move_rock(&mut rock, 'v', shapes, &chamber) {
            rest_rock(&mut rock, shapes, &mut chamber);
            rock = Rock {
                shape: (rock.shape + 1) % shapes.len(),
                x: 2,
                y: chamber.len() + 2 + shapes[(rock.shape + 1) % shapes.len()].len(),
            };
            rocks += 1;

            if rocks > max_rocks {
                println!("{}", chamber.len() - 1);
                return;
            }
        }
    }
}

fn move_rock(
    rock: &mut Rock,
    direction: char,
    shapes: &[Vec<Vec<bool>>],
    chamber: &[Vec<bool>],
) -> bool {
    let (new_x, new_y) = match direction {
        '>' => (rock.x as i32 + 1, rock.y),
        '<' => (rock.x as i32 - 1, rock.y),
        'v' => (rock.x as i32, rock.y - 1),
        _ => (0, 0),
    };
    if new_x < 0 || new_x + shapes[rock.shape][0].len() as i32 > 7 {
        return false;
    }
    for (y, line) in shapes[rock.shape].iter().enumerate() {
        for (x, piece) in line.iter().enumerate() {
            if *piece {
                let chamber_x = new_x + x as i32;
                let chamber_y = new_y - y;

                if chamber_y < chamber.len() && chamber[chamber_y][chamber_x as usize] {
                    return false;
                }
            }
        }
    }
    rock.x = new_x as usize;
    rock.y = new_y;
    true
}

fn rest_rock(rock: &mut Rock, shapes: &[Vec<Vec<bool>>], chamber: &mut Vec<Vec<bool>>) {
    if rock.y >= chamber.len() {
        (0..=rock.y - chamber.len()).for_each(|_i| chamber.push(vec![false; 7]));
    }
    for (y, line) in shapes[rock.shape].iter().enumerate() {
        for (x, piece) in line.iter().enumerate() {
            if *piece {
                chamber[rock.y - y][rock.x + x] = true;
            }
        }
    }
}

#[derive(Debug)]
struct Rock {
    shape: usize,
    x: usize,
    y: usize,
}
