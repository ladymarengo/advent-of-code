use std::fs::read_to_string;

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

fn main() {
    let input = read_to_string("input/06.txt").unwrap();
    let time_and_distance = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    let races = (1..time_and_distance[0].len())
        .map(|i| Race {
            time: time_and_distance[0][i].parse::<usize>().unwrap(),
            distance: time_and_distance[1][i].parse::<usize>().unwrap(),
        })
        .collect::<Vec<Race>>();

    let ways = races.iter().map(|race| {
        (1..race.time)
            .filter(|time| (race.time - time) * time > race.distance)
            .count()
    });

    let mut first_answer = 1;
    ways.for_each(|number| first_answer *= number);

    println!("First answer is {first_answer}");

    let final_race = Race {
        time: 45988373,
        distance: 295173412781210,
    };
    let second_answer = (1..final_race.time)
        .filter(|time| (final_race.time - time) * time > final_race.distance)
        .count();

    println!("Second answer is {second_answer}");
}
