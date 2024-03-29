use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/13.txt").unwrap();

    let patterns = input.split("\n\n").map(|part| {
        let mut rows = Vec::new();
        let mut columns = Vec::new();
        part.lines().for_each(|line| {
            rows.push(line.chars().collect::<Vec<char>>());
            line.chars().enumerate().for_each(|(i, c)| {
                if columns.len() == i {
                    columns.push(vec![]);
                }
                columns[i].push(c);
            });
        });
        (rows, columns)
    });

    let mut sum = 0;
    patterns.clone().enumerate().for_each(|(p, pattern)| {
        println!("Pattern {p}");
        for (i, row) in pattern.0.iter().enumerate() {
            if i < pattern.0.len() - 1 && *row == pattern.0[i + 1] {
                let mut first = pattern.0[..=i].to_vec();
                let second = pattern.0[i + 1..].to_vec();
                first.reverse();
                if (first.len() < second.len() && second.starts_with(&first))
                    || first.starts_with(&second)
                {
                    println!("Add {}", (i + 1) * 100);
                    sum += (i + 1) * 100;
                    break;
                }
            }
        }
        for (i, column) in pattern.1.iter().enumerate() {
            if i < pattern.1.len() - 1 && *column == pattern.1[i + 1] {
                let mut first = pattern.1[..=i].to_vec();
                let second = pattern.1[i + 1..].to_vec();
                first.reverse();
                if (first.len() < second.len() && second.starts_with(&first))
                    || first.starts_with(&second)
                {
                    println!("Add {}", i + 1);
                    sum += i + 1;
                    break;
                }
            }
        }
    });
    println!("First answer is {sum}");

    sum = 0;
    patterns.enumerate().for_each(|(p, pattern)| {
        println!("Pattern {p}");
        for (i, _row) in pattern.0.iter().enumerate() {
            if i < pattern.0.len() - 1 {
                let mut difference = 0;
                let mut first = i as i32;
                let mut second = i + 1;

                while first >= 0 && second < pattern.0.len() {
                    difference += pattern.0[first as usize]
                        .iter()
                        .enumerate()
                        .filter(|(index, c)| **c != pattern.0[second][*index])
                        .count();
                    if difference > 1 {
                        break;
                    }
                    first -= 1;
                    second += 1;
                }

                if difference == 1 {
                    sum += (i + 1) * 100;
                    break;
                }
            }
        }
        for (i, _column) in pattern.1.iter().enumerate() {
            if i < pattern.1.len() - 1 {
                let mut difference = 0;
                let mut first = i as i32;
                let mut second = i + 1;

                while first >= 0 && second < pattern.1.len() {
                    difference += pattern.1[first as usize]
                        .iter()
                        .enumerate()
                        .filter(|(index, c)| **c != pattern.1[second][*index])
                        .count();
                    if difference > 1 {
                        break;
                    }
                    first -= 1;
                    second += 1;
                }

                if difference == 1 {
                    sum += i + 1;
                    break;
                }
            }
        }
    });
    println!("Second answer is {sum}");
}
