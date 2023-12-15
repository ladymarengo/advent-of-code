use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/15.txt").unwrap();

    let mut boxes: Vec<Vec<(&str, usize)>> = vec![vec![]; 256];

    let mut results: usize = input
        .split(',')
        .map(|string| {
            let mut value = 0;
            let mut label = "";
            let mut box_number = 0;
            string.chars().enumerate().for_each(|(i, c)| {
                if c == '=' || c == '-' {
                    label = &string[..i];
                    box_number = value;
                    let position = boxes[box_number]
                        .iter()
                        .position(|(string, _fl)| *string == label);
                    if c == '=' {
                        let focal_length = string[i + 1..].parse::<usize>().unwrap();
                        if position.is_some() {
                            boxes[box_number][position.unwrap()].1 = focal_length;
                        } else {
                            boxes[box_number].push((label, focal_length));
                        }
                    } else if position.is_some() {
                        boxes[box_number].remove(position.unwrap());
                    }
                }
                value = (value + c as usize) * 17 % 256;
            });
            value
        })
        .sum();

    println!("First answer is {results}");

    results = boxes
        .iter()
        .enumerate()
        .map(|(box_number, box_contents)| {
            box_contents
                .iter()
                .enumerate()
                .map(|(lens_number, (_label, focal_length))| {
                    (box_number + 1) * (lens_number + 1) * focal_length
                })
                .sum::<usize>()
        })
        .sum();

    println!("Second answer is {results}");
}
