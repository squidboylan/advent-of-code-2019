use std::fs;
use std::env;
use std::cmp::max;
use std::iter::successors;

fn load_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Reading file failed")
}

fn parse_input<'a>(data: &'a str) -> impl Iterator<Item = i64> + 'a + Clone {
    data.lines().map(|x| x.parse::<i64>().expect("Failed to convert string to i64"))
}

fn calculate_fuel(mass: i64) -> i64 {
    max(0, mass / 3 - 2)
}

// Return the output of calculate_fuel, but as an Option, Some(x) if x > 0 else None
// This makes doing calculate_fuel_fuel easier because it will stop on None
fn calculate_fuel_fuel_helper(mass: i64) -> Option<i64> {
    let val = calculate_fuel(mass);
    if val > 0 {
        Some(val)
    } else {
        None
    }
}

// This calculates the fuel needed for the mass + the fuel needed for the fuel as is required for
// part 2
fn calculate_fuel_fuel(mass: i64) -> i64 {
    successors(calculate_fuel_fuel_helper(mass), |&x| calculate_fuel_fuel_helper(x)).fold(0, |sum, x| sum + x)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() >= 2 {
        args[1].clone()
    } else {
        "input".to_string()
    };
    let input = load_input(&filename);
    let parsed_input = parse_input(&input);
    let fuel_total = parsed_input.clone().map(|x| calculate_fuel(x)).fold(0, |sum, x| sum + x);

    println!("Answer 1: {}", fuel_total);

    let fuel_total = parsed_input.clone().map(|x| calculate_fuel_fuel(x)).fold(0, |sum, x| sum + x);

    println!("Answer 2: {}", fuel_total);
}
