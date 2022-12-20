use std::fs;

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
    let wrap_length = (file.len()) as i32;
    let mut mixed_file = file.clone();
    if debug {
        print_vec("file", &file);
    }
    for value in &file {
        if *value == 0 {
            continue;
        }
        let position = mixed_file.iter().position(|val| val == value).unwrap();
        let shift = *value;
        let wraps = (shift / wrap_length).abs() + 1;
        let new_position_no_modulo = position as i32 + shift;
        let mut new_position =
            ((new_position_no_modulo + wraps * wrap_length) % wrap_length) as usize;
        if new_position_no_modulo < 0 && new_position > position {
            new_position -= 1;
        } else if new_position_no_modulo > wrap_length && new_position < position {
            new_position += 1;
        } else if shift < 0 && new_position == 0 {
            new_position = file.len() - 1;
        } else if shift > 0 && new_position == file.len() - 1 {
            new_position = 0;
        }
        // if new_position_no_modulo < 0 {
        //     new_position =
        //         (((new_position_no_modulo + wraps * wrap_length - 1) % wrap_length)) as usize;
        //     if new_position < position {
        //         new_position = ((new_position_no_modulo + wraps * wrap_length) % wrap_length) as usize;
        //     }
        // } else {
        //     new_position = ((new_position_no_modulo + wraps * wrap_length) % wrap_length) as usize;
        // }
        if debug {
            println!("Move {value} from {position} -> {new_position}");
        }

        // else if *value > 0 && new_position == (wrap_length - 1) as usize {
        //     new_position = 0 as usize;
        // }

        mixed_file.remove(position);
        if debug {
            print_vec("mixed_file after delete", &mixed_file);
        }
        mixed_file.insert(new_position, *value);
        if debug {
            print_vec("mixed_file after insert", &mixed_file);
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
    println!("{}", mixed_file[pos_1000]);
    println!("{}", mixed_file[pos_2000]);
    println!("{}", mixed_file[pos_3000]);
    let sum_part_one = mixed_file[pos_1000] + mixed_file[pos_2000] + mixed_file[pos_3000];
    println!("Day20:");
    println!("Sum of part one: {sum_part_one}");
}
