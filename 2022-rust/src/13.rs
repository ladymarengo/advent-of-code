use std::cmp::Ordering;
use std::fs::read_to_string;

fn main() {
    let pairs = read_to_string("input/13.txt")
        .unwrap()
        .split("\n\n")
        .map(|i| {
            let (left, right) = i.split_once('\n').unwrap();
            (parse_packet(left), parse_packet(right))
        })
        .collect::<Vec<(Vec<Element>, Vec<Element>)>>();

    // Part one

    let indices = pairs
        .iter()
        .enumerate()
        .filter(|(_i, pair)| right_order(pair))
        .map(|(i, _pair)| i + 1)
        .collect::<Vec<usize>>();

    println!("{}", indices.iter().sum::<usize>());

    // Part two

    let mut packets: Vec<Vec<Element>> = Vec::new();
    pairs.iter().for_each(|pair| {
        packets.push(pair.0.clone());
        packets.push(pair.1.clone());
    });
    let divider_one = parse_packet("[[2]]");
    let divider_two = parse_packet("[[6]]");
    packets.push(divider_one.clone());
    packets.push(divider_two.clone());

    packets.sort_by(|a, b| {
        if right_order(&(a.clone(), b.clone())) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    let divider_one_index = packets
        .iter()
        .position(|p| packets_equal(p, &divider_one))
        .unwrap()
        + 1;
    let divider_two_index = packets
        .iter()
        .position(|p| packets_equal(p, &divider_two))
        .unwrap()
        + 1;
    println!("{}", divider_one_index * divider_two_index);
}

fn parse_packet(chars: &str) -> Vec<Element> {
    let mut packet: Vec<Element> = Vec::new();
    let mut parse_digit = false;
    chars.chars().for_each(|c| {
        if c == '[' {
            packet.push(Element {
                el_type: ElementType::Open,
                value: None,
            });
            parse_digit = false;
        } else if c == ']' {
            packet.push(Element {
                el_type: ElementType::Close,
                value: None,
            });
            parse_digit = false;
        } else if c.is_ascii_digit() {
            if parse_digit {
                let last = packet.len() - 1;
                packet[last].value = Some(
                    packet[packet.len() - 1].value.unwrap() * 10 + c.to_digit(10).unwrap() as i32,
                );
            } else {
                packet.push(Element {
                    el_type: ElementType::Integer,
                    value: Some(c.to_digit(10).unwrap() as i32),
                });
                parse_digit = true;
            };
        } else {
            parse_digit = false;
        }
    });
    packet
}

fn right_order(pair: &(Vec<Element>, Vec<Element>)) -> bool {
    let mut left_index: usize = 0;
    let mut right_index: usize = 0;
    let mut left_el: Element;
    let mut right_el: Element;
    let mut left = pair.0.clone();
    let mut right = pair.1.clone();

    while left_index < left.len() && right_index < right.len() {
        left_el = left[left_index];
        right_el = right[right_index];
        match (left_el.el_type, right_el.el_type) {
            (ElementType::Open, ElementType::Close) => return false,
            (ElementType::Close, ElementType::Open) => return true,
            (ElementType::Open, ElementType::Integer) => {
                right.insert(
                    right_index + 1,
                    Element {
                        el_type: ElementType::Close,
                        value: None,
                    },
                );
                right_index -= 1;
            }
            (ElementType::Integer, ElementType::Open) => {
                left.insert(
                    left_index + 1,
                    Element {
                        el_type: ElementType::Close,
                        value: None,
                    },
                );
                left_index -= 1;
            }
            (ElementType::Close, ElementType::Integer) => return true,
            (ElementType::Integer, ElementType::Close) => return false,
            (ElementType::Integer, ElementType::Integer) => {
                match left_el.value.unwrap().cmp(&right_el.value.unwrap()) {
                    Ordering::Less => return true,
                    Ordering::Equal => (),
                    Ordering::Greater => return false,
                }
            }
            _ => (),
        };
        left_index += 1;
        right_index += 1;
    }
    left_index == left.len()
}

fn packets_equal(first: &[Element], second: &[Element]) -> bool {
    if first.len() != second.len() {
        return false;
    }
    for (i, element) in first.iter().enumerate() {
        if *element != second[i] {
            return false;
        }
    }
    true
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Element {
    el_type: ElementType,
    value: Option<i32>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ElementType {
    Open,
    Close,
    Integer,
}
