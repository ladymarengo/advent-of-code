use std::fs::read_to_string;
extern crate hex;
use to_binary::BinaryString;

fn main() {
    let input: String = read_to_string("input/16").unwrap();
    let answers = solve(&input);
    println!("First answer is {}", answers.0);
    println!("Second answer is {}", answers.1);
}

fn solve(transmission: &str) -> (usize, usize) {
    let transmission = BinaryString::from_hex(transmission).unwrap().to_string();
    let mut versions_sum: usize = 0;
    let (_, value) = decode(&transmission, &mut versions_sum);
    (versions_sum, value)
}

fn decode(packet: &str, versions_sum: &mut usize) -> (usize, usize) {
    *versions_sum += usize::from_str_radix(&packet[0..3], 2).unwrap();

    match usize::from_str_radix(&packet[3..6], 2).unwrap() {
        4 => {
            let (index, value) = parse_literal(&packet[6..]);
            (6 + index, value)
        }
        operator => match &packet.chars().collect::<Vec<char>>()[6] {
            '0' => {
                let (index, value) = decode_subpackets(
                    usize::from_str_radix(&packet[7..22], 2).unwrap(),
                    0,
                    &packet[22..],
                    versions_sum,
                    operator,
                );
                (22 + index, value)
            }
            '1' => {
                let (index, value) = decode_subpackets(
                    0,
                    usize::from_str_radix(&packet[7..18], 2).unwrap(),
                    &packet[18..],
                    versions_sum,
                    operator,
                );
                (18 + index, value)
            }
            _ => (0, 0),
        },
    }
}

fn decode_subpackets(
    length: usize,
    number: usize,
    packets: &str,
    versions_sum: &mut usize,
    operator: usize,
) -> (usize, usize) {
    let mut index: usize = 0;
    let mut values: Vec<usize> = vec![];
    let mut value: usize = 0;

    if length > 0 {
        while index < length {
            let (temp_i, temp_v) = decode(&packets[index..], versions_sum);
            index += temp_i;
            values.push(temp_v);
        }
    } else {
        for _i in 0..number {
            let (temp_i, temp_v) = decode(&packets[index..], versions_sum);
            index += temp_i;
            values.push(temp_v);
        }
    }

    match operator {
        0 => value = values.iter().sum(),
        1 => value = values.iter().product(),
        2 => value = *values.iter().min().unwrap(),
        3 => value = *values.iter().max().unwrap(),
        5 => value = if values[0] > values[1] { 1 } else { 0 },
        6 => value = if values[0] < values[1] { 1 } else { 0 },
        7 => value = if values[0] == values[1] { 1 } else { 0 },
        _ => (),
    }
    (index, value)
}

fn parse_literal(literal: &str) -> (usize, usize) {
    let mut index: usize = 0;
    let mut value = String::new();

    while literal.chars().collect::<Vec<char>>()[index] == '1' {
        value.push_str(&literal[index + 1..index + 5]);
        index += 5;
    }
    value.push_str(&literal[index + 1..index + 5]);
    index += 5;
    (index, usize::from_str_radix(&value, 2).unwrap())
}
