use std::{cmp::Ordering, collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("input/05.txt").unwrap();
    let (input_rules, input_updates) = input.split_once("\n\n").unwrap();
    let mut rules: HashMap<usize, Rule> = HashMap::new();
    input_rules.lines().for_each(|line| {
        let (left_str, right_str) = line.split_once("|").unwrap();
        let (left, right) = (
            left_str.parse::<usize>().unwrap(),
            right_str.parse::<usize>().unwrap(),
        );
        let rule = rules.entry(left).or_insert(Rule {
            before: Vec::new(),
            after: Vec::new(),
        });
        rule.after.push(right);
        let rule = rules.entry(right).or_insert(Rule {
            before: Vec::new(),
            after: Vec::new(),
        });
        rule.before.push(left);
    });

    let mut correct_updates_result: usize = 0;
    let mut incorrect_updates_result: usize = 0;
    input_updates.lines().for_each(|line| {
        let mut update: Vec<usize> = line
            .split(",")
            .map(|number| number.parse::<usize>().unwrap())
            .collect();
        if is_right_order(&update, &rules) {
            correct_updates_result += update[update.len() / 2];
        } else {
            update.sort_by(|a, b| {
                if let Some(rule) = rules.get(a) {
                    if rule.before.contains(b) {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                } else {
                    Ordering::Equal
                }
            });
            incorrect_updates_result += update[update.len() / 2];
        }
    });
    println!("First answer is {correct_updates_result}");
    println!("Second answer is {incorrect_updates_result}");
}

fn is_right_order(update: &Vec<usize>, rules: &HashMap<usize, Rule>) -> bool {
    !update.iter().enumerate().any(|(i, page)| {
        if let Some(rule) = rules.get(page) {
            if update[0..i].iter().any(|before_page| {
                !rule.before.contains(before_page) || rule.after.contains(before_page)
            }) || update[i + 1..].iter().any(|after_page| {
                rule.before.contains(after_page) || !rule.after.contains(after_page)
            }) {
                return true;
            }
        }
        false
    })
}

#[derive(Debug)]
struct Rule {
    before: Vec<usize>,
    after: Vec<usize>,
}
