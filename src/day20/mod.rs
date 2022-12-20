use std::fs;

use pathfinding::num_traits::signum;

fn print_vec(name: &str, vec: &Vec<i32>) {
    print!("{name}: [");
    for val in vec {
        print!("{val},");
    }
    println!("]");
}

pub fn solver(debug: bool) {
    let input = fs::read_to_string("./src/day20/input.txt")
        .expect("Should have been able to read the file");
    let file: Vec<i32> = input
        .split("\r\n")
        .map(|line| line.parse::<i32>().unwrap())
        .collect();
    let file_length = file.len() as i32;
    let mut mixed_file = file.clone();
    if debug {
        print_vec("file", &file);
    }
    for value in &file {
        if *value == 0 {
            continue;
        }
        let position = mixed_file.iter().position(|val| val == value).unwrap();
        let mut shift = *value;
        if *value > 0 {
            shift += 1;
        }
        let wraps = (shift / file_length).abs() + 1;
        let mut new_position =
            ((position as i32 + wraps * file_length + shift) % file_length) as usize;
        if debug {
            println!("Move {value} from {position} -> {new_position}");
        }
        if new_position == 0 {
            new_position = file.len();
        }
        else if new_position == file.len() {
            new_position = 0;
        }
        if new_position < position {
            mixed_file.remove(position);
            if debug {
                print_vec("mixed_file after delete", &mixed_file);
            }
            mixed_file.insert(new_position, *value);
            if debug {
                print_vec("mixed_file after insert", &mixed_file);
            }
        } else {
            mixed_file.insert(new_position, *value);
            if debug {
                print_vec("mixed_file after insert", &mixed_file);
            }
            mixed_file.remove(position);
            if debug {
                print_vec("mixed_file after delete", &mixed_file);
            }
        }
        // if debug {
        //     println!("After mixing {value}");
        //     print_vec("mixed_file", &mixed_file);
        // }
    }

    let pos_zero = mixed_file.iter().position(|val| *val == 0).unwrap();
    let pos_1000 = (pos_zero + 1000) % file.len();
    let pos_2000 = (pos_zero + 2000) % file.len();
    let pos_3000 = (pos_zero + 3000) % file.len();
    let sum_part_one = mixed_file[pos_1000] + mixed_file[pos_2000] + mixed_file[pos_3000];
    println!("Day20:");
    println!("Sum of part one: {sum_part_one}");
}
