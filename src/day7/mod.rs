use std::{collections::HashMap, fs};

fn convert_to_full_name(current_directory: &Vec<&str>) -> String {
    if current_directory.len() == 1 {
        return current_directory[0].to_owned();
    }

    let mut full_directory_name: String = String::from("");
    for current_path in &current_directory[1..] {
        full_directory_name += "/";
        full_directory_name += &current_path.to_string();
    }
    return full_directory_name;
}

fn add_size_to_path(
    directories: &mut HashMap<String, usize>,
    current_directory: &Vec<&str>,
    size: usize,
) {
    if current_directory.len() == 0 {
        return;
    }
    let directory_name = convert_to_full_name(&current_directory);

    if directories.contains_key(&directory_name) {
        let old_size = directories[&directory_name];
        *directories.get_mut(&directory_name).unwrap() = size + old_size;
    } else {
        directories.insert(directory_name, size);
    }
    add_size_to_path(directories, &current_directory[..current_directory.len() - 1 ].to_vec(), size);
}

fn build_directories(commands: &Vec<&str>) -> HashMap<String, usize> {
    let mut directories: HashMap<String, usize> = HashMap::new();
    let mut current_directory: Vec<&str> = Vec::new();
    for command in commands {
        if command.starts_with("$ ls") || command.starts_with("dir") {
            continue;
        }

        if command.starts_with("$ cd ") {
            let directory = &command[5..];
            if directory == ".." {
                current_directory.pop();
            } else {
                current_directory.push(directory);
            }
            continue;
        }
        let size_str: Vec<&str> = command.split(" ").collect();
        let size: usize = size_str[0].parse().unwrap();
        add_size_to_path(&mut directories, &current_directory, size);
    }
    return directories;
}

fn calculate_part_one(directories: &HashMap<String, usize>) -> usize {
    let mut sum = 0;
    for (_, value) in directories {
        if value < &(100000 as usize) {
            sum += value;
        }
    }
    return sum;
}

fn calculate_part_two(directories: &HashMap<String, usize>) -> usize {
    let total_disk_space = 70000000 as usize;
    let total_disk_usage = directories["/"];
    let total_disk_usage_goal = 30000000 as usize;
    let free_space = total_disk_space - total_disk_usage;
    if free_space > total_disk_usage_goal {
        println!("Already enough space");
        return 0;
    }

    let size_to_be_freed = total_disk_usage_goal - free_space;
    // println!("Size to be freed: {}", size_to_be_freed);
    let mut optimal_size = total_disk_space;
    for (_, value) in directories {
        if value > &size_to_be_freed && value < &optimal_size {
            optimal_size = *value;
        }
    }
    return optimal_size;
}

pub fn solver() {
    let input =
        fs::read_to_string("./src/day7/input.txt").expect("Should have been able to read the file");
    let commands: Vec<&str> = input.split("\r\n").collect();
    let directories = build_directories(&commands);

    let sum_part_one = calculate_part_one(&directories);
    let part_two = calculate_part_two(&directories);

    println!("Day7:");
    println!("First part: {sum_part_one}");
    println!("Second part: {part_two}");
}
