use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug, Clone)]
struct Row {
    springs: Vec<Condition>,
    groups: Vec<usize>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug)]
struct Part {
    springs: Vec<Condition>,
    arrangements: HashMap<Vec<usize>, usize>,
}

fn main() {
    let input = read_to_string("input/12_test.txt").unwrap();

    let mut rows: Vec<Row> = input
        .lines()
        .map(|line| {
            let (springs, groups) = line.split_once(' ').unwrap();
            let springs = springs
                .chars()
                .map(|c| {
                    if c == '.' {
                        Condition::Operational
                    } else if c == '#' {
                        Condition::Damaged
                    } else {
                        Condition::Unknown
                    }
                })
                .collect();
            let groups = groups
                .split(',')
                .map(|number| number.parse::<usize>().unwrap())
                .collect();
            Row { springs, groups }
        })
        .collect();

    let mut counts_sum: usize = rows
        .iter_mut()
        .enumerate()
        .map(|row| count_arrangements(&row.1.springs, &row.1.groups))
        .sum();
    println!("First answer is {counts_sum}");

    rows.iter_mut().for_each(|row| {
        let copy = row.springs.clone();
        for _i in 0..4 {
            row.springs.push(Condition::Unknown);
            row.springs.extend(copy.clone());
        }
        let copy = row.groups.clone();
        for _i in 0..4 {
            row.groups.extend(copy.clone());
        }
    });
    counts_sum = rows
        .iter_mut()
        .enumerate()
        .map(|row| count_arrangements(&row.1.springs, &row.1.groups))
        .sum();
    println!("Second answer is {counts_sum}");
}

fn count_arrangements(springs: &[Condition], groups: &[usize]) -> usize {
    if springs.iter().all(|spring| *spring == Condition::Unknown) {
        if groups.is_empty() {
            return 1;
        } else if groups.iter().sum::<usize>() + (groups.len() - 1) > springs.len() {
            return 0;
        } else if groups.len() == 1 {
            return springs.len() - groups[0] + 1;
        } else {
            return count_for_all_unknown(springs, groups);
        }
    }

    let mut amount = 0;
    if let Some(damaged) = springs
        .iter()
        .position(|spring| *spring == Condition::Damaged)
    {
        for (group_index, group) in groups.iter().enumerate() {
            for i in 0..*group {
                let left = damaged as i32 - i as i32;
                let mut right = damaged + (group - i) - 1;
                if left >= 0
                    && right < springs.len()
                    && !springs[left as usize..=right].contains(&Condition::Operational)
                    && (left == 0 || springs[left as usize - 1] != Condition::Damaged)
                    && (right == springs.len() - 1 || springs[right + 1] != Condition::Damaged)
                {
                    let left_count = if left > 0 {
                        count_arrangements(&springs[..left as usize - 1], &groups[..group_index])
                    } else {
                        count_arrangements(&springs[..left as usize], &groups[..group_index])
                    };
                    if right < springs.len() - 1 {
                        right += 1
                    };
                    let right_count = if left_count > 0 {
                        count_arrangements(&springs[right + 1..], &groups[group_index + 1..])
                    } else {
                        0
                    };
                    amount += left_count * right_count;
                }
            }
        }
    } else {
        let new_amount = count_arrangements_with_parts(&mut Row {
            springs: springs.to_vec(),
            groups: groups.to_vec(),
        });
        amount += new_amount;
    }
    amount
}

fn count_for_all_unknown(springs: &[Condition], groups: &[usize]) -> usize {
    let free_space: usize = springs.len() - groups.iter().sum::<usize>() - (groups.len() - 1);
    let baskets = groups.len() + 1;
    (factorial(free_space as u128 + baskets as u128 - 1, free_space)
        / factorial(baskets as u128 - 1, 0))
    .try_into()
    .unwrap()
}

fn factorial(number: u128, stop: usize) -> u128 {
    if number == stop as u128 {
        return 1;
    }
    match number {
        0 => 1,
        1 => 1,
        _ => factorial(number - 1, stop) * number,
    }
}

