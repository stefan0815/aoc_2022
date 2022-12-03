use std::{collections::HashSet, fs, iter::FromIterator};

fn find_common_letter_vec(strings: Vec<&str>) -> char {
    let mut hash: HashSet<char> = HashSet::from_iter(strings[0].chars().clone());
    // let first_string = strings[0];
    // println!("{first_string}");

    for string in &strings {
        let second_hash: HashSet<char> = HashSet::from_iter(string.chars().clone());
        let intersection = hash.intersection(&second_hash);
        let intersection_char: Vec<char> = intersection.cloned().collect();
        hash = HashSet::from_iter(intersection_char);
        // let collects = Vec::from_iter(hash.clone());
        // let string_from_chars:String = collects.iter().collect();
        // println!("{string_from_chars}");
    }

    let collection: Vec<char> = Vec::from_iter(hash);
    return collection[0];
}

fn find_common_letter(string1: &str, string2: &str) -> char {
    for letter1 in string1.chars() {
        for letter2 in string2.chars() {
            if letter1 == letter2 {
                return letter1;
            }
        }
    }
    return '\0';
}

fn convert_to_value(letter: char) -> u32 {
    let ascii: u32 = letter as u32;
    if ascii >= 97 {
        return ascii - 96;
    }
    return ascii - 38;
}

pub fn solver() {
    let input =
        fs::read_to_string("./src/day3/input.txt").expect("Should have been able to read the file");
    let rucksacks: Vec<&str> = input.split("\r\n").collect();
    let num_of_rucksacks = rucksacks.len();
    let mut sum: u32 = 0;
    for rucksack in &rucksacks {
        let rucksack_size = rucksack.chars().count();
        let index_of_half_rucksack = rucksack_size / 2;
        let compartment1: &str = &rucksack[0..index_of_half_rucksack];
        let compartment2: &str = &rucksack[index_of_half_rucksack..rucksack_size];
        let common_letter = find_common_letter(compartment1, compartment2);
        if common_letter == '\0' {
            println!("Common letter not found");
            println!("{compartment1}");
            println!("{compartment2}");
            println!("{rucksack}");
            println!("{rucksack_size}");
            println!("{index_of_half_rucksack}");
            continue;
        }
        sum += convert_to_value(common_letter);
    }

    let mut sum_part2: u32 = 0;
    for i in (0..num_of_rucksacks - 2).step_by(3) {
        let common_letter = find_common_letter_vec(rucksacks[i..i + 3].to_vec());
        // println!("{common_letter}");
        sum_part2 += convert_to_value(common_letter);
    }

    println!("Day3:");
    println!("Sum for first part: {sum}");
    println!("Sum for second part: {sum_part2}");
}
