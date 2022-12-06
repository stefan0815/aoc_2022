use std::{fs, collections::HashSet};


fn find_distinct_characters(input : &String, num_of_distinct_chars: usize) -> i32{
    let mut index = 0;
    let mut current_sequence: Vec<char> = Vec::new();
    for char in input.chars() {
        current_sequence.push(char);
        index += 1;
        if current_sequence.len() == num_of_distinct_chars {
            let sequence_hash: HashSet<char> = HashSet::from_iter(current_sequence.clone());
            if sequence_hash.len() == num_of_distinct_chars {
                return index;
            }
            current_sequence.remove(0);
        }
    }
    return index;
}
pub fn solver() {
    let input =
        fs::read_to_string("./src/day6/input.txt").expect("Should have been able to read the file");

    let first_part = find_distinct_characters(&input, 4);
    let second_part = find_distinct_characters(&input, 14);
    

    println!("Day6:");
    println!("First part: {first_part}");
    println!("Second part: {second_part}");
}
