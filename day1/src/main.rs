use std::fs;
use std::env;
use std::cmp::max;

fn load_file(filename: &str) -> Vec<i64> {
    fs::read_to_string(filename).expect("Reading file failed").lines().map(|x| x.parse::<i64>().expect("Failed to convert string to i64")).collect()
}

fn calculate_fuel(mass: i64) -> i64 {
    max(0, mass / 3 - 2)
}

// This calculates the fuel needed for the mass + the fuel needed for the fuel as is required for
// part 2
fn calculate_fuel_fuel(mass: i64) -> i64 {
    let mut fuel_total = calculate_fuel(mass);
    let mut fuel_to_add = calculate_fuel(fuel_total);
    while fuel_to_add > 0 {
        fuel_total += fuel_to_add;
        fuel_to_add = calculate_fuel(fuel_to_add);
    }
    fuel_total
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