fn count_arrangements_with_parts(row: &mut Row) -> usize {
    if row
        .springs
        .iter()
        .all(|spring| *spring == Condition::Unknown)
    {
        return count_arrangements(&row.springs, &row.groups);
    }

    let mut parts: Vec<Part> = Vec::new();
    let mut start = 0;
    for i in 0..row.springs.len() {
        if row.springs[i] == Condition::Operational {
            if start < i {
                let springs = row.springs[start..i].to_vec();
                let mut arrangements = HashMap::new();
                find_arrangements(&row.groups, springs.clone(), &mut arrangements);
                parts.push(Part {
                    springs,
                    arrangements,
                });
                start = i + 1;
            } else {
                start += 1;
            }
        }
    }
    if start < row.springs.len() {
        let springs = row.springs[start..].to_vec();
        let mut arrangements = HashMap::new();
        for part in &parts {
            if part.springs == springs {
                arrangements = part.arrangements.clone();
            }
        }
        if arrangements.is_empty() {
            find_arrangements(&row.groups, springs.clone(), &mut arrangements);
        }
        parts.push(Part {
            springs,
            arrangements,
        });
    }

    add_parts(&row.groups, &parts, 0, Vec::new())
}

fn find_arrangements(
    real_groups: &Vec<usize>,
    mut row: Vec<Condition>,
    arrangements: &mut HashMap<Vec<usize>, usize>,
) {
    if let Some(spring) = row.iter().position(|spring| *spring == Condition::Unknown) {
        let mut groups = Vec::new();
        let mut group: usize = 0;
        for spring in row.iter() {
            match *spring {
                Condition::Operational => {
                    if group != 0 {
                        groups.push(group);
                        group = 0;
                    }
                }
                Condition::Damaged => group += 1,
                Condition::Unknown => break,
            }
        }

        if !valid_group(real_groups, &groups) {
            return;
        }
        row[spring] = Condition::Damaged;
        find_arrangements(real_groups, row.clone(), arrangements);
        row[spring] = Condition::Operational;
        find_arrangements(real_groups, row.clone(), arrangements);
    } else {
        let mut groups = Vec::new();
        let mut group: usize = 0;
        for spring in row.iter() {
            match *spring {
                Condition::Operational => {
                    if group != 0 {
                        groups.push(group);
                        group = 0;
                    }
                }
                Condition::Damaged => group += 1,
                Condition::Unknown => panic!(),
            }
        }

        if group != 0 {
            groups.push(group);
        }
        if !valid_group(real_groups, &groups) {
            return;
        }
        *arrangements.entry(groups).or_insert(0) += 1;
    }
}

fn valid_group(mut groups: &[usize], group: &Vec<usize>) -> bool {
    if group.is_empty() {
        return true;
    }
    while !groups.is_empty() {
        if groups.starts_with(group) {
            return true;
        }
        groups = &groups[1..];
    }
    false
}

fn add_parts(
    groups: &Vec<usize>,
    parts: &Vec<Part>,
    part_index: usize,
    arrangement: Vec<Vec<usize>>,
) -> usize {
    let mut amount = 0;
    if part_index < parts.len() {
        for arr in &parts[part_index].arrangements {
            let mut new_arrangement = arrangement.clone();
            new_arrangement.push(arr.0.clone());
            let mut final_groups: Vec<usize> = Vec::new();
            for (i, _part) in parts.iter().enumerate() {
                if i < arrangement.len() {
                    final_groups.extend(&arrangement[i]);
                }
            }
            if groups.starts_with(&final_groups) {
                amount += add_parts(groups, parts, part_index + 1, new_arrangement);
            }
        }
    } else {
        amount = 1;
        let mut final_groups: Vec<usize> = Vec::new();
        for (i, part) in parts.iter().enumerate() {
            amount *= part.arrangements.get(&arrangement[i]).unwrap();
            final_groups.extend(&arrangement[i]);
        }
        if final_groups == *groups {
            return amount;
        } else {
            return 0;
        }
    }
    amount
}
