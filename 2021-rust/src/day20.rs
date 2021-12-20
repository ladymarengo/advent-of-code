use std::fs::read_to_string;

fn main() {
    let (algorithm, image) = parse_input();
    println!("First answer is {}", solve(&algorithm, image.clone(), 2));
    println!("Second answer is {}", solve(&algorithm, image, 50));
}

fn parse_input() -> (Vec<char>, Vec<Vec<char>>) {
    let input = read_to_string("input/20").unwrap();
    let split = input.split_once("\n\n").unwrap();

    let algorithm: Vec<char> = split
        .0
        .chars()
        .map(|c| if c == '#' { '1' } else { '0' })
        .collect();
    let image: Vec<Vec<char>> = split
        .1
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| if c == '#' { '1' } else { '0' })
                .collect()
        })
        .collect();
    (algorithm, image)
}

fn solve(algorithm: &[char], mut image: Vec<Vec<char>>, steps: usize) -> usize {
    image = scale_image(&image, steps + 5);
    for _i in 0..steps {
        image = enhance(&image, algorithm);
    }

    let mut count: usize = 0;
    for line in &image {
        for char in line {
            if *char == '1' {
                count += 1;
            }
        }
    }
    count
}

fn scale_image(image: &[Vec<char>], size: usize) -> Vec<Vec<char>> {
    let mut new: Vec<Vec<char>> = vec![];
    for _i in 0..size {
        new.push(vec!['0'; image[0].len() + (size * 2)]);
    }
    for line in image {
        let mut row: Vec<char> = vec!['0'; size];
        row.extend_from_slice(line);
        row.extend_from_slice(&vec!['0'; size]);
        new.push(row);
    }
    for _i in 0..size {
        new.push(vec!['0'; image[0].len() + (size * 2)]);
    }
    new
}

fn enhance(image: &[Vec<char>], algorithm: &[char]) -> Vec<Vec<char>> {
    let mut new: Vec<Vec<char>> = vec![];

    for (i, line) in image.iter().enumerate() {
        let mut row: Vec<char> = vec![];
        for (j, char) in line.iter().enumerate() {
            let mut index = String::new();

            for m in -1_i32..2 {
                for n in -1_i32..2 {
                    if i as i32 + m >= 0
                        && j as i32 + n >= 0
                        && i as i32 + m < image.len() as i32
                        && j as i32 + n < image[0].len() as i32
                    {
                        index.push(image[(i as i32 + m) as usize][(j as i32 + n) as usize]);
                    } else {
                        index.push(*char);
                    }
                }
            }
            row.push(algorithm[usize::from_str_radix(&index, 2).unwrap()]);
        }
        new.push(row);
    }
    new
}
