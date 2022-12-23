use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    fs,
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
fn print_map(map: &HashSet<(i32, i32)>) {
    let bounding_box = get_bounding_box(map);
    for row in bounding_box.0 .0..bounding_box.0 .1 {
        for col in bounding_box.1 .0..bounding_box.1 .1 {
            if map.contains(&(row, col)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn are_adjacent_positions_empty(map: &HashSet<(i32, i32)>, pos: &(i32, i32)) -> bool {
    for row_diff in [-1,0,1] {
        for col_diff in [-1,0,1]{
            if row_diff == 0 && col_diff == 0 {
                continue;
            }
            if map.contains(&(pos.0 + row_diff, pos.1 + col_diff)){
                return false;
            }
        }
    }
    return true;
}

fn check(map: &HashSet<(i32, i32)>, pos: &(i32, i32), round: usize) -> (bool, (i32, i32)) {
    let round_mod_4 = round % 4;
    let directions: Vec<char> = vec!['N', 'S', 'W', 'E'];
    if are_adjacent_positions_empty(map,pos) {
        return (false, (0,0));
    }

    for round_mod_4 in round_mod_4..round_mod_4 + 4 {
        let direction = directions[round_mod_4 % 4];
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

fn get_bounding_box(map: &HashSet<(i32, i32)>) -> ((i32, i32), (i32, i32)) {
    let mut bounding_box = ((i32::MAX, i32::MIN), (i32::MAX, i32::MIN));
    for pos in map {
        bounding_box.0 .0 = min(bounding_box.0 .0, pos.0);
        bounding_box.0 .1 = max(bounding_box.0 .1, pos.0);
        bounding_box.1 .0 = min(bounding_box.1 .0, pos.1);
        bounding_box.1 .1 = max(bounding_box.1 .1, pos.1);
    }
    bounding_box.0 .1 += 1;
    bounding_box.1 .1 += 1;
    return bounding_box;
}

fn perform_one_step(map: &HashSet<(i32, i32)>, round: usize) -> HashSet<(i32, i32)> {
    let mut proposed_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut duplicated_proposed_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut elf_proposals: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    for elf_pos in map {
        let (moves, proposed_position) = check(&map, elf_pos, round);
        if moves {
            if proposed_positions.contains(&proposed_position) {
                duplicated_proposed_positions.insert(proposed_position);
                continue;
            }
            proposed_positions.insert(proposed_position);
            elf_proposals.insert(*elf_pos, proposed_position);
        }
    }
    let mut new_map: HashSet<(i32, i32)> = HashSet::new();

    for elf_pos in map {
        if elf_proposals.contains_key(elf_pos) {
            let proposed_position = elf_proposals.get(elf_pos).unwrap();
            let proposed_position_is_duplicated =
                duplicated_proposed_positions.contains(proposed_position);
            if !proposed_position_is_duplicated {
                new_map.insert(*proposed_position);
                continue;
            }
        }
        new_map.insert(*elf_pos);
    }
    return new_map;
}

fn empty_tiles(map: &HashSet<(i32, i32)>) -> usize {
    let bounding_box = get_bounding_box(&map);
    let area: usize = ((bounding_box.0 .1 - bounding_box.0 .0)
        * (bounding_box.1 .1 - bounding_box.1 .0)) as usize;
    return area - map.len();
}

fn solve_part_one(map_in: &HashSet<(i32, i32)>) -> usize {
    let mut map = map_in.clone();

    for round in 0..10 {
        map = perform_one_step(&map, round);
    }
    return empty_tiles(&map);
}

fn solve_part_two(map_in: &HashSet<(i32, i32)>) -> usize {
    let mut map = map_in.clone();

    let mut round = 0;
    loop {
        let new_map = perform_one_step(&map, round);
        if new_map == map {
            return round + 1;
        }
        map = new_map;
        round += 1;
    }
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
    let map = get_input("./src/day23/input.txt");
    println!("Day23:");
    let empty_area = solve_part_one(&map);
    println!("Empty area part one: {empty_area}");
    let number_of_rounds_until_no_movement = solve_part_two(&map);
    println!("Round in which no elf moves: {number_of_rounds_until_no_movement}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day23_small_example_one_step() {
        let mut map = get_input("./src/day23/small_example_input.txt");
        // print_map(&map);
        assert_eq!(3, empty_tiles(&map));
        map = perform_one_step(&map, 0);
        // println!();
        // print_map(&map);
        assert_eq!(5, empty_tiles(&map));
    }

    #[test]
    fn day23_example_part_one() {
        let map = get_input("./src/day23/example_input.txt");
        let solution_part_one = solve_part_one(&map);
        assert_eq!(110, solution_part_one);
    }

    #[test]
    fn day23_part_one() {
        let map = get_input("./src/day23/input.txt");
        let solution_part_one = solve_part_one(&map);
        assert_eq!(3849, solution_part_one);
    }

    #[test]
    fn day23_example_part_two() {
        let map = get_input("./src/day23/example_input.txt");
        let solution_part_two = solve_part_two(&map);
        assert_eq!(20, solution_part_two);
    }

    #[test]
    fn day23_part_two() {
        let map = get_input("./src/day23/input.txt");
        let solution_part_two = solve_part_two(&map);
        assert_eq!(995, solution_part_two);
    }
}
