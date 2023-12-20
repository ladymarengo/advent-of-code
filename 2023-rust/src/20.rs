use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
};

#[derive(Debug, Clone)]
struct Module {
    module_type: ModuleType,
    active: bool,
    recent_pulses: HashMap<usize, PulseType>,
    destinations: Vec<usize>,
    dest_source: String,
}

#[derive(Debug, Clone)]
enum ModuleType {
    Broadcast,
    FlipFlop,
    Conjunction,
}

#[derive(Debug, PartialEq, Clone)]
enum PulseType {
    High,
    Low,
}

#[derive(Debug)]
struct Pulse {
    source: usize,
    destination: usize,
    pulse_type: PulseType,
}

fn main() {
    let input = read_to_string("input/20.txt").unwrap();

    let mut modules: Vec<Module> = Vec::new();
    let mut names: Vec<String> = Vec::new();
    input.lines().for_each(|line| {
        let (module, destinations) = line.split_once(" -> ").unwrap();
        names.push(module[1..].to_string());
        modules.push(Module {
            module_type: match &module[..1] {
                "%" => ModuleType::FlipFlop,
                "&" => ModuleType::Conjunction,
                _ => ModuleType::Broadcast,
            },
            active: false,
            recent_pulses: HashMap::new(),
            destinations: vec![],
            dest_source: destinations.to_string(),
        });
    });

    (0..modules.len()).for_each(|m| {
        let dest_source = modules[m].dest_source.clone();
        dest_source.split(", ").for_each(|dest| {
            if let Some(pos) = names.iter().position(|name| name == dest) {
                modules[m].destinations.push(pos);
                modules[pos]
                    .recent_pulses
                    .entry(m)
                    .or_insert(PulseType::Low);
            } else {
                modules[m].destinations.push(names.len() + 10);
            }
        });
    });

    let (mut low, mut high) = (0, 0);
    for _i in 0..1000 {
        let (new_low, new_high) = push_button(&mut modules, &names);
        low += new_low;
        high += new_high;
    }
    println!("First answer is {}", low * high);
}

fn push_button(modules: &mut [Module], names: &[String]) -> (i32, i32) {
    let mut amount = (1, 0);
    let mut open_list: VecDeque<Pulse> = VecDeque::new();
    let broadcaster_index = names.iter().position(|name| name == "roadcaster").unwrap();
    modules[broadcaster_index]
        .destinations
        .iter()
        .for_each(|dest| {
            open_list.push_back(Pulse {
                source: broadcaster_index,
                destination: *dest,
                pulse_type: PulseType::Low,
            })
        });

    while let Some(pulse) = open_list.pop_front() {
        match pulse.pulse_type {
            PulseType::High => amount.1 += 1,
            PulseType::Low => amount.0 += 1,
        }
        if pulse.destination < names.len() {
            let module = &mut modules[pulse.destination];
            let mut next_type = PulseType::Low;
            let mut send_pulse = true;
            match module.module_type {
                ModuleType::FlipFlop => {
                    if pulse.pulse_type == PulseType::Low {
                        if module.active {
                            module.active = false;
                            next_type = PulseType::Low;
                        } else {
                            module.active = true;
                            next_type = PulseType::High;
                        }
                    } else {
                        send_pulse = false;
                    }
                }
                ModuleType::Conjunction => {
                    *module.recent_pulses.get_mut(&pulse.source).unwrap() = pulse.pulse_type;
                    if module.recent_pulses.iter().any(|p| *p.1 == PulseType::Low) {
                        next_type = PulseType::High;
                    } else {
                        next_type = PulseType::Low;
                    }
                }
                _ => (),
            }
            if send_pulse {
                module.destinations.iter().for_each(|dest| {
                    open_list.push_back(Pulse {
                        source: pulse.destination,
                        destination: *dest,
                        pulse_type: next_type.clone(),
                    })
                });
            }
        }
    }
    amount
}
