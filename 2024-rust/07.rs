use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/07.txt").unwrap();

    let mut result_part_one: usize = 0;
    let mut result_part_two: usize = 0;
    input.lines().for_each(|line| {
        let (test_value, numbers) = line.split_once(": ").unwrap();
        let test_value = test_value.parse::<usize>().unwrap();
        let numbers: Vec<usize> = numbers
            .split(" ")
            .map(|number| number.parse::<usize>().unwrap())
            .collect();
        if is_true_equation(test_value, numbers[0], &numbers[1..], false) {
            result_part_one += test_value;
            result_part_two += test_value;
        } else if is_true_equation(test_value, numbers[0], &numbers[1..], true) {
            result_part_two += test_value;
        }
    });

    println!("First answer is {result_part_one}");
    println!("Second answer is {result_part_two}");
}

fn is_true_equation(
    final_value: usize,
    current_value: usize,
    remaining_values: &[usize],
    part_two: bool,
) -> bool {
    if current_value == final_value && remaining_values.is_empty() {
        return true;
    } else if current_value > final_value || remaining_values.is_empty() {
        return false;
    } else {
        if part_two {
            let new_value = format!(
                "{}{}",
                current_value.to_string(),
                remaining_values[0].to_string()
            )
            .parse::<usize>()
            .unwrap();
            if is_true_equation(final_value, new_value, &remaining_values[1..], part_two) {
                return true;
            }
        }
        return is_true_equation(
            final_value,
            current_value + remaining_values[0],
            &remaining_values[1..],
            part_two,
        ) || is_true_equation(
            final_value,
            current_value * remaining_values[0],
            &remaining_values[1..],
            part_two,
        );
    }
}
