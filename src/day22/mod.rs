use std::{cmp::max, collections::HashMap, fs};

fn print_vec<T: std::fmt::Display>(name: String, vec: &Vec<T>) {
    print!("{name}: [");
    for val in vec {
        print!("{val},");
    }
    println!("]");
}

fn print_map(map: HashMap<(usize, usize), char>, map_dimensions: (usize, usize)) {
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

fn get_input(file: &str) -> (HashMap<(usize, usize), char>, (usize, usize), Vec<String>) {
    let input = fs::read_to_string(file).expect("Should have been able to read the file");
    let split: Vec<&str> = input.split("\r\n\r\n").collect();
    let mut map: HashMap<(usize, usize), char> = HashMap::new();
    let mut instructions: Vec<String> = Vec::new();
    let map_rows: Vec<&str> = split[0].split("\r\n").collect();
    let mut map_dimensions: (usize, usize) = (map_rows.len(), 0);
    let instruction_string = split[1];

    for (row, row_string) in map_rows.iter().enumerate() {
        let row_chars: Vec<char> = row_string.chars().collect();
        map_dimensions.1 = max(map_dimensions.1, row_chars.len());

        for col in 0..row_chars.len() {
            let map_char = row_chars[col];
            if map_char != ' ' {
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

    return (map, map_dimensions, instructions);
}

pub fn solver(debug: bool) {
    let (map, map_dimensions, instructions) = get_input("./src/day22/input.txt");
    // print_map(map, map_dimensions);
    println!("Day22:");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day22_print_input_example() {
        let (map, map_dimensions, instructions) = get_input("./src/day22/example_input.txt");
        print_map(map, map_dimensions);
        print_vec("Instructions".to_owned(), &instructions);
    }
}
