use core::panic;
use std::fs;

fn index_of_closing_bracket(slice: &str) -> usize {
    let mut opening_brackets = 0;
    let mut closing_brackets = 0;
    for (index, char) in slice.chars().enumerate() {
        if char == '[' {
            opening_brackets += 1;
        }
        if char == ']' {
            closing_brackets += 1;
        }
        if opening_brackets == closing_brackets {
            return index;
        }
    }
    panic!(
        "no closing bracket found for opening bracket for: {}",
        slice
    );
}

fn convert_to_list(elements: &str) -> Vec<&str> {
    if !elements.contains("[") {
        return elements.split(',').collect();
    }
    let chars: Vec<char> = elements.chars().collect();
    if chars[0] == '[' && elements.chars().position(|char| char == ']').unwrap() == elements.chars().count() - 1 {
        return elements[1..elements.len() - 1].split(',').collect();
    }

    let mut list: Vec<&str> = Vec::new();

    let mut last_index = 0;
    let mut index = 0;
    while index < elements.len() {
        if chars[index] == '[' {
            let end = index_of_closing_bracket(&elements[index..]); // check for empty list
            list.push(&elements[index + 1..index + end]);
            index = index + end + 1;
            last_index = index;
            continue;
        }
        if chars[index] == ',' {
            if index != last_index {
                list.push(&elements[last_index..index]);
            }
            index += 1;
            last_index = index;
            continue;
        }
        index += 1;
    }
    if last_index != index {
        list.push(&elements[last_index..index]);
    }
    return list;
}

fn compare_pair(pair_one: &str, pair_two: &str) -> i32 {
    let next_elements_one = convert_to_list(pair_one);
    let next_elements_two = convert_to_list(pair_two);
    for (index, ele_one) in next_elements_one.iter().enumerate() {
        if index >= next_elements_two.len() {
            return -1;
        }
        let ele_two = next_elements_two[index];

        if (ele_one.contains(",") || ele_one.contains("[")  || ele_one.contains("]")) 
        && (ele_two.contains(",") || ele_two.contains("[")  || ele_two.contains("]")) {
            let comp = compare_pair(ele_one, ele_two);
            if comp != 0 {
                return comp;
            }
            continue;
        }

        if ele_one.contains(",") || ele_one.contains("[") {
            let list_one = convert_to_list(ele_one);
            if list_one.is_empty() {
                return 1;
            }
            let comp = compare_pair(list_one.first().unwrap(), ele_two);
            if comp != 0 {
                return comp;
            }
            if list_one.len() == 1 {
                continue;
            }
            return -1;
        }
        if ele_two.contains(",") || ele_two.contains("["){
            let list_two = convert_to_list(ele_two);
            if list_two.is_empty() {
                return -1;
            }
            let comp = compare_pair(ele_one, list_two.first().unwrap());
            if comp != 0 {
                return comp;
            }
            if list_two.len() == 1 {
                continue;
            }
            return 1;
        }

        if ele_one.is_empty() && ele_two.is_empty() {
            continue;
        }
        if ele_one.is_empty() {
            return 1;
        }
        if ele_two.is_empty() {
            return -1;
        }

        let val_one: usize = ele_one.parse().unwrap();
        let val_two: usize = ele_two.parse().unwrap();
        if val_one < val_two {
            return 1;
        }
        if val_one > val_two {
            return -1;
        }
    }

    if next_elements_one.len() < next_elements_two.len() {
        return 1;
    }
    if next_elements_one.len() > next_elements_two.len() {
        return -1;
    }
    return 0;
}

pub fn solver() {
    let input = fs::read_to_string("./src/day13/input.txt")
        .expect("Should have been able to read the file");
    let pairs: Vec<&str> = input.split("\r\n\r\n").collect();
    let mut ordered_pairs: Vec<usize> = Vec::new();
    for (index, pair) in pairs.iter().enumerate() {
        let split: Vec<&str> = pair.split("\r\n").collect();
        let comp = compare_pair(split[0], split[1]);
        println!("compare: {} and {} == {}", split[0], split[1], comp);
        if comp == 1 {
            ordered_pairs.push(index + 1);
        }
    }

    let sum_part_one: usize = ordered_pairs.iter().sum();
    println!("Day13:");
    println!("Number of Ordered Indices: {}", sum_part_one);
}
