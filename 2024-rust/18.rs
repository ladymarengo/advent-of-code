use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/18.txt").unwrap();
    let bytes_order: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            let values = line.split_once(",").unwrap();
            (
                values.0.parse::<usize>().unwrap(),
                values.1.parse::<usize>().unwrap(),
            )
        })
        .collect();
    let mut grid: Vec<Vec<bool>> = vec![vec![true; 71]; 71];

    bytes_order[..1024]
        .iter()
        .for_each(|byte| grid[byte.1][byte.0] = false);

    let result = shortest_path(&grid);
    println!("First answer is {result}");

    grid = vec![vec![true; 71]; 71];
    for byte in bytes_order {
        grid[byte.1][byte.0] = false;
        if shortest_path(&grid) == 0 {
            println!("Second answer is {},{}", byte.0, byte.1);
            break;
        }
    }
}

#[derive(Debug)]
struct Node {
    row: usize,
    col: usize,
    score: usize,
    f: usize,
}

fn shortest_path(grid: &Vec<Vec<bool>>) -> usize {
    let mut open: Vec<Node> = Vec::from([Node {
        row: 0,
        col: 0,
        score: 0,
        f: 0,
    }]);
    let mut closed: Vec<Node> = Vec::new();
    let neighbors = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    while !open.is_empty() {
        open.sort_by(|a, b| b.f.cmp(&a.f));

        let node = open.pop().unwrap();
        for (n_row, n_col) in neighbors.iter() {
            let (new_row, new_col) = (n_row + node.row as i32, n_col + node.col as i32);
            if new_row >= 0
                && new_row < grid.len() as i32
                && new_col >= 0
                && new_col < grid[0].len() as i32
                && grid[new_row as usize][new_col as usize]
            {
                if new_row == grid.len() as i32 - 1 && new_col == grid[0].len() as i32 - 1 {
                    return node.score + 1;
                }
                let f = (grid.len() - new_row as usize)
                    + (grid[0].len() - new_col as usize)
                    + node.score
                    + 1;
                let new_node = Node {
                    row: new_row as usize,
                    col: new_col as usize,
                    score: node.score + 1,
                    f,
                };
                if !open.iter().any(|n| {
                    n.row == new_node.row && n.col == new_node.col && n.score <= new_node.score
                }) && !closed.iter().any(|n| {
                    n.row == new_node.row && n.col == new_node.col && n.score <= new_node.score
                }) {
                    open.push(new_node);
                }
            }
        }
        closed.push(node);
    }
    0
}
