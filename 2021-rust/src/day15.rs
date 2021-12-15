use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let input = parse_input();
    print!("First answer is ");
    a_star_search(&input);
    let new: Vec<Vec<u32>> = scale_map(&input);
    print!("Second answer is ");
    a_star_search(&new);
}

fn parse_input() -> Vec<Vec<u32>> {
    read_to_string("input/15")
        .unwrap()
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u32).collect())
        .collect()
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

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy, std::default::Default)]
struct Pair(i32, i32);

fn a_star_search(grid: &Vec<Vec<u32>>) {
    let destination = Pair(grid.len() as i32 - 1, grid[0].len() as i32 - 1);
    let start = Pair(0, 0);

    let mut closed_list: Vec<Pair> = vec![];
    let mut open_list: Vec<Pair> = vec![start];
    let mut distances: HashMap<Pair, i32> = HashMap::new();
    let mut parents: HashMap<Pair, Pair> = HashMap::new();

    distances.insert(start.clone(), 0);
    parents.insert(start.clone(), start.clone());

    while !open_list.is_empty() {
        let mut new: Option<Pair> = None;
        for p in &open_list {
            if new.is_none()
                || distances[p] + heuristic(&Some(p.clone()), &destination)
                    < distances[&new.unwrap()] + heuristic(&new, &destination)
            {
                new = Some(p.clone());
            }
        }

        if new.unwrap() == destination {
            calculate_cost(grid, parents, destination);
            return;
        } else {
            for (neighbour, cost) in get_neighbours(&new.unwrap(), grid) {
                if !open_list.contains(&neighbour) && !closed_list.contains(&neighbour) {
                    open_list.push(neighbour.clone());
                    parents.insert(neighbour.clone(), new.unwrap());
                    distances.insert(neighbour.clone(), distances[&new.unwrap()] + cost as i32);
                } else {
                    if distances[&neighbour] > distances[&new.unwrap()] + cost as i32 {
                        *distances.entry(neighbour).or_default() =
                            *distances.entry(new.unwrap()).or_default() + cost as i32;
                        *parents.entry(neighbour).or_default() =
                            *parents.entry(new.unwrap()).or_default();

                        if closed_list.contains(&neighbour) {
                            closed_list.retain(|&x| x == neighbour);
                            open_list.push(neighbour.clone());
                        }
                    }
                }
            }
        }
        open_list.retain(|&x| x != new.unwrap());
        closed_list.push(new.unwrap());
    }
}

fn is_valid(x: i32, y: i32, rows: usize, columns: usize) -> bool {
    x >= 0 && y >= 0 && x < columns as i32 && y < rows as i32
}

fn heuristic(current: &Option<Pair>, destination: &Pair) -> i32 {
    match current {
        Some(p) => (destination.0 - p.0).abs() + (destination.1 - p.1).abs(),
        None => 0,
    }
}

fn get_neighbours(pair: &Pair, grid: &Vec<Vec<u32>>) -> Vec<(Pair, u32)> {
    let to_check: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut neighbours: Vec<(Pair, u32)> = vec![];
    for neighbour in to_check {
        if is_valid(
            pair.0 + neighbour.0,
            pair.1 + neighbour.1,
            grid.len(),
            grid[0].len(),
        ) {
            neighbours.push((
                Pair(pair.0 + neighbour.0, pair.1 + neighbour.1),
                grid[(pair.1 + neighbour.1) as usize][(pair.0 + neighbour.0) as usize],
            ));
        }
    }
    neighbours
}

fn calculate_cost(grid: &Vec<Vec<u32>>, parents: HashMap<Pair, Pair>, mut destination: Pair) {
    let mut cost: u32 = 0;
    while parents[&destination] != destination {
        cost += grid[destination.1 as usize][destination.0 as usize];
        destination = parents[&destination];
    }
    println!("{}", cost);
}
