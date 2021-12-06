use std::fs::read_to_string;

fn main() {
    let input: Vec<usize> = read_to_string("input/06").
        unwrap().split(',').map(|l| l.parse().unwrap()).collect();
    let mut fish: Vec<usize> = vec![0; 10];
    for number in input {
        fish[number] += 1;
    }
    println!("First answer is {}, second answer is {}", solve(fish.clone(), 80), solve(fish.clone(), 256));
}

fn solve(mut fish: Vec<usize>, days: usize) -> usize {
    let mut sum: usize = 0;
    for _day in 0..days {
        let mut new: Vec<usize> = vec![0; 9];
        for (i, amount) in fish.iter().enumerate() {
            if i == 0 {
                new[8] += *amount;
                new[6] += *amount;
            } else {
                new[i - 1] += *amount;
            }
        }
        fish = new;
    }
    for amount in fish {
        sum += amount;
    }
    sum
}