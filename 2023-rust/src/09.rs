use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/09.txt").unwrap();

    let values = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|number| number.parse::<i32>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut first_answer: i32 = 0;
    let mut second_answer: i32 = 0;

    values.iter().for_each(|value| {
        let mut history = value.clone();
        let mut final_value = 0;
        let mut first_values = vec![history[0]];

        while !history.iter().all(|v| *v == 0) {
            (0..history.len() - 1).for_each(|i| {
                history[i] = history[i + 1] - history[i];
            });
            final_value += history.pop().unwrap();
            first_values.push(history[0]);
        }
        first_answer += final_value;
		
        (0..first_values.len() - 1)
            .rev()
            .for_each(|i| first_values[i] -= first_values[i + 1]);
        second_answer += first_values[0];
    });

    println!("First answer is {first_answer}");
    println!("Second answer is {second_answer}");
}
