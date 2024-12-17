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
            0 => reg_a = value.parse::<i32>().unwrap(),
            1 => reg_b = value.parse::<i32>().unwrap(),
            _ => reg_c = value.parse::<i32>().unwrap(),
        }
    });
    let program: Vec<i32> = prog_input
        .split(",")
        .map(|n| n.parse::<i32>().unwrap())
        .collect();
    run_program(&program, &mut reg_a, &mut reg_b, &mut reg_c);
}

fn run_program(program: &Vec<i32>, reg_a: &mut i32, reg_b: &mut i32, reg_c: &mut i32) {
    let mut pointer: i32 = 0;
    while pointer < program.len() as i32 {
        let instruction = program[pointer as usize];
        let literal_operand = program[pointer as usize + 1];
        let combo_operand = match literal_operand {
            4 => *reg_a,
            5 => *reg_b,
            6 => *reg_c,
            _ => literal_operand,
        };
        match instruction {
            0 => *reg_a = *reg_a / 2_i32.pow(combo_operand as u32),
            1 => *reg_b = *reg_b ^ literal_operand,
            2 => *reg_b = combo_operand % 8,
            3 => {
                if *reg_a != 0 {
                    pointer = literal_operand - 2
                }
            }
            4 => *reg_b = *reg_b ^ *reg_c,
            5 => print!("{},", combo_operand % 8),
            6 => *reg_b = *reg_a / 2_i32.pow(combo_operand as u32),
            7 => *reg_c = *reg_a / 2_i32.pow(combo_operand as u32),
            _ => println!("Something is wrong"),
        }
        pointer += 2;
    }
    println!();
}
