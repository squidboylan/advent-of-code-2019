use std::fs;
use std::env;

fn load_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Reading file failed")
}

fn parse_input<'a>(data: &'a str) -> impl Iterator<Item = i64> + 'a + Clone {
    data.split(',').map(|x| { x.parse::<i64>().expect("Failed to convert string to i64") })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() >= 2 {
        args[1].clone()
    } else {
        "input".to_string()
    };
    let mut input = load_input(&filename);
    input.pop();
    let mut parsed_input: Vec<i64> = parse_input(&input).collect();

    // Fix wrong inputs
    parsed_input[1] = 12;
    parsed_input[2] = 2;

    process_opcodes(&mut parsed_input);

    println!("part 1: {}", parsed_input[0]);

    // Part 2
    'outer: loop { 
        for noun in 0..=99 {
            for verb in 0..=99 {
                parsed_input = parse_input(&input).collect();
                // Set noun and verb
                parsed_input[1] = noun;
                parsed_input[2] = verb;
                process_opcodes(&mut parsed_input);
                if parsed_input[0] == 19690720 {
                    break 'outer;
                }
            }
        }
    }

    println!("part 2: noun: {}, verb: {}", parsed_input[1], parsed_input[2]);
}

fn process_opcodes(input: &mut [i64]) {
    // "Instruction pointer"
    let mut ip = 0;
    loop {
        match input[ip] {
            1 => add_instruction(input, &mut ip),
            2 => mul_instruction(input, &mut ip),
            99 => return,
            _ => unreachable!("This opcode should not be reached"),
        }
    }
}

fn add_instruction(input: &mut [i64], ip: &mut usize) {
    let val1 = input[input[*ip + 1] as usize];
    let val2 = input[input[*ip + 2] as usize];
    let dest = input[*ip + 3];

    input[dest as usize] = val1 + val2;
    *ip += 4;
}

fn mul_instruction(input: &mut [i64], ip: &mut usize) {
    let val1 = input[input[*ip + 1] as usize];
    let val2 = input[input[*ip + 2] as usize];
    let dest = input[*ip + 3];

    input[dest as usize] = val1 * val2;
    *ip += 4;
}
