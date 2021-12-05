use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let vents: Vec<String> = read_to_string("input/05").
        unwrap().lines().map(|l| l.to_string()).collect();
    let len: usize = 1000;
    let mut diagram: Vec<Vec<i32>> = vec![];
    
    for _i in 0..len {
        let line: Vec<i32> = vec![0; len];
        diagram.push(line);
    }
    
    solve(diagram.clone(), diagram.clone(), &vents);
}

fn solve(mut diagram_one: Vec<Vec<i32>>, mut diagram_two: Vec<Vec<i32>>, vents: &[String]) {
    let re = Regex::new(r"(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)").unwrap();
    
    for vent in vents {
        let caps = re.captures(vent).unwrap();
        let (mut x1, mut y1, mut x2, mut y2) = ((&caps["x1"]).parse::<usize>().unwrap(),
        (&caps["y1"]).parse::<usize>().unwrap(),
        (&caps["x2"]).parse::<usize>().unwrap(),
        (&caps["y2"]).parse::<usize>().unwrap());
        
        if x1 == x2 || y1 == y2 {
            if x1 > x2 {
                std::mem::swap(&mut x1, &mut x2);
            }
            if y1 > y2 {
                std::mem::swap(&mut y1, &mut y2);
            }
            
            for x in x1..=x2 {
                for y in y1..=y2 {
                    diagram_one[y][x] += 1;
                    diagram_two[y][x] += 1;
                }
            }
        } else {
            if x1 > x2 {
                std::mem::swap(&mut x1, &mut x2);
                std::mem::swap(&mut y1, &mut y2);
            }
            
            for x in x1..=x2 {
                let y = if y1 < y2 {y1 + (x - x1)} else {y1 - (x - x1)};
                diagram_two[y][x] += 1;
            }
        }
    }
    
    println!("First answer is {}, second answer is {}", calculate(diagram_one), calculate(diagram_two));
}

fn calculate(diagram: Vec<Vec<i32>>) -> i32 {
    let mut amount: i32 = 0;
    
    for y in 0..diagram.len() {
        for x in 0..diagram[0].len() {
            if diagram[y][x] > 1 {
                amount += 1;
            }
        }
    }
    
    amount
}