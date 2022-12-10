use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/10.txt").unwrap();

    let mut signal_strength = 0;
    let mut used_cycles: Vec<i32> = Vec::new();
    let mut register = 1;
    let mut cycle: i32 = 0;
    let mut draw: i32 = 0;

    input.lines().for_each(|l| {
        let mut value = 0;
        if l.eq("noop") {
            cycle += 1;
        } else {
            let (_instruction, value_str) = l.split_once(' ').unwrap();
            value = value_str.parse().unwrap();
            cycle += 2;
        }
        let interesting_signal = (cycle - 20) / 2 * 2;
        if !used_cycles.contains(&interesting_signal)
            && cycle >= 20
            && cycle <= 221
            && interesting_signal % 40 == 0
        {
            used_cycles.push(interesting_signal);
            signal_strength += (interesting_signal + 20) * register;
        }
        while draw < cycle {
            print!(
                "{}",
                if (register - 1..=register + 1).contains(&(draw % 40)) {
                    '#'
                } else {
                    '.'
                }
            );
            draw += 1;
            if draw % 40 == 0 {
                println!();
            }
        }
        register += value;
    });
    dbg!(signal_strength);
}
