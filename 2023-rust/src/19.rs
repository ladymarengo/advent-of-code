use std::{collections::HashMap, fs::read_to_string};

use regex::Regex;

#[derive(Debug)]
struct Rule {
    category: char,
    greater: bool,
    number: usize,
    last: bool,
    next: String,
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
                        greater: false,
                        number: 0,
                        last: true,
                        next: rule[..rule.len() - 1].to_string(),
                    }
                } else {
                    let (first, second) = rule.split_once(':').unwrap();
                    Rule {
                        category: rule_chars[0],
                        greater: rule_chars[1] == '>',
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
                            true => matching_number > rule.number,
                            false => matching_number < rule.number,
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
}
