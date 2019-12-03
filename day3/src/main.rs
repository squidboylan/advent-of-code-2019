use std::fs;
use std::env;

#[derive(Clone)]
enum Move {
    U(i64),
    D(i64),
    L(i64),
    R(i64),
}

fn convert_to_move(m: &str) -> Move {
    if m.starts_with('U') {
        let val = m.get(1..).unwrap().parse::<i64>().expect("failed to convert string to i64");
        Move::U(val)
    } else if m.starts_with('D') {
        let val = m.get(1..).unwrap().parse::<i64>().expect("failed to convert string to i64");
        Move::D(val)

    } else if m.starts_with('L') {
        let val = m.get(1..).unwrap().parse::<i64>().expect("failed to convert string to i64");
        Move::L(val)

    } else if m.starts_with('R') {
        let val = m.get(1..).unwrap().parse::<i64>().expect("failed to convert string to i64");
        Move::R(val)
    } else {
        unreachable!("this shouldnt happen");
    }
}

fn load_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Reading file failed")
}

fn parse_input(data: & str) -> impl Iterator<Item = Move> + '_ + Clone {
    data.split(',').map(|x| convert_to_move(x))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() >= 2 {
        args[1].clone()
    } else {
        "input".to_string()
    };
    let input = load_input(&filename);
    let split_input: Vec<&str> = input.split('\n').collect();
    let parsed_input1 = parse_input(split_input[0]);
    let parsed_input2 = parse_input(split_input[1]);

    let mut distance_map1: Vec<Vec<((i64, i64), i64)>> = Vec::new();
    distance_map1.push(Vec::new());
    add_moves_to_map(&mut distance_map1, parsed_input1);

    let mut distance_map2: Vec<Vec<((i64, i64), i64)>> = Vec::new();
    distance_map2.push(Vec::new());
    add_moves_to_map(&mut distance_map2, parsed_input2);

    // Part 1
    'outer: for (distance, pairs) in distance_map1.iter().enumerate().skip(1) {
        for (coord1, _) in pairs {
            for (coord2, _) in distance_map2[distance].iter() {
                if *coord2 == *coord1 {
                    println!("part1: {}", distance);
                    break 'outer;
                }
            }
        }
    }

    let mut shortest_wire_distance = std::i64::MAX;
    // Part 2
    for (distance, pairs) in distance_map1.iter().enumerate().skip(1) {
        for (coord1, d1) in pairs {
            for (coord2, d2) in distance_map2[distance].iter() {
                if *coord2 == *coord1 && *d1 + *d2 < shortest_wire_distance {
                    shortest_wire_distance = *d1 + *d2;
                }
            }
        }
    }
    println!("part2: {}", shortest_wire_distance);
}

fn add_moves_to_map(distance_map: &mut Vec<Vec<((i64, i64), i64)>>, input: impl Iterator<Item = Move> + Clone) {
    let mut wire_length = 0;
    let mut curr: (i64, i64) = (0, 0);
    distance_map[0].push((curr, wire_length));
    for m in input {
        let (mut x, mut y) = curr;
        if let Move::U(mut moves) = m {
            while moves > 0 {
                wire_length += 1;
                y += 1;
                moves -= 1;
                let index = (y.abs() + x.abs()) as usize;
                if distance_map.len() <= index {
                    distance_map.push(vec![((x, y), wire_length)]);
                } else {
                    distance_map[index].push(((x, y), wire_length));
                }
            }
        } else if let Move::D(mut moves) = m {
            while moves > 0 {
                wire_length += 1;
                y -= 1;
                moves -= 1;
                let index = (y.abs() + x.abs()) as usize;
                if distance_map.len() <= index {
                    distance_map.push(vec![((x, y), wire_length)]);
                } else {
                    distance_map[index].push(((x, y), wire_length));
                }
            }
        } else if let Move::L(mut moves) = m {
            while moves > 0 {
                wire_length += 1;
                x -= 1;
                moves -= 1;
                let index = (y.abs() + x.abs()) as usize;
                if distance_map.len() <= index {
                    distance_map.push(vec![((x, y), wire_length)]);
                } else {
                    distance_map[index].push(((x, y), wire_length));
                }
            }
        } else if let Move::R(mut moves) = m {
            while moves > 0 {
                wire_length += 1;
                x += 1;
                moves -= 1;
                let index = (y.abs() + x.abs()) as usize;
                if distance_map.len() <= index {
                    distance_map.push(vec![((x, y), wire_length)]);
                } else {
                    distance_map[index].push(((x, y), wire_length));
                }
            }
        }

        curr = (x, y)
    }
}
