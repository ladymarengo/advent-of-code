use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/04.txt").unwrap();
    let pairs = input.lines().map(|l| {
        let elves = l.split_once(',').unwrap();
        let first = elves.0.split_once('-').unwrap();
        let second = elves.1.split_once('-').unwrap();
        (
            (
                first.0.parse::<usize>().unwrap(),
                first.1.parse::<usize>().unwrap(),
            ),
            (
                second.0.parse::<usize>().unwrap(),
                second.1.parse::<usize>().unwrap(),
            ),
        )
    });
    let overlaps_one = pairs
        .clone()
        .filter(|((s1, e1), (s2, e2))| (s1 <= s2 && e1 >= e2) || (s1 >= s2 && e1 <= e2))
        .count();
    let overlaps_two = pairs
        .filter(|((s1, e1), (s2, e2))| s2 <= e1 && s1 <= e2)
        .count();
    println!("{overlaps_one}, {overlaps_two}");
}
