use std::{collections::HashMap, fs};

fn print_vec(name: &str, vec: &Vec<i128>) {
    print!("{name}: [");
    for val in vec {
        print!("{val},");
    }
    println!("]");
}

fn move_distance(vec: &mut Vec<i128>, index: usize) {
    let distance = vec.remove(index);
    let wrap_length = vec.len() as i128;
    let mut new_index = index as i128 + distance;
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

fn solve(encrypted_file: &Vec<i128>, iterations: usize, debug: bool) -> (Vec<i128>, i128) {
    let mut encrypted_file_no_duplicates = encrypted_file.clone();
    let wrap_length = encrypted_file.len() as i128 - 1;
    let mut values: HashMap<i128, i128> = HashMap::new();
    for value in &mut encrypted_file_no_duplicates {
        let original_value = value.clone();
        while values.contains_key(value) {
            let new_value = *value + wrap_length * value.signum();
            *value = new_value;
        }
        values.insert(*value, original_value);
    }

    let mut mixed_file_no_duplicates = encrypted_file_no_duplicates.clone();

    for _ in 0..iterations {
        for value in &encrypted_file_no_duplicates {
            if *value == 0 {
                continue;
            }
            let index = mixed_file_no_duplicates
                .iter()
                .position(|val| val == value)
                .unwrap();

            move_distance(&mut mixed_file_no_duplicates, index);
            if debug {
                print_vec(
                    "mixed_file_no_duplicates after move",
                    &mixed_file_no_duplicates,
                );
            }
        }
    }

    let pos_zero = mixed_file_no_duplicates
        .iter()
        .position(|val| *val == 0)
        .unwrap();
    let pos_1000 = (pos_zero + 1000) % encrypted_file.len();
    let pos_2000 = (pos_zero + 2000) % encrypted_file.len();
    let pos_3000 = (pos_zero + 3000) % encrypted_file.len();

    let mut value_at_pos_1000 = mixed_file_no_duplicates[pos_1000];
    if values.contains_key(&mixed_file_no_duplicates[pos_1000]) {
        value_at_pos_1000 = *values.get(&value_at_pos_1000).unwrap();
    }
    let mut value_at_pos_2000 = mixed_file_no_duplicates[pos_2000];
    if values.contains_key(&value_at_pos_2000) {
        value_at_pos_2000 = *values.get(&value_at_pos_2000).unwrap();
    }
    let mut value_at_pos_3000 = mixed_file_no_duplicates[pos_3000];
    if values.contains_key(&value_at_pos_3000) {
        value_at_pos_3000 = *values.get(&value_at_pos_3000).unwrap();
    }
    let sum_part_one = value_at_pos_1000 + value_at_pos_2000 + value_at_pos_3000;
    return (mixed_file_no_duplicates, sum_part_one);
}

pub fn solver(debug: bool) {
    let input = fs::read_to_string("./src/day20/input.txt")
        .expect("Should have been able to read the file");
    let encrypted_file: Vec<i128> = input
        .split("\r\n")
        .map(|line| line.parse::<i128>().unwrap())
        .collect();
    if debug {
        print_vec("file", &encrypted_file);
        println!("file length: {}", encrypted_file.len());
    }
    println!("Day20:");
    let (_, sum_part_one) = solve(&encrypted_file, 1, debug);
    println!("Sum of part one: {sum_part_one}");

    let encryption_key = 811589153;
    let encrypted_file_with_key:Vec<i128> = encrypted_file.iter().map(|value| value * encryption_key).collect();
    let (_, sum_part_two) = solve(&encrypted_file_with_key, 10, debug);
    println!("Sum of part two: {sum_part_two}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day20_solve_example() {
        let encrypted_file: Vec<i128> = vec![1, 2, -3, 3, -2, 0, 4];
        let (mixed_file, sum_part_one) = solve(&encrypted_file, 1, false);

        assert_eq!(vec![1, 2, -3, 4, 0, 3, -2], mixed_file);
        assert_eq!(3, sum_part_one);
    }

    #[test]
    fn day20_move_example_partial() {
        let mut vec: Vec<i128> = vec![1, 2, -2, -3, 0, 3, 4];
        move_distance(&mut vec, 2);
        assert_eq!(vec![1, 2, -3, 0, 3, 4, -2], vec);
    }

    #[test]
    fn day20_move_example_one_line_1() {
        let mut vec: Vec<i128> = vec![4, 5, 6, 1, 7, 8, 9];
        move_distance(&mut vec, 3);
        assert_eq!(vec![4, 5, 6, 7, 1, 8, 9], vec);
    }

    #[test]
    fn day20_move_example_one_line_2() {
        let mut vec: Vec<i128> = vec![4, -2, 5, 6, 7, 8, 9];
        move_distance(&mut vec, 1);
        assert_eq!(vec![4, 5, 6, 7, 8, -2, 9], vec);
    }
}
