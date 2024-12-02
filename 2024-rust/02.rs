use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/02.txt").unwrap();
    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|level| level.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let safe_reports = reports
        .iter()
        .filter(|report| is_report_safe(report))
        .count();
    println!("First answer is {safe_reports}");

    let safe_reports_with_problem_dampener = reports
        .iter()
        .filter(|report| is_report_safe_with_problem_dampener(report))
        .count();
    println!("Second answer is {safe_reports_with_problem_dampener}");
}

fn is_report_safe(report: &Vec<i32>) -> bool {
    let decreasing = report[0] > report[1];
    for i in 1..report.len() {
        let distance = (report[i] - report[i - 1]).abs();
        if (decreasing && report[i] > report[i - 1])
            || (!decreasing && report[i] < report[i - 1])
            || distance > 3
            || distance < 1
        {
            return false;
        }
    }
    true
}

fn is_report_safe_with_problem_dampener(report: &Vec<i32>) -> bool {
    if is_report_safe(report) || is_report_safe(&report[1..].to_vec()) {
        return true;
    }
    for ignore in 1..report.len() {
        let decreasing = if ignore != 1 {
            report[0] > report[1]
        } else {
            report[0] > report[2]
        };
        let mut previous = report[0];
        let mut safe = true;
        for i in 1..report.len() {
            if i != ignore {
                let current = report[i];
                let distance = (current - previous).abs();
                if (decreasing && current > previous)
                    || (!decreasing && current < previous)
                    || distance > 3
                    || distance < 1
                {
                    safe = false;
                    break;
                } else {
                    previous = current;
                }
            }
        }
        if safe {
            return true;
        }
    }
    false
}
