use std::fs::read_to_string;

fn main() {
    let input: Vec<Vec<char>> = read_to_string("input/12.txt")
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let mut start: (i32, i32) = (0, 0);
    let mut end: (i32, i32) = (0, 0);

    let heightmap: Vec<Vec<i32>> = input
        .iter()
        .map(|l| {
            l.iter()
                .map(|c| {
                    if *c == 'S' {
                        0
                    } else if *c == 'E' {
                        'z' as i32 - 'a' as i32
                    } else {
                        *c as i32 - 'a' as i32
                    }
                })
                .collect()
        })
        .collect();
    (0..input.len()).for_each(|y| {
        (0..input[0].len()).for_each(|x| {
            if input[y][x] == 'S' {
                start = (x as i32, y as i32);
            } else if input[y][x] == 'E' {
                end = (x as i32, y as i32);
            }
        })
    });

    solve(heightmap.clone(), start, end, false);
    solve(heightmap, end, end, true);
}

fn solve(heightmap: Vec<Vec<i32>>, start: (i32, i32), end: (i32, i32), is_part_two: bool) {
    let mut parents: Vec<Vec<(i32, i32)>> = vec![vec![(0, 0); heightmap[0].len()]; heightmap.len()];
    let mut visited: Vec<Vec<bool>> = vec![vec![false; heightmap[0].len()]; heightmap.len()];
    let mut queue: Vec<(i32, i32)> = vec![start];
    let neighbours: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    while !queue.is_empty() {
        let square = queue[0];
        queue.remove(0);
        if !is_part_two && square == end {
            calculate_path(&parents, start, end);
            return;
        } else if is_part_two && heightmap[square.1 as usize][square.0 as usize] == 0 {
            calculate_path(&parents, start, square);
            return;
        }
        neighbours.iter().for_each(|n| {
            let (n_x, n_y) = (square.0 + n.0, square.1 + n.1);
            if n_x >= 0
                && n_x < heightmap[0].len() as i32
                && n_y >= 0
                && n_y < heightmap.len() as i32
                && !visited[n_y as usize][n_x as usize]
                && ((!is_part_two
                    && heightmap[n_y as usize][n_x as usize]
                        <= heightmap[square.1 as usize][square.0 as usize] + 1)
                    || (is_part_two
                        && heightmap[n_y as usize][n_x as usize]
                            >= heightmap[square.1 as usize][square.0 as usize] - 1))
            {
                queue.push((n_x, n_y));
                parents[n_y as usize][n_x as usize] = square;
                visited[n_y as usize][n_x as usize] = true;
            }
        });
    }
}

fn calculate_path(parents: &[Vec<(i32, i32)>], start: (i32, i32), end: (i32, i32)) {
    let mut steps = 0;
    let mut current = end;
    while current != start {
        current = parents[current.1 as usize][current.0 as usize];
        steps += 1;
    }
    println!("{steps}");
}
