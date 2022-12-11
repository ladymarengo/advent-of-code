use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/11.txt")
        .unwrap()
        .split("\n\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let mut monkeys_part_one: Vec<Monkey> = input.iter().map(|m| new_monkey(m)).collect();
    let mut monkeys_part_two = monkeys_part_one.clone();
    let mut division = 1;
    monkeys_part_one.iter().for_each(|m| division *= m.test);

    // Part one

    let mut rounds = 20;
    (0..rounds).for_each(|_i| {
        (0..monkeys_part_one.len()).for_each(|m| monkey_turn(&mut monkeys_part_one, m, 0))
    });
    let mut inspected = monkeys_part_one
        .iter()
        .map(|m| m.inspected)
        .collect::<Vec<usize>>();
    inspected.sort();
    inspected.reverse();
    println!("{}", inspected[0] * inspected[1]);

    // Part two

    rounds = 10000;
    (0..rounds).for_each(|_i| {
        (0..monkeys_part_two.len()).for_each(|m| monkey_turn(&mut monkeys_part_two, m, division))
    });
    inspected = monkeys_part_two
        .iter()
        .map(|m| m.inspected)
        .collect::<Vec<usize>>();
    inspected.sort();
    inspected.reverse();
    println!("{}", inspected[0] * inspected[1]);
}

fn new_monkey(input: &str) -> Monkey {
    let input = input.lines().collect::<Vec<&str>>();
    let (_s, items) = input[1].split_once(": ").unwrap();
    let items = items
        .split(", ")
        .map(|i| i.parse().unwrap())
        .collect::<Vec<usize>>();

    let (_o, operation) = input[2].split_once("old ").unwrap();
    let operation = operation.split_once(' ').unwrap();
    let operation_sign = if operation.0 == "*" {
        OperationSign::Multiply
    } else {
        OperationSign::Increase
    };
    let mut operation_self = false;
    let mut operation_value: usize = 0;
    if operation.1 == "old" {
        operation_self = true;
    } else {
        operation_value = operation.1.parse::<usize>().unwrap();
    }

    let (_t, test) = input[3].split_once("by ").unwrap();
    let test = test.parse::<usize>().unwrap();

    let (_t, test_true) = input[4].split_once("monkey ").unwrap();
    let test_true = test_true.parse::<usize>().unwrap();

    let (_t, test_false) = input[5].split_once("monkey ").unwrap();
    let test_false = test_false.parse::<usize>().unwrap();

    Monkey {
        items,
        inspected: 0,
        operation_sign,
        operation_self,
        operation_value,
        test,
        test_true,
        test_false,
    }
}

fn monkey_turn(monkeys: &mut [Monkey], current: usize, division: usize) {
    monkeys[current].items.clone().iter().for_each(|i| {
        monkeys[current].items.remove(0);
        let mut item = *i;
        let value = if monkeys[current].operation_self {
            item
        } else {
            monkeys[current].operation_value
        };
        match monkeys[current].operation_sign {
            OperationSign::Multiply => item *= value,
            OperationSign::Increase => item += value,
        };
        if division == 0 {
            item /= 3;
        } else {
            item %= division;
        }
        let next_monkey = if item % monkeys[current].test == 0 {
            monkeys[current].test_true
        } else {
            monkeys[current].test_false
        };
        monkeys[next_monkey].items.push(item);
        monkeys[current].inspected += 1;
    })
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<usize>,
    inspected: usize,
    operation_sign: OperationSign,
    operation_self: bool,
    operation_value: usize,
    test: usize,
    test_true: usize,
    test_false: usize,
}

#[derive(Debug, Clone)]
enum OperationSign {
    Multiply,
    Increase,
}
