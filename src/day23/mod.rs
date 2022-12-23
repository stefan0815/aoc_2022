use std::{
    collections::{HashMap, HashSet},
    fs, cmp::{min, max},
};

#[allow(dead_code)]
fn print_vec<T: std::fmt::Display>(name: String, vec: &Vec<T>) {
    print!("{name}: [");
    for val in vec {
        print!("{val},");
    }
    println!("]");
}

#[allow(dead_code)]
fn print_map(map: &HashSet<(usize, usize)>, map_dimensions: &(usize, usize)) {
    for row in 0..map_dimensions.0 {
        for col in 0..map_dimensions.1 {
            if map.contains(&(row, col)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn check(map: &HashSet<(i32, i32)>, pos: &(i32, i32), round: usize) -> (bool, (i32, i32)) {
    let round_mod_4 = round % 4;
    let directions: Vec<char> = vec!['N', 'S', 'W', 'E'];

    for round_mod_4 in round_mod_4..round_mod_4 + 4 {
        let direction = directions[round_mod_4];
        let check_positions: Vec<(i32, i32)>;
        match direction {
            'N' => {
                check_positions = vec![
                    (pos.0 - 1, pos.1 - 1),
                    (pos.0 - 1, pos.1),
                    (pos.0 - 1, pos.1 + 1),
                ]
            }
            'S' => {
                check_positions = vec![
                    (pos.0 + 1, pos.1 - 1),
                    (pos.0 + 1, pos.1),
                    (pos.0 + 1, pos.1 + 1),
                ]
            }
            'W' => {
                check_positions = vec![
                    (pos.0 - 1, pos.1 - 1),
                    (pos.0, pos.1 - 1),
                    (pos.0 + 1, pos.1 - 1),
                ]
            }
            'E' => {
                check_positions = vec![
                    (pos.0 - 1, pos.1 + 1),
                    (pos.0, pos.1 + 1),
                    (pos.0 + 1, pos.1 + 1),
                ]
            }
            _ => panic!("Can't happen"),
        }
        let mut empty_direction = true;
        for check_position in &check_positions {
            if map.contains(&check_position) {
                empty_direction = false;
                break;
            }
        }
        if empty_direction {
            return (true, check_positions[1]);
        }
    }

    return (false, (0, 0));
}

fn  get_bounding_box(map:&HashSet<(i32, i32)>) -> ((i32,i32),(i32,i32)){
    let mut bounding_box = ((i32::MAX, i32::MIN), (i32::MAX, i32::MIN));
    for pos in map {
        bounding_box.0.0 = min(bounding_box.0.0, pos.0);
        bounding_box.0.1 = max(bounding_box.0.1, pos.0);
        bounding_box.1.0 = min(bounding_box.1.0, pos.1);
        bounding_box.1.1 = max(bounding_box.1.1, pos.1);
    }
    return bounding_box;
}

fn solve_part_one(map_in: &HashSet<(i32, i32)>) -> usize {
    let mut map = map_in.clone();
    for round in 0..10 {
        let mut proposed_positions: HashSet<(i32, i32)> = HashSet::new();
        let mut duplicated_proposed_positions: HashSet<(i32, i32)> = HashSet::new();
        let mut elf_proposals: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
        for elf_pos in &map {
            let (can_move, proposed_position) = check(&map, elf_pos, round);
            if can_move {
                if proposed_positions.contains(&proposed_position){
                    duplicated_proposed_positions.insert(proposed_position);
                    continue;
                }
                proposed_positions.insert(proposed_position);
                elf_proposals.insert(*elf_pos, proposed_position);
            }
        }
        let mut new_map: HashSet<(i32, i32)> = HashSet::new();

        for elf_pos in &map {
            if elf_proposals.contains_key(elf_pos) {
                let proposed_position = elf_proposals.get(elf_pos).unwrap();
                let proposed_position_is_duplicated = duplicated_proposed_positions.contains(proposed_position);
                if !proposed_position_is_duplicated {
                    new_map.insert(*proposed_position);
                    continue;
                }
            }
            new_map.insert(*elf_pos);
        }
        map = new_map;
    }
    let bounding_box = get_bounding_box(&map);
    let area:usize = ((bounding_box.0.1 - bounding_box.0.0) * (bounding_box.1.1 - bounding_box.1.0)) as usize;
    return area - map.len();
}

fn get_input(file: &str) -> HashSet<(i32, i32)> {
    let input = fs::read_to_string(file).expect("Should have been able to read the file");
    let rows: Vec<&str> = input.split("\r\n").collect();
    let mut map: HashSet<(i32, i32)> = HashSet::new();
    for (row, row_string) in rows.iter().enumerate() {
        let row_chars: Vec<char> = row_string.chars().collect();
        for col in 0..row_chars.len() {
            let map_char = row_chars[col];
            if map_char == '#' {
                map.insert((row as i32, col as i32));
            }
        }
    }

    return map;
}

pub fn solver(_: bool) {
    let map = get_input("./src/day22/example_input.txt");
    println!("Day23:");
    let empty_area = solve_part_one(&map);
    println!("Empty area part one: {empty_area}");
}

// #[cfg(test)]
// mod tests {
//     use super::*;
// }
