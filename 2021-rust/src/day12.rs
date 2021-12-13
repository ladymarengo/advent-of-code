use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let input = parse_input();
    println!(
        "First answer is {}, second answer is {}",
        solve(input.clone(), 1),
        solve(input, 2)
    );
}

fn parse_input() -> HashMap<String, Vec<String>> {
    let input: Vec<Vec<String>> = read_to_string("input/12")
        .unwrap()
        .lines()
        .map(|l| l.to_string().split('-').map(|i| i.to_string()).collect())
        .collect();
    let mut caves: HashMap<String, Vec<String>> = HashMap::new();

    for line in input {
        caves
            .entry(line[0].clone())
            .or_default()
            .push(line[1].clone());
        caves
            .entry(line[1].clone())
            .or_default()
            .push(line[0].clone());
    }
    caves
}

fn solve(caves: HashMap<String, Vec<String>>, part: usize) -> usize {
    let path: Vec<String> = vec!["start".to_string()];
    let paths: usize = find_path(&caves, path, &"start".to_string(), part);
    paths
}

fn find_path(
    caves: &HashMap<String, Vec<String>>,
    path: Vec<String>,
    cave: &str,
    part: usize,
) -> usize {
    let mut total_paths: usize = 0;

    for way in &caves[cave] {
        let mut new_path = path.clone();
        new_path.push(way.clone());
        if check_path(&new_path, part) {
            if *new_path.last().unwrap() == "end" {
                total_paths += 1;
            } else {
                total_paths += find_path(caves, new_path, way, part);
            }
        }
    }
    total_paths
}

fn check_path(path: &[String], part: usize) -> bool {
    if part == 1 {
        for cave in path {
            if cave.chars().all(char::is_lowercase)
                && path.iter().filter(|&n| n == cave).count() > 1
            {
                return false;
            }
        }
        true
    } else {
        let mut small_visited_twice: usize = 0;
        let mut checked: Vec<String> = vec![];

        for cave in path {
            if !checked.contains(cave)
                && cave.chars().all(char::is_lowercase)
                && path.iter().filter(|&n| n == cave).count() > 1
            {
                checked.push(cave.clone());
                if path.iter().filter(|&n| n == cave).count() == 2
                    && cave != &"start".to_string()
                    && cave != &"end".to_string()
                {
                    small_visited_twice += 1;
                } else {
                    return false;
                }
            }
        }
        if small_visited_twice > 1 {
            return false;
        }
        true
    }
}
