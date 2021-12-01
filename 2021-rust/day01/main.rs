use std::io::BufReader;
use std::io::BufRead;
use std::fs;

fn main() {
    let file = fs::File::open("input").expect("error");
    let buf = BufReader::new(file);
    let measurements: Vec<String> = buf.lines().map(|l| l.expect("error")).collect();
    println!("First answer is {}, second answer is {}", 
             count_increase(1, &measurements), count_increase(3, &measurements));
}

fn count_increase(interval: usize, measurements: &Vec<String>) -> i32 {
    let mut answer: i32 = 0;
    for i in 3..measurements.len() {
        if i > 2 && measurements[i].parse::<i32>().unwrap()
            > measurements[i-interval].parse::<i32>().unwrap() {
            answer = answer + 1;
        }
    }
    answer
}
