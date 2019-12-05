use std::fs;
use std::env;
use std::collections::HashSet;

fn load_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Reading file failed")
}

fn parse_input<'a>(data: &'a str) -> impl Iterator<Item = i64> + 'a + Clone {
    data.trim().split('-').map(|x| { x.parse::<i64>().expect("Failed to convert string to i64") })
}

fn part1_check(curr: i64) -> bool {
    let mut divider = 10;
    while divider < curr {
        divider = divider * 10;
    }
    let mut last = -1;
    let mut duplicate = false;

    while divider > 1 {
        let val = (curr % divider) / (divider / 10);
        if last == val {
            duplicate = true;
        } else if last > val {
            break;
        }
        last = val;
        divider = divider / 10;
    }
    // Divider will only be 1 if we make it through the while loop without breaking early
    if divider == 1 && duplicate == true {
        return true;
    }
    false
}

fn part2_check(curr: i64) -> bool {
    let mut divider = 10;
    while divider < curr {
        divider = divider * 10;
    }
    let mut last = -1;
    let mut duplicates = HashSet::new();
    let mut bad_duplicates = HashSet::new();

    while divider > 1 {
        let val = (curr % divider) / (divider / 10);
        if last == val {
            if duplicates.contains(&val) {
                bad_duplicates.insert(val);
            } else {
                duplicates.insert(val);
            }
        } else if last > val {
            break;
        }
        last = val;
        divider = divider / 10;
    }
    // Divider will only be 1 if we make it through the while loop without breaking early
    if divider == 1 && duplicates.difference(&bad_duplicates).next() != None {
        return true;
    }
    false
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() >= 2 {
        args[1].clone()
    } else {
        "input".to_string()
    };
    let input = load_input(&filename);
    let parsed_input: Vec<i64> = parse_input(&input).collect();
    let lower_bound = parsed_input[0];
    let upper_bound = parsed_input[1];

    // Part 1
    let count = (lower_bound..upper_bound).map(|x| part1_check(x)).filter(|x| *x).count();

    println!("part 1: {}", count);

    // Part 2
    let count = (lower_bound..upper_bound).map(|x| part2_check(x)).filter(|x| *x).count();

    println!("part 2: {}", count);
}
