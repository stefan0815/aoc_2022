use std::{cmp::max, collections::HashMap, fs, os::windows::process};

fn print_vec<T: std::fmt::Display>(name: String, vec: &Vec<T>) {
    print!("{name}: [");
    for val in vec {
        print!("{val},");
    }
    println!("]");
}

fn print_map(map: &HashMap<(usize, usize), char>, map_dimensions: &(usize, usize)) {
    for row in 0..map_dimensions.0 {
        for col in 0..map_dimensions.1 {
            if map.contains_key(&(row, col)) {
                print!("{}", map.get(&(row, col)).unwrap());
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn perform_rotation(facing_direction: usize, rotation: String) -> usize {
    match rotation.as_str() {
        "R" => return (facing_direction + 1) % 4,
        "L" => return (facing_direction + 4 - 1) % 4,
        _ => panic!("Invalid rotation string {rotation}"),
    }
}

fn advance(
    pos: &(usize, usize),
    facing_direction: usize,
    map: &HashMap<(usize, usize), char>,
    map_dimensions: &(usize, usize),
) -> (usize, usize) {
    let mut new_pos: (i32, i32) = (pos.0 as i32, pos.1 as i32);

    match facing_direction {
        0 => new_pos = (new_pos.0, new_pos.1 + 1),
        1 => new_pos = (new_pos.0 + 1, new_pos.1),
        2 => new_pos = (new_pos.0, new_pos.1 - 1),
        3 => new_pos = (new_pos.0 - 1, new_pos.1),
        _ => panic!("Invalid facing direction {facing_direction}"),
    }

    return (new_pos.0 as usize, new_pos.1 as usize);
}

fn process_instructions(
    start: &(usize, usize),
    map: &HashMap<(usize, usize), char>,
    map_dimensions: &(usize, usize),
    instructions: &Vec<String>,
) -> ((usize, usize), usize) {
    let mut pos = start.clone();
    let mut facing_direction: usize = 0; // 0: right, 1:down, 2:left, 3:up

    for instruction in instructions {
        if instruction.as_str() == "R" || instruction.as_str() == "L" {
            facing_direction = perform_rotation(facing_direction, instruction.to_string());
            continue;
        }
        let distance: usize = instruction.parse().unwrap();
        for _ in 0..distance {
            let new_pos = advance(&pos, facing_direction, map, map_dimensions);
            if new_pos == pos {
                break;
            }
            pos = new_pos;
        }
    }
    return (pos, facing_direction);
}

fn get_input(
    file: &str,
) -> (
    (usize, usize),
    HashMap<(usize, usize), char>,
    (usize, usize),
    Vec<String>,
) {
    let input = fs::read_to_string(file).expect("Should have been able to read the file");
    let split: Vec<&str> = input.split("\r\n\r\n").collect();
    let mut map: HashMap<(usize, usize), char> = HashMap::new();
    let mut instructions: Vec<String> = Vec::new();
    let map_rows: Vec<&str> = split[0].split("\r\n").collect();
    let mut map_dimensions: (usize, usize) = (map_rows.len(), 0);
    let instruction_string = split[1];
    let mut start: (usize, usize) = (0, 0);
    let mut start_initialized = false;
    for (row, row_string) in map_rows.iter().enumerate() {
        let row_chars: Vec<char> = row_string.chars().collect();
        map_dimensions.1 = max(map_dimensions.1, row_chars.len());

        for col in 0..row_chars.len() {
            let map_char = row_chars[col];
            if map_char != ' ' {
                if !start_initialized {
                    start = (row, col);
                    start_initialized = true;
                }
                map.insert((row, col), map_char);
            }
        }
    }
    let instructions_split_right: Vec<&str> = instruction_string.split("R").collect();

    for (index_right, instruction_split_right) in instructions_split_right.iter().enumerate() {
        let instructions_split_left: Vec<&str> = instruction_split_right.split("L").collect();
        for (index_left, instruction_split_left) in instructions_split_left.iter().enumerate() {
            instructions.push((*instruction_split_left).to_owned());
            if index_left < instructions_split_left.len() - 1 {
                instructions.push("L".to_owned());
            }
        }
        if index_right < instruction_split_right.len() - 1 {
            instructions.push("R".to_owned());
        }
    }

    return (start, map, map_dimensions, instructions);
}

fn solve_part_one(
    start: &(usize, usize),
    map: &HashMap<(usize, usize), char>,
    map_dimensions: &(usize, usize),
    instructions: &Vec<String>,
) -> usize {
    let (end_position, facing_direction) =
        process_instructions(&start, &map, &map_dimensions, &instructions);

    let password = 1000 * (end_position.0 + 1) + 4 * (end_position.1 + 1) + facing_direction;
    return password;
}

pub fn solver(debug: bool) {
    let (start, map, map_dimensions, instructions) = get_input("./src/day22/input.txt");
    let password = solve_part_one(&start, &map, &map_dimensions, &instructions);
    println!("Day22:");
    println!("Password part one: {password}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day22_print_input_example() {
        let (_, map, map_dimensions, instructions) = get_input("./src/day22/example_input.txt");
        print_map(&map, &map_dimensions);
        print_vec("Instructions".to_owned(), &instructions);
    }

    #[test]
    fn day22_parse_example_check_start() {
        let (start, _, _, _) = get_input("./src/day22/example_input.txt");
        assert_eq!((0, 8), start);
    }

    #[test]
    fn day22_process_instructions() {
        let (start, map, map_dimensions, instructions) = get_input("./src/day22/example_input.txt");
        let (end, facing_direction) = process_instructions(&start, &map, &map_dimensions, &instructions);

        assert_eq!((5, 7), end);
        assert_eq!(0, facing_direction);
    }
}
