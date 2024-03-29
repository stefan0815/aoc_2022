use std::{
    cmp::max,
    collections::{HashMap, HashSet},
    fs,
};

#[allow(dead_code)]
fn print_blizzards(blizzards: &HashMap<(i32, i32), Vec<char>>, dimensions: (i32, i32)) {
    println!();
    for row in 0..dimensions.0 {
        for col in 0..dimensions.1 {
            let pos = (row, col);
            if is_out_of_bounds(pos, dimensions) {
                print!("#");
                continue;
            }
            if blizzards.contains_key(&pos) {
                let blizzard = blizzards.get(&pos).unwrap();
                if blizzard.len() == 1 {
                    print!("{}", blizzard[0]);
                } else {
                    print!("{}", blizzard.len());
                }
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[allow(dead_code)]
fn print_path(path: &Vec<(i32, i32)>) {
    print!("Path: [");
    for pos in path {
        print!("({},{}),", pos.0, pos.1);
    }
    println!("]");
}

fn distance(pos: (i32, i32), goal: (i32, i32)) -> usize {
    return (goal.0.abs_diff(pos.0) + goal.1.abs_diff(pos.1)) as usize;
}

fn is_out_of_bounds(new_pos: (i32, i32), dimensions: (i32, i32)) -> bool {
    if new_pos.0 <= 0 || new_pos.1 <= 0 {
        return true;
    }

    if new_pos.0 >= dimensions.0 - 1 || new_pos.1 >= dimensions.1 - 1 {
        return true;
    }

    return false;
}

fn is_out_of_bounds_with_start(
    new_pos: (i32, i32),
    dimensions: (i32, i32),
    start: (i32, i32),
) -> bool {
    if new_pos == start {
        return false;
    }

    return is_out_of_bounds(new_pos, dimensions);
}

fn possible_moves(
    blizzards: &HashMap<(i32, i32), Vec<char>>,
    dimensions: (i32, i32),
    pos: (i32, i32),
    start: (i32, i32),
    goal: (i32, i32),
) -> Vec<(i32, i32)> {
    let mut possible_new_positions: Vec<(i32, i32)> = Vec::new();

    for new_pos in [
        pos,
        (pos.0 + 1, pos.1),
        (pos.0, pos.1 + 1),
        (pos.0 - 1, pos.1),
        (pos.0, pos.1 - 1),
    ] {
        if new_pos == goal {
            return vec![goal];
        }
        if pos != start && new_pos == start {
            continue;
        }
        if is_out_of_bounds_with_start(new_pos, dimensions, start) {
            continue;
        }
        if blizzards.contains_key(&new_pos) {
            continue;
        }
        possible_new_positions.push(new_pos);
    }
    return possible_new_positions;
}

fn advance_blizzards(
    blizzards: &HashMap<(i32, i32), Vec<char>>,
    dimensions: (i32, i32),
) -> HashMap<(i32, i32), Vec<char>> {
    let mut new_blizzards: HashMap<(i32, i32), Vec<char>> = HashMap::new();
    for (blizzard_pos, blizzard_directions) in blizzards {
        for direction in blizzard_directions {
            let mut new_blizzard_pos: (i32, i32);
            match direction {
                '>' => new_blizzard_pos = (blizzard_pos.0, blizzard_pos.1 + 1),
                'v' => new_blizzard_pos = (blizzard_pos.0 + 1, blizzard_pos.1),
                '<' => new_blizzard_pos = (blizzard_pos.0, blizzard_pos.1 - 1),
                '^' => new_blizzard_pos = (blizzard_pos.0 - 1, blizzard_pos.1),
                _ => panic!("wrong blizzard direction"),
            }
            if is_out_of_bounds(new_blizzard_pos, dimensions) {
                match direction {
                    '>' => new_blizzard_pos = (new_blizzard_pos.0, 1),
                    'v' => new_blizzard_pos = (1, new_blizzard_pos.1),
                    '<' => new_blizzard_pos = (new_blizzard_pos.0, dimensions.1 - 2),
                    '^' => new_blizzard_pos = (dimensions.0 - 2, new_blizzard_pos.1),
                    _ => panic!("wrong blizzard direction"),
                }
            }
            if new_blizzards.contains_key(&new_blizzard_pos) {
                new_blizzards
                    .get_mut(&new_blizzard_pos)
                    .unwrap()
                    .push(*direction);
            } else {
                new_blizzards.insert(new_blizzard_pos, vec![*direction]);
            }
        }
    }
    return new_blizzards;
}

fn get_all_paths(
    blizzards: &mut HashMap<usize, HashMap<(i32, i32), Vec<char>>>,
    dimensions: (i32, i32),
    pos: (i32, i32),
    start: (i32, i32),
    goal: (i32, i32),
    time: usize,
    no_progress_limit: usize,
    best_soltion_so_far: usize,
    previous_positions: &mut HashSet<((i32, i32), usize)>,
    debug: bool,
) -> Vec<Vec<(i32, i32)>> {
    if previous_positions.contains(&(pos, time)) {
        return vec![vec![]];
    }
    previous_positions.insert((pos, time));
    if debug {
        println!("Current pos: ({},{})", pos.0, pos.1);
    }
    let distance_to_goal = distance(pos, goal);
    if best_soltion_so_far > 0 && time + distance_to_goal > best_soltion_so_far {
        return vec![vec![]];
    }
    let mut all_paths: Vec<Vec<(i32, i32)>> = Vec::new();
    let new_blizzard;
    if blizzards.contains_key(&(time + 1)) {
        new_blizzard = blizzards.get(&(time + 1)).unwrap().clone();
    } else if blizzards.contains_key(&time) {
        let current_blizzard = blizzards.get(&time).unwrap();
        new_blizzard = advance_blizzards(current_blizzard, dimensions);
        blizzards.insert(time + 1, new_blizzard.clone());
    } else {
        panic!("Current time is not in Blizzard Hashset");
    }

    let possible_moves = possible_moves(&new_blizzard, dimensions, pos, start, goal);
    if debug {
        println!("Possible moves: {}", possible_moves.len());
    }
    for new_pos in possible_moves {
        if new_pos == goal {
            return vec![vec![goal]];
        }

        let mut new_stand_still_limit = no_progress_limit;
        let new_distance = distance(new_pos, goal);
        if new_distance >= distance_to_goal {
            if no_progress_limit <= 0 {
                continue;
            }
            new_stand_still_limit -= 1;
        }

        let paths = get_all_paths(
            blizzards,
            dimensions,
            new_pos,
            start,
            goal,
            time + 1,
            new_stand_still_limit,
            best_soltion_so_far,
            previous_positions,
            debug,
        );
        for path in paths {
            let mut current_path = vec![new_pos];
            current_path.append(&mut path.clone());
            all_paths.push(current_path);
        }
    }
    return all_paths;
}

fn solve(
    initial_blizzard: &HashMap<(i32, i32), Vec<char>>,
    dimensions: (i32, i32),
    start: (i32, i32),
    goal: (i32, i32),
    debug: bool,
) -> (Vec<(i32, i32)>, HashMap<(i32, i32), Vec<char>>) {
    let mut best_path_so_far: Vec<(i32, i32)> = Vec::new();
    let mut last_blizzard: HashMap<(i32, i32), Vec<char>> = HashMap::new();
    let distance_to_goal = distance(start, goal);
    for no_progress_limit in [128, 256, 512] {
        let mut previous_positions = HashSet::new();
        let mut blizzards: HashMap<usize, HashMap<(i32, i32), Vec<char>>> = HashMap::new();
        blizzards.insert(0, initial_blizzard.clone());
        let all_paths = get_all_paths(
            &mut blizzards,
            dimensions,
            start,
            start,
            goal,
            0,
            no_progress_limit,
            best_path_so_far.len(),
            &mut previous_positions,
            false,
        );
        let mut all_paths_reaching_goal: Vec<Vec<(i32, i32)>> = Vec::new();
        for path in &all_paths {
            if path.len() > 0 && *path.last().unwrap() == goal {
                all_paths_reaching_goal.push(path.clone());
            }
        }
        if debug {
            println!(
                "no_progress_limit: {no_progress_limit}: {}/{} paths reached the goal",
                all_paths_reaching_goal.len(),
                all_paths.len()
            );
        }
        all_paths_reaching_goal.sort_by(|a, b| a.len().cmp(&b.len()));
        if all_paths_reaching_goal.len() > 0 {
            best_path_so_far = all_paths_reaching_goal[0].clone();
            last_blizzard = blizzards.get(&best_path_so_far.len()).unwrap().clone();
            return (best_path_so_far, last_blizzard);
        }
        if best_path_so_far.len() == distance_to_goal {
            last_blizzard = blizzards.get(&best_path_so_far.len()).unwrap().clone();
            return (best_path_so_far, last_blizzard);
        }
        // if debug {
        //     print_path(&best_path_so_far);
        // }
    }
    return (best_path_so_far, last_blizzard);
}

fn get_input(
    file: &str,
) -> (
    HashMap<(i32, i32), Vec<char>>,
    (i32, i32),
    (i32, i32),
    (i32, i32),
) {
    let input = fs::read_to_string(file).expect("Should have been able to read the file");
    let rows: Vec<&str> = input.split("\r\n").collect();
    let mut blizzards: HashMap<(i32, i32), Vec<char>> = HashMap::new();
    let mut start: (i32, i32) = (0, 0);
    let mut goal: (i32, i32) = (0, 0);
    let mut dimensions: (i32, i32) = (rows.len() as i32, 0);
    for (row, row_string) in rows.iter().enumerate() {
        let row_chars: Vec<char> = row_string.chars().collect();
        dimensions.1 = max(dimensions.1, row_chars.len() as i32);

        for col in 0..row_chars.len() {
            let map_char = row_chars[col];
            if row == 0 && map_char == '.' {
                start = (row as i32, col as i32);
            }
            if row == rows.len() - 1 && map_char == '.' {
                goal = (row as i32, col as i32);
            }
            if map_char != '#' && map_char != '.' {
                blizzards.insert((row as i32, col as i32), vec![map_char]);
            }
        }
    }

    return (blizzards, dimensions, start, goal);
}

pub fn solver(debug: bool) {
    let (initial_blizzard, dimensions, start, goal) = get_input("./src/day24/input.txt");

    let (best_path, last_blizzard) = solve(&initial_blizzard, dimensions, start, goal, debug);
    println!("Day24:");
    println!("Shortest path is {} minutes", best_path.len());
    let (best_path_back_to_start, last_blizzard_back_to_start) =
        solve(&last_blizzard, dimensions, goal, start, debug);
    println!(
        "Shortest path back to start {} minutes",
        best_path_back_to_start.len()
    );
    let (best_path_back_to_goal, _) =
        solve(&last_blizzard_back_to_start, dimensions, start, goal, debug);
    println!(
        "Shortest path back to goal {} minutes",
        best_path_back_to_goal.len()
    );
    println!(
        "Shortest path three trips overall {} minutes",
        best_path.len() + best_path_back_to_start.len() + best_path_back_to_goal.len()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day24_simple_example_get_input() {
        let (_, dimensions, start, goal) = get_input("./src/day24/simple_example_input.txt");
        assert_eq!((7, 7), dimensions);
        assert_eq!((0, 1), start);
        assert_eq!((6, 5), goal);
    }

    #[test]
    fn day24_example_get_input() {
        let (_, dimensions, start, goal) = get_input("./src/day24/example_input.txt");
        assert_eq!((6, 8), dimensions);
        assert_eq!((0, 1), start);
        assert_eq!((5, 6), goal);
    }

    #[test]
    fn day24_print_example_print_blizzards() {
        let (blizzards, dimensions, _, _) = get_input("./src/day24/example_input.txt");
        print_blizzards(&blizzards, dimensions);
        let new_blizzard = advance_blizzards(&blizzards, dimensions);
        print_blizzards(&new_blizzard, dimensions);
    }

    #[test]
    fn day24_print_example_solve() {
        let (initial_blizzard, dimensions, start, goal) =
            get_input("./src/day24/example_input.txt");
        let (best_path, last_blizzard) = solve(&initial_blizzard, dimensions, start, goal, false);
        assert_eq!(18, best_path.len());
        let (best_path_back_to_start, last_blizzard_back_to_start) =
            solve(&last_blizzard, dimensions, goal, start, false);
        let (best_path_back_to_goal, _) =
            solve(&last_blizzard_back_to_start, dimensions, start, goal, false);

        assert_eq!(
            54,
            best_path.len() + best_path_back_to_start.len() + best_path_back_to_goal.len()
        );
    }
}
