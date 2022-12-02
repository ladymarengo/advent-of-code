use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/02.txt").unwrap();
    let scores = input.lines().map(|l| {
        let t = l.split_once(' ').unwrap();
        let enemy = t.0.chars().next().unwrap();
        let you = t.1.chars().next().unwrap();
        let mut score_one = 0;
        let mut score_two = 0;
        match you {
            'X' => {
                score_one += 1;
                match enemy {
                    'A' => {
                        score_two += 3;
                        score_one += 3
                    }
                    'B' => score_two += 1,
                    'C' => {
                        score_two += 2;
                        score_one += 6
                    }
                    _ => (),
                }
            }
            'Y' => {
                score_one += 2;
                score_two += 3;
                match enemy {
                    'A' => {
                        score_two += 1;
                        score_one += 6
                    }
                    'B' => {
                        score_two += 2;
                        score_one += 3
                    }
                    'C' => score_two += 3,
                    _ => (),
                }
            }
            'Z' => {
                score_one += 3;
                score_two += 6;
                match enemy {
                    'A' => score_two += 2,
                    'B' => {
                        score_two += 3;
                        score_one += 6
                    }
                    'C' => {
                        score_two += 1;
                        score_one += 3
                    }
                    _ => (),
                }
            }
            _ => (),
        };
        (score_one, score_two)
    });
    println!(
        "{}, {}",
        scores.clone().map(|s| s.0).sum::<i32>(),
        scores.map(|s| s.1).sum::<i32>()
    );
}
