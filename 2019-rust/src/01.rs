use std::fs::read_to_string;

fn main() {
    let input: Vec<i32> = read_to_string("input/01")
        .unwrap()
        .lines()
        .map(|l| l.parse().unwrap())
		.collect();
	
	let sum: i32 = input.into_iter().map(calculate_fuel).sum();
	println!("{sum}");
}

fn calculate_fuel(mass: i32) -> i32 {
    mass / 3 - 2
}

fn calculate_fuel_part2(mass: i32) -> i32 {
    2
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use crate::{calculate_fuel, calculate_fuel_part2};

    #[test_case(12, 2 ; "mass 12")]
    #[test_case(14, 2 ; "mass 14")]
    #[test_case(1969, 654 ; "mass 1969")]
    #[test_case(100756, 33583 ; "mass 100756")]
    fn test_calculate_fuel(mass: i32, expected: i32) {
        let fuel = calculate_fuel(mass);

        assert_eq!(fuel, expected)
    }

	#[test_case(12, 2 ; "mass 12")]
    // #[test_case(14, 2 ; "mass 14")]
    // #[test_case(1969, 654 ; "mass 1969")]
    // #[test_case(100756, 33583 ; "mass 100756")]
    fn test_calculate_fuel_part2(mass: i32, expected: i32) {
        let fuel = calculate_fuel_part2(mass);

        assert_eq!(fuel, expected)
    }
}
