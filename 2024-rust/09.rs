use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/09.txt").unwrap();
    let mut disk_map: Vec<usize> = Vec::new();
    let mut id_number: usize = 1;
    input.chars().enumerate().for_each(|(i, c)| {
        let number = c.to_digit(10).unwrap();
        if i % 2 == 0 {
            (0..number).for_each(|_| disk_map.push(id_number));
            id_number += 1;
        } else {
            (0..number).for_each(|_| disk_map.push(0));
        }
    });

    let mut result: usize = 0;
    let mut i: usize = 0;
    while i < disk_map.len() {
        if disk_map[i] == 0 {
            let mut id: usize = 0;
            while id == 0 {
                id = disk_map.pop().unwrap();
            }
            disk_map[i] = id;
        }
        result += i * (disk_map[i] - 1);
        i += 1;
    }

    println!("First answer is {result}");

    let mut disk_map: Vec<Block> = Vec::new();
    let mut spaces: Vec<Space> = Vec::new();
    let mut id_number: usize = 0;

    input.chars().enumerate().for_each(|(i, c)| {
        let number = c.to_digit(10).unwrap();
        if i % 2 == 0 {
            disk_map.push(Block {
                is_file: true,
                value: number as usize,
                index: id_number,
                closest_space: spaces.len(),
            });
            id_number += 1;
        } else if number > 0 {
            disk_map.push(Block {
                is_file: false,
                value: 0,
                index: spaces.len(),
                closest_space: 0,
            });
            spaces.push(Space {
                space_left: number as usize,
                files: Vec::new(),
            });
        }
    });

    let mut i: usize = disk_map.len() - 1;
    while i > 0 {
        if disk_map[i].is_file {
            if let Some(space) = spaces[..disk_map[i].closest_space]
                .iter_mut()
                .find(|s| s.space_left >= disk_map[i].value)
            {
                space.files.push((disk_map[i].value, disk_map[i].index));
                space.space_left -= disk_map[i].value;
                disk_map[i].is_file = false;
            }
        }
        i -= 1;
    }

    let mut result = 0;
    let mut i: usize = 0;

    disk_map.iter().for_each(|block| {
        if block.is_file {
            (0..block.value).for_each(|_| {
                result += i * block.index;
                i += 1;
            });
        } else if block.value == 0 {
            let space = &spaces[block.index];
            space.files.iter().for_each(|file| {
                (0..file.0).for_each(|_| {
                    result += i * file.1;
                    i += 1;
                });
            });
            i += space.space_left;
        } else {
            i += block.value;
        }
    });

    println!("Second answer is {result}");
}

#[derive(Debug)]
struct Block {
    is_file: bool,
    value: usize,
    index: usize,
    closest_space: usize,
}

#[derive(Debug)]
struct Space {
    space_left: usize,
    files: Vec<(usize, usize)>,
}
