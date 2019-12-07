use std::fs;
use std::env;
use std::io;
use std::io::*;

fn load_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Reading file failed")
}

fn parse_input<'a>(data: &'a str) -> impl Iterator<Item = i64> + 'a + Clone {
    data.trim().split(',').map(|x| { x.parse::<i64>().expect("Failed to convert string to i64") })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() >= 2 {
        args[1].clone()
    } else {
        "input".to_string()
    };
    let mut input = load_input(&filename);
    let mut parsed_input: Vec<i64> = parse_input(&input).collect();
    process_opcodes(&mut parsed_input);
}


fn process_opcodes(input: &mut [i64]) {
    // "Instruction pointer"
    let mut ip = 0;
    loop {
        let instruction = format!("{:05}", input[ip]);
        println!("{}", instruction);
        let mode3 = instruction.get(0..1).unwrap();
        let mode2 = instruction.get(1..2).unwrap();
        let mode1 = instruction.get(2..3).unwrap();
        let opcode = instruction.get(3..).unwrap();
        match opcode {
            "01" => add_instruction(input, &mut ip, mode1, mode2, mode3),
            "02" => mul_instruction(input, &mut ip, mode1, mode2, mode3),
            "03" => input_instruction(input, &mut ip),
            "04" => output_instruction(input, &mut ip, mode1),
            "05" => jump_t_instruction(input, &mut ip, mode1, mode2),
            "06" => jump_f_instruction(input, &mut ip, mode1, mode2),
            "07" => lt_instruction(input, &mut ip, mode1, mode2),
            "08" => eq_instruction(input, &mut ip, mode1, mode2),
            "99" => return,
            _ => unreachable!("This opcode should not be reached"),
        }
    }
}

fn add_instruction(input: &mut [i64], ip: &mut usize, mode1: &str, mode2: &str, mode3: &str) {
    let val1 = if mode1 == "0" {
        input[input[*ip + 1] as usize]
    } else {
        input[*ip + 1]
    };
    let val2 = if mode2 == "0" {
        input[input[*ip + 2] as usize]
    } else {
        input[*ip + 2]
    };
    let dest = input[*ip + 3];

    input[dest as usize] = val1 + val2;
    *ip += 4;
}

fn mul_instruction(input: &mut [i64], ip: &mut usize, mode1: &str, mode2: &str, mode3: &str) {
    let val1 = if mode1 == "0" {
        input[input[*ip + 1] as usize]
    } else {
        input[*ip + 1]
    };
    let val2 = if mode2 == "0" {
        input[input[*ip + 2] as usize]
    } else {
        input[*ip + 2]
    };
    let dest = input[*ip + 3];

    input[dest as usize] = val1 * val2;
    *ip += 4;
}

fn input_instruction(input: &mut [i64], ip: &mut usize) {
    print!("input an int: ");
    // print! doesnt automatically flush stdout
    io::stdout().flush().unwrap();
    let mut input_val = String::new();
    io::stdin().read_line(&mut input_val).expect("error: unable to read user input");
    // Convert our stdin IO to an int
    let input_val: i64 = input_val.trim().parse().expect("enter an int");
    let dest = input[*ip + 1];

    input[dest as usize] = input_val;
    *ip += 2;
}

fn output_instruction(input: &mut [i64], ip: &mut usize, mode1: &str) {
    let dest = input[*ip + 1];
    println!("{}", input[dest as usize]);

    *ip += 2;
}

fn jump_t_instruction(input: &mut [i64], ip: &mut usize, mode1: &str, mode2: &str) {
    let val1 = if mode1 == "0" {
        input[input[*ip + 1] as usize]
    } else {
        input[*ip + 1]
    };
    let dest = if mode2 == "0" {
        input[input[*ip + 2] as usize]
    } else {
        input[*ip + 2]
    };
    if val1 != 0 {
        *ip = dest as usize;
    } else {
        *ip += 3;
    }
}

fn jump_f_instruction(input: &mut [i64], ip: &mut usize, mode1: &str, mode2: &str) {
    let val1 = if mode1 == "0" {
        input[input[*ip + 1] as usize]
    } else {
        input[*ip + 1]
    };
    let dest = if mode2 == "0" {
        input[input[*ip + 2] as usize]
    } else {
        input[*ip + 2]
    };
    if val1 == 0 {
        *ip = dest as usize;
    } else {
        *ip += 3;
    }
}

fn lt_instruction(input: &mut [i64], ip: &mut usize, mode1: &str, mode2: &str) {
    let val1 = if mode1 == "0" {
        input[input[*ip + 1] as usize]
    } else {
        input[*ip + 1]
    };
    let val2 = if mode2 == "0" {
        input[input[*ip + 2] as usize]
    } else {
        input[*ip + 2]
    };
    let dest = input[*ip + 3];
    input[dest as usize] = if val1 < val2 { 1 } else { 0 };
    *ip += 4;
}

fn eq_instruction(input: &mut [i64], ip: &mut usize, mode1: &str, mode2: &str) {
    let val1 = if mode1 == "0" {
        input[input[*ip + 1] as usize]
    } else {
        input[*ip + 1]
    };
    let val2 = if mode2 == "0" {
        input[input[*ip + 2] as usize]
    } else {
        input[*ip + 2]
    };
    let dest = input[*ip + 3];
    input[dest as usize] = if val1 == val2 { 1 } else { 0 };
    *ip += 4;
}
