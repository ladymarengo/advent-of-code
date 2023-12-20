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
                names.push(dest.to_string());
                modules[m].destinations.push(names.len() - 1);
            }
        });
    });

    let (mut low, mut high) = (0, 0);
    let rx_index = if let Some(pos) = names.iter().position(|name| name == "rx") {
        pos
    } else {
        0
    };
    let rx_source = if let Some(pos) = modules
        .iter()
        .enumerate()
        .find(|module| module.1.destinations.contains(&rx_index))
    {
        pos.0
    } else {
        0
    };
    let mut src_cycles: HashMap<usize, usize> = HashMap::new();
    let mut cycle = 0;

    loop {
        cycle += 1;
        let (new_low, new_high) =
            push_button(&mut modules, &names, cycle, &mut src_cycles, rx_source);
        low += new_low;
        high += new_high;
        if cycle == 1000 {
            println!("First answer is {}", low * high);
        }
        if src_cycles.len() == modules[rx_source].recent_pulses.len() {
            let min = *src_cycles.values().min().unwrap();
            for i in 2..=min / 2 {
                if src_cycles.values().all(|number| number % i == 0) {
                    src_cycles.values_mut().for_each(|value| *value /= i);
                }
            }

            let mut answer: usize = 1;
            src_cycles.values().for_each(|value| answer *= *value);
            println!("Second answer is {answer}");
            break;
        }
    }
}

fn push_button(
    modules: &mut [Module],
    names: &[String],
    cycle: usize,
    src_cycles: &mut HashMap<usize, usize>,
    rx_source: usize,
) -> (i32, i32) {
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
        if pulse.destination == rx_source
            && pulse.pulse_type == PulseType::High
            && !src_cycles.contains_key(&pulse.source)
        {
            src_cycles.insert(pulse.source, cycle);
            if src_cycles.len() == modules[pulse.destination].recent_pulses.len() {
                return (0, 0);
            }
        }
        match pulse.pulse_type {
            PulseType::High => amount.1 += 1,
            PulseType::Low => amount.0 += 1,
        }
        if pulse.destination < modules.len() {
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
