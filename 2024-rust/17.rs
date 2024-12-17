use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/17.txt").unwrap();
    let (reg_input, prog_input) = input.split_once("\n\n").unwrap();
    let (_, prog_input) = prog_input.split_once(": ").unwrap();

    let mut reg_a = 0;
    let mut reg_b = 0;
    let mut reg_c = 0;

    reg_input.lines().enumerate().for_each(|(number, reg)| {
        let (_, value) = reg.split_once(": ").unwrap();
        match number {
            0 => reg_a = value.parse::<usize>().unwrap(),
            1 => reg_b = value.parse::<usize>().unwrap(),
            _ => reg_c = value.parse::<usize>().unwrap(),
        }
    });
    let program: Vec<usize> = prog_input
        .split(",")
        .map(|n| n.parse::<usize>().unwrap())
        .collect();

    print!("First answer is ");
    run_program(&program, &mut reg_a, &mut reg_b, &mut reg_c);

    let mut results: Vec<u64> = Vec::new();
    results.push(0);
    for value in program[0..].iter().rev() {
        let mut new_results: Vec<u64> = Vec::new();
        for x in 0..8 {
            for res in results.iter() {
                let mut b = (res + x) % 8;
                b = b ^ 5;
                let c = (res + x) / 2_u32.pow(b as u32) as u64;
                b = b ^ c;
                b = b ^ 6;
                b = b % 8;
                if b == *value as u64 {
                    new_results.push(res + x);
                }
            }
        }
        results = new_results;
        for res in results.iter_mut() {
            *res = *res * 8;
        }
    }
    println!("Second answer is {}", results.iter().min().unwrap() / 8);
}

fn run_program(
    program: &Vec<usize>,
    reg_a: &mut usize,
    reg_b: &mut usize,
    reg_c: &mut usize,
) -> bool {
    let mut pointer: usize = 0;
    while pointer < program.len() as usize {
        let instruction = program[pointer as usize];
        let literal_operand = program[pointer as usize + 1];
        let combo_operand = match literal_operand {
            4 => *reg_a,
            5 => *reg_b,
            6 => *reg_c,
            _ => literal_operand,
        };
        match instruction {
            0 => *reg_a = *reg_a / 2_usize.pow(combo_operand as u32),
            1 => *reg_b = *reg_b ^ literal_operand,
            2 => *reg_b = combo_operand % 8,
            3 => {
                if *reg_a != 0 {
                    pointer = literal_operand - 2
                }
            }
            4 => *reg_b = *reg_b ^ *reg_c,
            5 => {
                print!("{},", combo_operand % 8);
            }
            6 => *reg_b = *reg_a / 2_usize.pow(combo_operand as u32),
            7 => *reg_c = *reg_a / 2_usize.pow(combo_operand as u32),
            _ => println!("Something is wrong"),
        }
        pointer += 2;
    }
    println!("");
    true
}
