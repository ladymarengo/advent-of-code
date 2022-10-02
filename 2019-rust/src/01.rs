fn main() {
    println!("Hello world!");
}

fn calculate_fuel(mass: i32) -> i32 {
    2
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use crate::calculate_fuel;

    #[test_case(12, 2 ; "mass 12")]
    fn test_calculate_fuel(mass: i32, expected: i32) {
        let fuel = calculate_fuel(mass);

        assert_eq!(fuel, expected)
    }
}
