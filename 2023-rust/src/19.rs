use std::{
    cmp::{max, min},
    collections::HashMap,
    fs::read_to_string,
};

use regex::Regex;

#[derive(Debug, Clone)]
struct Rule {
    category: char,
    greater: Comparison,
    number: usize,
    last: bool,
    next: String,
}

#[derive(Debug, Clone)]
enum Comparison {
    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

fn main() {
    let input = read_to_string("input/19.txt").unwrap();

    let (workflows_str, parts_str) = input.split_once("\n\n").unwrap();
    let mut workflows: HashMap<String, Vec<Rule>> = HashMap::new();
    workflows_str.lines().for_each(|line| {
        let (name, rules) = line.split_once('{').unwrap();
        let rules: Vec<Rule> = rules
            .split(',')
            .map(|rule| {
                let rule_chars = rule.chars().collect::<Vec<char>>();
                if rule_chars[rule_chars.len() - 1] == '}' {
                    Rule {
                        category: '.',
                        greater: Comparison::Greater,
                        number: 0,
                        last: true,
                        next: rule[..rule.len() - 1].to_string(),
                    }
                } else {
                    let (first, second) = rule.split_once(':').unwrap();
                    Rule {
                        category: rule_chars[0],
                        greater: if rule_chars[1] == '>' {
                            Comparison::Greater
                        } else {
                            Comparison::Less
                        },
                        number: first[2..].parse::<usize>().unwrap(),
                        last: false,
                        next: second.to_string(),
                    }
                }
            })
            .collect();
        workflows.insert(name.to_string(), rules);
    });

    let parts: Vec<Part> = parts_str
        .lines()
        .map(|line| {
            let re = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)}").unwrap();
            let caps = re.captures(line).unwrap();
            Part {
                x: caps[1].parse::<usize>().unwrap(),
                m: caps[2].parse::<usize>().unwrap(),
                a: caps[3].parse::<usize>().unwrap(),
                s: caps[4].parse::<usize>().unwrap(),
            }
        })
        .collect();

    let result: usize = parts
        .iter()
        .filter(|part| {
            let mut workflow = "in";
            while workflow != "A" && workflow != "R" {
                for rule in workflows[workflow].iter() {
                    if rule.last {
                        workflow = &rule.next;
                        break;
                    } else {
                        let matching_number = match rule.category {
                            'x' => part.x,
                            'm' => part.m,
                            'a' => part.a,
                            's' => part.s,
                            _ => 0,
                        };
                        if match rule.greater {
                            Comparison::Greater => matching_number > rule.number,
                            Comparison::Less => matching_number < rule.number,
                            _ => false,
                        } {
                            workflow = &rule.next;
                            break;
                        }
                    }
                }
            }
            workflow == "A"
        })
        .map(|part| part.x + part.m + part.a + part.s)
        .sum();

    println!("First answer is {result}");

    let mut current_rules = Vec::new();
    let result: i128 = count_accepted_combinations(&workflows, &mut current_rules, "in");
    println!("Second answer is {result}");
}

fn count_accepted_combinations(
    workflows: &HashMap<String, Vec<Rule>>,
    current_rules: &mut Vec<Rule>,
    workflow: &str,
) -> i128 {
    if workflow == "A" {
        return calculate_combinations(current_rules);
    } else if workflow == "R" {
        return 0;
    }

    let mut combinations = 0;
    let mut added_rules = 0;
    for rule in workflows[workflow].iter() {
        if rule.last {
            combinations += count_accepted_combinations(workflows, current_rules, &rule.next);
        } else {
            let new_rule = rule.clone();
            current_rules.push(new_rule);
            combinations += count_accepted_combinations(workflows, current_rules, &rule.next);
            let index = current_rules.len() - 1;
            current_rules[index].greater = match current_rules[index].greater {
                Comparison::Greater => Comparison::LessOrEqual,
                Comparison::GreaterOrEqual => Comparison::Less,
                Comparison::Less => Comparison::GreaterOrEqual,
                Comparison::LessOrEqual => Comparison::Greater,
            };
            added_rules += 1;
        }
    }
    for _i in 0..added_rules {
        current_rules.pop();
    }
    combinations
}

fn calculate_combinations(current_rules: &[Rule]) -> i128 {
    // dbg!(current_rules);
    let mut x = (1, 4000);
    let mut m = (1, 4000);
    let mut a = (1, 4000);
    let mut s = (1, 4000);
    current_rules.iter().for_each(|rule| {
        let changing_category = match rule.category {
            'x' => &mut x,
            'm' => &mut m,
            'a' => &mut a,
            _ => &mut s,
        };
        match rule.greater {
            Comparison::Greater => changing_category.0 = max(changing_category.0, rule.number + 1),
            Comparison::GreaterOrEqual => {
                changing_category.0 = max(changing_category.0, rule.number)
            }
            Comparison::Less => changing_category.1 = min(changing_category.1, rule.number - 1),
            Comparison::LessOrEqual => changing_category.1 = min(changing_category.1, rule.number),
        }
    });
    (x.1 - x.0 + 1) as i128
        * (m.1 - m.0 + 1) as i128
        * (a.1 - a.0 + 1) as i128
        * (s.1 - s.0 + 1) as i128
}
