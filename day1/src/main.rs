use std::fs;
use std::env;
use std::cmp::max;
use std::iter::successors;

// It would be cool if this could just return an iterator (Map) instead of
// collecting the results, but alas my rust-foo is lacking and i cannot
// appease the type and borrow checker
fn load_file(filename: &str) -> Vec<i64> {
    fs::read_to_string(filename).expect("Reading file failed").lines().map(|x| x.parse::<i64>().expect("Failed to convert string to i64")).collect()
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
    let input = load_file(&filename);
    let fuel_total = input.iter().map(|&x| calculate_fuel(x)).fold(0, |sum, x| sum + x);

    println!("Answer 1: {}", fuel_total);

    let fuel_total = input.iter().map(|&x| calculate_fuel_fuel(x)).fold(0, |sum, x| sum + x);

    println!("Answer 2: {}", fuel_total);
}
