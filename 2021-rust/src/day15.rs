use pathfinding::prelude::{absdiff, astar};
use std::fs::read_to_string;

fn main() {
    let input = parse_input();
    print!("First answer is ");
    solve(&input);
    let new: Vec<Vec<u32>> = scale_map(&input);
    print!("Second answer is ");
    solve(&new);
}

fn parse_input() -> Vec<Vec<u32>> {
    read_to_string("input/15")
        .unwrap()
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u32).collect())
        .collect()
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
    fn distance(&self, other: &Pos) -> u32 {
        (absdiff(self.0, other.0) + absdiff(self.1, other.1)) as u32
    }

    fn successors(&self, graph: &Vec<Vec<u32>>) -> Vec<(Pos, u32)> {
        let &Pos(x, y) = self;
        let mut successors: Vec<(Pos, u32)> = vec![];
        if x > 1 {
            successors.push((Pos(x - 1, y), graph[y as usize][x as usize - 1]));
        }
        if y < graph.len() as i32 - 1 {
            successors.push((Pos(x, y + 1), graph[y as usize + 1][x as usize]));
        }
        if y > 1 {
            successors.push((Pos(x, y - 1), graph[y as usize - 1][x as usize]));
        }
        if x < graph[0].len() as i32 - 1 {
            successors.push((Pos(x + 1, y), graph[y as usize][x as usize + 1]));
        }
        successors
    }
}

fn solve(graph: &Vec<Vec<u32>>) {
    let goal: Pos = Pos((graph.len() - 1) as i32, (graph[0].len() - 1) as i32);
    let result = astar(
        &Pos(0, 0),
        |p| p.successors(graph),
        |p| p.distance(&goal) / 3,
        |p| *p == goal,
    );
    println!("{}", result.expect("no path found").1);
}

fn scale_map(graph: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut new: Vec<Vec<u32>> = vec![];
    for line in graph {
        let mut row: Vec<u32> = vec![];
        for index in 0..5 {
            for number in line {
                let mut new_value = *number;
                for _i in 0..index {
                    new_value = if new_value == 9 { 1 } else { new_value + 1 };
                }
                row.push(new_value);
            }
        }
        new.push(row);
    }
    let first_lines = new.clone();
    for index in 1..5 {
        for line in first_lines.clone() {
            let mut row: Vec<u32> = vec![];
            for number in &line {
                let mut new_value = *number;
                for _i in 0..index {
                    new_value = if new_value == 9 { 1 } else { new_value + 1 };
                }
                row.push(new_value);
            }
            new.push(row);
        }
    }
    new
}
