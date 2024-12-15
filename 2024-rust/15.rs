use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/15.txt").unwrap();
    let (map_input, moves) = input.split_once("\n\n").unwrap();
    let mut robot: (usize, usize) = (0, 0);
    let mut map: Vec<Vec<Block>> = map_input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| {
                    if c == '@' {
                        robot = (row, col);
                    }
                    match c {
                        '@' => Block::Robot,
                        '#' => Block::Wall,
                        'O' => Block::Box,
                        _ => Block::Empty,
                    }
                })
                .collect()
        })
        .collect();

    make_moves(&mut map, moves, robot, false);
    let result: usize = map
        .iter()
        .enumerate()
        .map(|(row, line)| {
            line.iter()
                .enumerate()
                .map(|(col, block)| {
                    if *block == Block::Box {
                        100 * row + col
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum();
    println!("First answer is {result}");

    let mut map: Vec<Vec<Block>> = Vec::new();
    map_input.lines().enumerate().for_each(|(row, line)| {
        map.push(Vec::new());
        line.chars().enumerate().for_each(|(col, c)| {
            if c == '@' {
                robot = (row, col * 2);
            }
            match c {
                '@' => {
                    map[row].push(Block::Robot);
                    map[row].push(Block::Empty);
                }
                '#' => {
                    map[row].push(Block::Wall);
                    map[row].push(Block::Wall);
                }
                'O' => {
                    map[row].push(Block::LeftBox);
                    map[row].push(Block::RightBox);
                }
                _ => {
                    map[row].push(Block::Empty);
                    map[row].push(Block::Empty);
                }
            }
        })
    });

    make_moves(&mut map, moves, robot, true);

    let result: usize = map
        .iter()
        .enumerate()
        .map(|(row, line)| {
            line.iter()
                .enumerate()
                .map(|(col, block)| {
                    if *block == Block::LeftBox {
                        100 * row + col
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum();
    println!("Second answer is {result}");
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Block {
    Wall,
    Empty,
    Box,
    LeftBox,
    RightBox,
    Robot,
}

fn make_moves(map: &mut Vec<Vec<Block>>, moves: &str, mut robot: (usize, usize), part_two: bool) {
    moves.chars().for_each(|c| {
        if c != '\n' {
            let direction = match c {
                '^' => (-1, 0),
                '>' => (0, 1),
                'v' => (1, 0),
                '<' => (0, -1),
                _ => (0, 0),
            };
            let next = (
                (robot.0 as i32 + direction.0) as usize,
                (robot.1 as i32 + direction.1) as usize,
            );
            if map[next.0][next.1] != Block::Wall {
                if map[next.0][next.1] == Block::Empty {
                    map[next.0][next.1] = Block::Robot;
                    map[robot.0][robot.1] = Block::Empty;
                    robot = next;
                } else if !part_two {
                    let mut final_block: (i32, i32) = (next.0 as i32, next.1 as i32);
                    while map[final_block.0 as usize][final_block.1 as usize] == Block::Box {
                        final_block = (final_block.0 + direction.0, final_block.1 + direction.1);
                    }
                    if map[final_block.0 as usize][final_block.1 as usize] == Block::Empty {
                        map[final_block.0 as usize][final_block.1 as usize] = Block::Box;
                        map[next.0][next.1] = Block::Robot;
                        map[robot.0][robot.1] = Block::Empty;
                        robot = next;
                    }
                } else {
                    let mut moving_blocks: Vec<(i32, i32)> =
                        Vec::from([(next.0 as i32, next.1 as i32)]);
                    if map[next.0][next.1] == Block::LeftBox {
                        moving_blocks.push((next.0 as i32, next.1 as i32 + 1));
                    } else {
                        moving_blocks.push((next.0 as i32, next.1 as i32 - 1));
                    }
                    let mut can_move = true;
                    let mut i = 0;
                    while i < moving_blocks.len() {
                        let block = moving_blocks[i];
                        let next_block = (block.0 + direction.0, block.1 + direction.1);
                        if map[next_block.0 as usize][next_block.1 as usize] == Block::Wall {
                            can_move = false;
                            break;
                        }
                        if map[next_block.0 as usize][next_block.1 as usize] != Block::Empty {
                            if direction.1 == 0 {
                                if !moving_blocks.contains(&(next_block.0, next_block.1)) {
                                    moving_blocks.push((next_block.0, next_block.1));
                                }
                                if map[next_block.0 as usize][next_block.1 as usize]
                                    == Block::LeftBox
                                {
                                    if !moving_blocks.contains(&(next_block.0, next_block.1 + 1)) {
                                        moving_blocks.push((next_block.0, next_block.1 + 1));
                                    }
                                } else {
                                    if !moving_blocks.contains(&(next_block.0, next_block.1 - 1)) {
                                        moving_blocks.push((next_block.0, next_block.1 - 1));
                                    }
                                }
                            } else {
                                if !moving_blocks.contains(&(next_block.0, next_block.1)) {
                                    moving_blocks.push((next_block.0, next_block.1));
                                }
                            }
                        }
                        i += 1;
                    }
                    if can_move {
                        moving_blocks.iter().rev().for_each(|block| {
                            map[(block.0 + direction.0) as usize]
                                [(block.1 + direction.1) as usize] =
                                map[block.0 as usize][block.1 as usize];
                            if !moving_blocks
                                .contains(&(block.0 - direction.0, block.1 - direction.1))
                            {
                                map[block.0 as usize][block.1 as usize] = Block::Empty;
                            }
                        });
                        map[next.0][next.1] = Block::Robot;
                        map[robot.0][robot.1] = Block::Empty;
                        robot = next;
                    }
                }
            }
        }
    });
}
