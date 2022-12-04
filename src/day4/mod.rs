use std::{collections::HashSet, fs, iter::FromIterator};

fn convert_range_to_integer_list(range: &str) -> std::ops::Range<i32> {
    let start_end: Vec<&str> = range.split("-").collect();
    let start: i32 = start_end[0].parse().unwrap();
    let end: i32 = start_end[1].parse().unwrap();
    return start..end + 1;
}

fn check_for_full_overlap(elves: Vec<&str>) -> [bool; 2]  {
    let range_of_elf1 = convert_range_to_integer_list(elves[0]);
    let range_of_elf2 = convert_range_to_integer_list(elves[1]);
    let hash1: HashSet<i32> = HashSet::from_iter(range_of_elf1);
    let hash2: HashSet<i32> = HashSet::from_iter(range_of_elf2);

    let subset1 = hash1.is_subset(&hash2);
    let subset2 = hash2.is_subset(&hash1);
    return [subset1 || subset2, !hash1.is_disjoint(&hash2)];
}

pub fn solver() {
    let input =
        fs::read_to_string("./src/day4/input.txt").expect("Should have been able to read the file");
    let pairs: Vec<&str> = input.split("\r\n").collect();
    let mut sum_part1 = 0;
    let mut sum_part2 = 0;
    for pair in pairs {
        let elves: Vec<&str> = pair.split(",").collect();
        let overlap = check_for_full_overlap(elves);
        if overlap[0] {
            sum_part1 += 1;
        }
        if overlap[1]{
            sum_part2 += 1;
        }
    }

    println!("Day4:");
    println!("Sum for first part: {sum_part1}");
    println!("Sum for second part: {sum_part2}");
}
