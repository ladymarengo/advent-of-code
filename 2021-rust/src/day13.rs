use regex::Regex;
use std::fs::read_to_string;

#[derive(Debug, PartialEq, Copy, Clone)]
struct Dot(usize, usize);

#[derive(Debug, Clone)]
struct Fold(String, usize);

fn main() {
    let (dots, instructions) = parse_input();
    solve(dots, instructions);
}

fn parse_input() -> (Vec<Dot>, Vec<Fold>) {
    let input: Vec<String> = read_to_string("input/13")
        .unwrap()
        .split("\n\n")
        .map(|l| l.to_string())
        .collect();

    let dots: Vec<Dot> = input[0]
        .lines()
        .map(|d| {
            let mut spl = d.split(',');
            Dot(
                spl.next().unwrap().parse::<usize>().unwrap(),
                spl.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect();

    let re = Regex::new(r"fold along (x|y)=(\d+)").unwrap();
    let instructions: Vec<Fold> = input[1]
        .lines()
        .map(|d| {
            let caps = re.captures(d).unwrap();
            Fold(caps[1].to_string(), caps[2].parse::<usize>().unwrap())
        })
        .collect();
    (dots, instructions)
}

fn solve(mut dots: Vec<Dot>, instructions: Vec<Fold>) {
    for (i, instruction) in instructions.iter().enumerate() {
        dots = fold_paper(dots, instruction);
        if i == 0 {
            println!("First answer is {}", dots.len());
        }
    }
    print_paper(dots);
}

fn fold_paper(dots: Vec<Dot>, instruction: &Fold) -> Vec<Dot> {
    let mut new_dots: Vec<Dot> = vec![];

    for dot in &dots {
        match instruction.0.as_str() {
            "x" => match dot.0 {
                x if x < instruction.1 => {
                    if !(&new_dots).contains(dot) {
                        new_dots.push(Dot(dot.0, dot.1))
                    }
                }
                x => {
                    if !(&new_dots).contains(&Dot(instruction.1 - (x - instruction.1), dot.1)) {
                        new_dots.push(Dot(instruction.1 - (x - instruction.1), dot.1))
                    }
                }
            },
            "y" => match dot.1 {
                y if y < instruction.1 => {
                    if !(&new_dots).contains(dot) {
                        new_dots.push(Dot(dot.0, dot.1))
                    }
                }
                y => {
                    if !(&new_dots).contains(&Dot(dot.0, instruction.1 - (y - instruction.1))) {
                        new_dots.push(Dot(dot.0, instruction.1 - (y - instruction.1)))
                    }
                }
            },
            _ => (),
        }
    }
    new_dots
}

fn print_paper(dots: Vec<Dot>) {
    let x_max = dots
        .clone()
        .into_iter()
        .map(|Dot(v, _)| v)
        .fold(0, std::cmp::max);
    let y_max = dots
        .clone()
        .into_iter()
        .map(|Dot(_, v)| v)
        .fold(0, std::cmp::max);

    for x in 0..=x_max {
        for y in 0..=y_max {
            if dots.contains(&Dot(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
