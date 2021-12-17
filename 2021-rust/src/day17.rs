use std::collections::HashMap;

struct Target {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

fn main() {

    let target = Target {
        min_x: 265,
        max_x: 287,
        min_y: -103,
        max_y: -58,
    };
    
    println!("First answer is {}", (target.min_y.abs() - 1) * target.min_y.abs() / 2);
    println!("Second answer is {}", part_two(&target));
}

fn part_two(target: &Target) -> usize {
    let mut x_steps: HashMap<i32, Vec<i32>> = HashMap::new();
    
    for step in 0..=target.max_x {
        for x in 1..=target.max_x {
            let mut position: i32 = 0;
            let mut temp_x = x;
            for _i in 0..step {
                position += temp_x;
                if temp_x > 0 {
                    temp_x -= 1;
                }
            }
            if position >= target.min_x && position <= target.max_x {
                x_steps.entry(step).or_default().push(x);
            }
        }
    }
    
    let mut y_steps: HashMap<i32, Vec<i32>> = HashMap::new();
    
    for step in 0..=target.max_x {
        for y in target.min_y..=target.min_y.abs() {
            let mut position: i32 = 0;
            let mut temp_y = y;
            for _i in 0..step {
                position += temp_y;
                temp_y -= 1;
            }
            if position >= target.min_y && position <= target.max_y {
                y_steps.entry(step).or_default().push(y);
            }
        }
    }
    
    let mut both: Vec<(i32, i32)> = vec![];
    
    for (step, x_vec) in x_steps.iter() {
        if y_steps.contains_key(step) {
            for x in x_vec {
                for y in &y_steps[step] {
                    if !both.contains(&(*x, *y)) {
                        both.push((*x, *y));
                    }
                }
            }
        }
    }
    
    both.len()
}