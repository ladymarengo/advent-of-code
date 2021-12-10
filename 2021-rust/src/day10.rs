use std::fs::read_to_string;

fn main() {
    let lines: Vec<String> = read_to_string("input/10")
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect();
    let answers = solve(&lines);
    println!(
        "First answer is {}, second answer is {}",
        answers.0, answers.1
    );
}

fn solve(lines: &[String]) -> (usize, usize) {
    let mut score_one: usize = 0;
    let mut scores_two: Vec<usize> = vec![];
    for line in lines {
        let mut opened: String = String::new();
        let mut corrupted = false;
        for symbol in line.chars() {
            if "({[<".contains(symbol) {
                opened.push(symbol);
            } else {
                let last: char = opened.as_bytes()[opened.len() - 1] as char;
                match symbol {
                    ')' if last != '(' => {
                        score_one += 3;
                        corrupted = true;
                        break;
                    }
                    ']' if last != '[' => {
                        score_one += 57;
                        corrupted = true;
                        break;
                    }
                    '}' if last != '{' => {
                        score_one += 1197;
                        corrupted = true;
                        break;
                    }
                    '>' if last != '<' => {
                        score_one += 25137;
                        corrupted = true;
                        break;
                    }
                    ')' if last == '(' => {
                        opened.pop();
                    }
                    ']' if last == '[' => {
                        opened.pop();
                    }
                    '}' if last == '{' => {
                        opened.pop();
                    }
                    '>' if last == '<' => {
                        opened.pop();
                    }
                    _ => (),
                }
            }
        }
        if !corrupted {
            scores_two.push(count_score_part_two(opened));
        }
    }
    scores_two.sort_unstable();
    (score_one, scores_two[scores_two.len() / 2])
}

fn count_score_part_two(opened: String) -> usize {
    let mut score: usize = 0;
    for i in (0..opened.len()).rev() {
        score *= 5;
        match opened.as_bytes()[i] as char {
            '(' => score += 1,
            '[' => score += 2,
            '{' => score += 3,
            '<' => score += 4,
            _ => (),
        }
    }
    score
}
