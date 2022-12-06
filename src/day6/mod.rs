use std::{collections::HashSet, fs};

fn find_distinct_characters(input: &String, num_of_distinct_chars: usize) -> usize {
    let char_vec_input:Vec<char> = input.chars().collect();
    for index in num_of_distinct_chars..input.chars().count() {
        let current_slice = char_vec_input[index-num_of_distinct_chars..index].to_vec();
        let sequence_hash: HashSet<char> = HashSet::from_iter(current_slice.to_vec());
        if sequence_hash.len() == num_of_distinct_chars{
            return index;
        }
    }
    return 0;
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
