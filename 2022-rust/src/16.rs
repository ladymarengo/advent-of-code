use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/16.txt").unwrap();
    let re =
        Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.+)").unwrap();

    let mut valves: HashMap<String, Valve> = HashMap::new();
    input.lines().for_each(|l| {
        let caps = re.captures(l).unwrap();
        let name = (&caps[1]).to_string();
        let flow_rate = (&caps[2]).parse::<usize>().unwrap();
        let valves_strings = (&caps[3])
            .split(", ")
            .map(|v| v.to_string())
            .collect::<Vec<String>>();
        valves.insert(
            name,
            Valve {
                open: false,
                flow_rate,
                valves: valves_strings,
            },
        );
    });

    part_one(&mut valves);
}

fn part_one(valves: &mut HashMap<String, Valve>) {
    let mut waythrough = Waythrough {
        minutes_left: 30,
        pressure: 0,
        path: vec!["AA".to_string()],
    };
    let mut best_pressure: usize = 0;
    next_valve(valves, &mut waythrough, &mut best_pressure);
    println!("{best_pressure}");
}

fn next_valve(
    valves: &mut HashMap<String, Valve>,
    waythrough: &mut Waythrough,
    best_pressure: &mut usize,
) {
    if waythrough.minutes_left == 0 {
        if waythrough.pressure > *best_pressure {
            println!("{}", best_pressure);
            *best_pressure = waythrough.pressure;
        }
        return;
    }
    let current_name = waythrough.path.last().unwrap().clone();
    let current = valves.get(&current_name).unwrap().clone();
    let mut opened = false;

    if !current.open && current.flow_rate > 0 {
        opened = true;
        valves.get_mut(&current_name).unwrap().open = true;
        waythrough.minutes_left -= 1;
        waythrough.pressure += current.flow_rate * (waythrough.minutes_left);
        next_valve(valves, waythrough, best_pressure);
        valves.get_mut(&current_name).unwrap().open = false;
        waythrough.pressure -= current.flow_rate * waythrough.minutes_left;
        waythrough.minutes_left += 1;
    }
    current.valves.iter().for_each(|v| {
        let path_len = waythrough.path.len();
        if path_len == 1 || current.valves.len() == 1 || waythrough.path[path_len - 2] != *v {
            waythrough.minutes_left -= 1;
            waythrough.path.push(v.clone());
            next_valve(valves, waythrough, best_pressure);
            waythrough.minutes_left += 1;
            waythrough.path.pop();
        }
    });
}

#[derive(Debug, Clone)]
struct Valve {
    open: bool,
    flow_rate: usize,
    valves: Vec<String>,
}

#[derive(Debug, Clone)]
struct Waythrough {
    minutes_left: usize,
    pressure: usize,
    path: Vec<String>,
}
