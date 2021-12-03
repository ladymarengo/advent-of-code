use std::fs::read_to_string;

struct Common(i32, i32);

fn main() {
    let numbers: Vec<String> = read_to_string("input/03").
        unwrap().lines().map(|l| l.parse().unwrap()).collect();
    let len = numbers[0].len();
    let mut gamma = String::new();
    let mut epsilon = String::new();
    
    for i in 0..len {
        let mut common = Common(0, 0);
        for number in &numbers {
            match number.as_bytes()[i] {
                48 => common.0 = common.0 + 1,
                49 => common.1 = common.1 + 1,
                _ => (),
            }
        }
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
    
    let mut oxygen = numbers.clone();
    let mut co2 = numbers.clone();
    
    for i in 0..len {
        if oxygen.len() == 1 {
            break;
        }
        let mut new_oxygen = Vec::new();
        let mut common = Common(0, 0);
        
        for number in &oxygen {
            match number.as_bytes()[i] {
                48 => common.0 = common.0 + 1,
                49 => common.1 = common.1 + 1,
                _ => (),
            }
        }
        
        for j in 0..oxygen.len() {
            if (common.0 > common.1 && oxygen[j].as_bytes()[i] == 48) ||
                (common.1 >= common.0 && oxygen[j].as_bytes()[i] == 49) {
                new_oxygen.push(oxygen[j].clone());
            }
        }
        oxygen = new_oxygen;
    }
    
    for i in 0..len {
        if co2.len() == 1 {
            break;
        }
        let mut new_co2 = Vec::new();
        let mut common = Common(0, 0);
        
        for number in &co2 {
            match number.as_bytes()[i] {
                48 => common.0 = common.0 + 1,
                49 => common.1 = common.1 + 1,
                _ => (),
            }
        }
        
        for j in 0..co2.len() {
            if (common.0 <= common.1 && co2[j].as_bytes()[i] == 48) ||
                (common.1 < common.0 && co2[j].as_bytes()[i] == 49) {
                new_co2.push(co2[j].clone());
            }
        }
        co2 = new_co2;
    }

    println!("Second answer is {}", isize::from_str_radix(&(oxygen[0]), 2).unwrap() *
             isize::from_str_radix(&(co2[0]), 2).unwrap());
}
