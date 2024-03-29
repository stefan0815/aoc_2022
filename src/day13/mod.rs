use core::panic;
use std::cmp::Ordering;
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

fn expand_to_list(elements: &str) -> Vec<&str> {
    if !elements.contains("[") {
        return elements.split(',').collect();
    }

    let mut list: Vec<&str> = Vec::new();
    let chars: Vec<char> = elements.chars().collect();

    let mut last_index = 0;
    let mut index = 0;
    while index < elements.len() {
        if chars[index] == '[' {
            let end = index_of_closing_bracket(&elements[index..]);
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

fn is_list(ele: &str) -> bool {
    return ele.contains(",") || ele.contains("[") || ele.contains("]");
}

fn compare(elements_one: &str, elements_two: &str) -> Ordering {
    let next_elements_one = expand_to_list(elements_one);
    let next_elements_two = expand_to_list(elements_two);
    for (index, ele_one) in next_elements_one.iter().enumerate() {
        if index >= next_elements_two.len() {
            return Ordering::Greater;
        }
        let ele_two = next_elements_two[index];

        if is_list(&ele_one) || is_list(ele_two) {
            let comp = compare(ele_one, ele_two);
            if comp == Ordering::Equal {
                continue;
            }
            return comp;
        }

        if ele_one.is_empty() && ele_two.is_empty() {
            continue;
        }
        if ele_one.is_empty() {
            return Ordering::Less;
        }
        if ele_two.is_empty() {
            return Ordering::Greater;
        }

        let val_one: i32 = ele_one.parse().unwrap();
        let val_two: i32 = ele_two.parse().unwrap();
        if val_one < val_two {
            return Ordering::Less;
        }
        if val_one > val_two {
            return Ordering::Greater;
        }
    }

    if next_elements_one.len() < next_elements_two.len() {
        return Ordering::Less;
    }
    if next_elements_one.len() > next_elements_two.len() {
        return Ordering::Greater;
    }
    return Ordering::Equal;
}

pub fn solver() {
    let input = fs::read_to_string("./src/day13/input.txt")
        .expect("Should have been able to read the file");
    let pairs: Vec<&str> = input.split("\r\n\r\n").collect();
    let mut indices_of_correcly_ordered_pairs: Vec<usize> = Vec::new();
    for (index, pair) in pairs.iter().enumerate() {
        let split: Vec<&str> = pair.split("\r\n").collect();
        let comp = compare(split[0], split[1]);
        // let comp_val;
        // match comp {
        //     Ordering::Less => comp_val = 1,
        //     Ordering::Equal => comp_val = 0,
        //     Ordering::Greater => comp_val = -1,
        // }
        // println!("compare: {} and {} == {}", split[0], split[1], comp_val);
        if comp == Ordering::Less {
            indices_of_correcly_ordered_pairs.push(index + 1);
        }
    }
    let sum_part_one: usize = indices_of_correcly_ordered_pairs.iter().sum();

    let mut packets: Vec<&str> = Vec::new();
    packets.push("[[2]]");
    packets.push("[[6]]");
    for pair in pairs {
        let mut split: Vec<&str> = pair.split("\r\n").collect();
        packets.append(&mut split);
    }

    packets.sort_by(|a, b| return compare(a, b));

    let mut mult_part_two = 1;
    for (index, packet) in packets.iter().enumerate() {
        // println!("{}", packet);
        if *packet == "[[2]]" || *packet == "[[6]]" {
            mult_part_two *= index + 1;
        }
    }
    println!("Day13:");
    println!("Number of Ordered Indices: {}", sum_part_one);
    println!("Multiplication of divider Indices: {}", mult_part_two);
}
