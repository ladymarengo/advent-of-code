use std::collections::HashMap;

fn main() {
    let input: Vec<usize> = vec![1750884, 193, 866395, 7, 1158, 31, 35216, 0];

    let mut map: HashMap<(usize, usize), usize> = HashMap::new();
    println!("First answer is {}", count_stones(&input, 26, &mut map));
    map.clear();
    println!("Second answer is {}", count_stones(&input, 76, &mut map));
}

fn count_stones(
    input: &Vec<usize>,
    blinks: usize,
    map: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if blinks == 0 {
        return 1;
    }
    let mut result: usize = 0;
    input.iter().for_each(|&stone| {
        if map.contains_key(&(stone, blinks)) {
            let amount = map.get(&(stone, blinks)).unwrap();
            result += amount;
        } else {
            let mut new_input: Vec<usize> = Vec::new();
            if stone == 0 {
                new_input.push(1);
            } else if stone.to_string().len() % 2 == 0 {
                let new = stone.to_string();
                let new_len = new.len() / 2;
                new_input.push(new[0..new_len].parse::<usize>().unwrap());
                new_input.push(new[new_len..].parse::<usize>().unwrap());
            } else {
                new_input.push(stone * 2024);
            }
            let amount = count_stones(&new_input, blinks - 1, map);
            result += amount;
            map.insert((stone, blinks), amount);
        }
    });
    result
}
