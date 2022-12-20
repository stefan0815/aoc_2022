use std::fs;

fn print_vec(name: &str, vec: &Vec<i32>) {
    print!("{name}: [");
    for val in vec {
        print!("{val},");
    }
    println!("]");
}

fn move_distance(vec: &mut Vec<i32>, value: i32) {
    let index = vec.iter().position(|val| *val == value).unwrap();
    let distance = vec.remove(index);
    let wrap_length = vec.len() as i32;
    let mut new_index = index as i32 + distance;
    if new_index < 0 {
        new_index = new_index + ((new_index / wrap_length).abs() + 1) * wrap_length;
    }
    if new_index > wrap_length {
        new_index = new_index % wrap_length;
    }
    if new_index == 0 {
        new_index = wrap_length;
    }
    vec.insert(new_index as usize, distance);
}

pub fn solver(debug: bool) {
    let input = fs::read_to_string("./src/day20/input.txt")
        .expect("Should have been able to read the file");
    let file: Vec<i32> = input
        .split("\r\n")
        .map(|line| line.parse::<i32>().unwrap())
        .collect();
    let mut mixed_file = file.clone();
    if debug {
        print_vec("file", &file);
    }

    println!("file length: {}", file.len());
    for value in &file {
        if *value == 0 || (*value).abs() as usize == file.len() {
            continue;
        }

        move_distance(&mut mixed_file, *value);
        if debug {
            print_vec("mixed_file after move", &mixed_file);
        }
    }

    print_vec("mixed_file", &mixed_file);
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
