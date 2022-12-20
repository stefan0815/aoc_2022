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
    new_index = ((new_index % wrap_length) + wrap_length) % wrap_length;

    if new_index == 0 {
        new_index = wrap_length;
    }
    vec.insert(new_index as usize, distance);
}

fn map_to_unique_vec(vec: &Vec<i128>) -> (Vec<i128>, HashMap<i128, i128>) {
    let mut unique_vec = vec.clone();
    let wrap_length = vec.len() as i128 - 1;
    let mut values_mapping: HashMap<i128, i128> = HashMap::new();
    for value in &mut unique_vec {
        let original_value = value.clone();
        while values_mapping.contains_key(value) {
            let new_value = *value + wrap_length * value.signum();
            *value = new_value;
        }
        values_mapping.insert(*value, original_value);
    }
    return (unique_vec, values_mapping);
}

fn get_original_value(vec: &Vec<i128>, index: usize, values_mapping: &HashMap<i128, i128>) -> i128 {
    let mut value = vec[index % vec.len()];
    if values_mapping.contains_key(&value) {
        value = *values_mapping.get(&value).unwrap();
    }
    return value;
}

fn solve(encrypted_file: &Vec<i128>, iterations: usize, debug: bool) -> (Vec<i128>, i128) {
    let wrap_length = encrypted_file.len() as i128 - 1;
    let (encrypted_file_no_duplicates, values_mapping) = map_to_unique_vec(encrypted_file);
    let mut mixed_file_no_duplicates = encrypted_file_no_duplicates.clone();

    for _ in 0..iterations {
        for value in &encrypted_file_no_duplicates {
            if *value == 0 && value % wrap_length == 0 {
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
    let sum_part_one =
        get_original_value(&mixed_file_no_duplicates, pos_zero + 1000, &values_mapping)
            + get_original_value(&mixed_file_no_duplicates, pos_zero + 2000, &values_mapping)
            + get_original_value(&mixed_file_no_duplicates, pos_zero + 3000, &values_mapping);
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
    let encrypted_file_with_key: Vec<i128> = encrypted_file
        .iter()
        .map(|value| value * encryption_key)
        .collect();
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

    #[test]
    fn day20_part_1() {
        let input = fs::read_to_string("./src/day20/input.txt")
            .expect("Should have been able to read the file");
        let encrypted_file: Vec<i128> = input
            .split("\r\n")
            .map(|line| line.parse::<i128>().unwrap())
            .collect();
        let (_, sum_part_one) = solve(&encrypted_file, 1, false);
        assert_eq!(8764, sum_part_one);
    }

    #[test]
    fn day20_part_2() {
        let input = fs::read_to_string("./src/day20/input.txt")
            .expect("Should have been able to read the file");
        let encrypted_file: Vec<i128> = input
            .split("\r\n")
            .map(|line| line.parse::<i128>().unwrap())
            .collect();
        let encryption_key = 811589153;
        let encrypted_file_with_key: Vec<i128> = encrypted_file
            .iter()
            .map(|value| value * encryption_key)
            .collect();
        let (_, sum_part_one) = solve(&encrypted_file_with_key, 10, false);
        assert_eq!(535648840980, sum_part_one);
    }
}
