use std::fs::read_to_string;

struct Common(i32, i32);

fn count_common(arr: &Vec<String>, i: usize) -> Common {
    let mut common = Common(0, 0);
    for number in arr {
        match number.as_bytes()[i] {
            48 => common.0 = common.0 + 1,
            49 => common.1 = common.1 + 1,
            _ => (),
        }
    }
    common
}

fn calculate_part_two(mut arr: Vec<String>, len: usize, rating: &str) -> isize {
    for i in 0..len {
        if arr.len() == 1 {
            break;
        }
        let mut new_arr = Vec::new();
        let common = count_common(&arr, i);

        for j in 0..arr.len() {
            match rating {
                "oxygen" => {
                    if (common.0 > common.1 && arr[j].as_bytes()[i] == 48) ||
                        (common.1 >= common.0 && arr[j].as_bytes()[i] == 49) {
                        new_arr.push(arr[j].clone());
                    }
                },
                "co2" => {
                    if (common.0 <= common.1 && arr[j].as_bytes()[i] == 48) ||
                        (common.1 < common.0 && arr[j].as_bytes()[i] == 49) {
                        new_arr.push(arr[j].clone());
                    }
                },
                _ => (),
            }
        }
        arr = new_arr;
    }
    isize::from_str_radix(&(arr[0]), 2).unwrap()
}

fn main() {
    let numbers: Vec<String> = read_to_string("input/03").
        unwrap().lines().map(|l| l.parse().unwrap()).collect();
    let len = numbers[0].len();
    let mut gamma = String::new();
    let mut epsilon = String::new();
    
    for i in 0..len {
        let common = count_common(&numbers, i);
        if common.0 > common.1 {
            gamma.push('0');
            epsilon.push('1');
        } else {
            gamma.push('1');
            epsilon.push('0');
        }
    }
    
    println!("First answer is {}", isize::from_str_radix(&gamma, 2).unwrap() *
             isize::from_str_radix(&epsilon, 2).unwrap());
    
    let oxygen = numbers.clone();
    let co2 = numbers.clone();

    println!("Second answer is {}", calculate_part_two(oxygen, len, "oxygen") *
             calculate_part_two(co2, len, "co2"));
}
