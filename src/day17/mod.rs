use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos(i32, i32);

fn shape_one(height: i32) -> HashSet<Pos> {
    let mut shape: HashSet<Pos> = HashSet::new();
    shape.insert(Pos(2, height + 3));
    shape.insert(Pos(3, height + 3));
    shape.insert(Pos(4, height + 3));
    shape.insert(Pos(5, height + 3));
    return shape;
}

fn shape_two(height: i32) -> HashSet<Pos> {
    let mut shape: HashSet<Pos> = HashSet::new();
    shape.insert(Pos(3, height + 5));

    shape.insert(Pos(2, height + 4));
    shape.insert(Pos(3, height + 4));
    shape.insert(Pos(4, height + 4));

    shape.insert(Pos(3, height + 3));

    return shape;
}

fn shape_three(height: i32) -> HashSet<Pos> {
    let mut shape: HashSet<Pos> = HashSet::new();
    shape.insert(Pos(4, height + 5));

    shape.insert(Pos(4, height + 4));

    shape.insert(Pos(2, height + 3));
    shape.insert(Pos(3, height + 3));
    shape.insert(Pos(4, height + 3));

    return shape;
}

fn shape_four(height: i32) -> HashSet<Pos> {
    let mut shape: HashSet<Pos> = HashSet::new();
    shape.insert(Pos(2, height + 6));
    shape.insert(Pos(2, height + 5));
    shape.insert(Pos(2, height + 4));
    shape.insert(Pos(2, height + 3));

    return shape;
}

fn shape_five(height: i32) -> HashSet<Pos> {
    let mut shape: HashSet<Pos> = HashSet::new();
    shape.insert(Pos(2, height + 4));
    shape.insert(Pos(3, height + 4));
    shape.insert(Pos(2, height + 3));
    shape.insert(Pos(3, height + 3));

    return shape;
}

fn shape(shape_type: usize, height: i32) -> HashSet<Pos> {
    match shape_type {
        0 => return shape_one(height + 1),
        1 => return shape_two(height + 1),
        2 => return shape_three(height + 1),
        3 => return shape_four(height + 1),
        4 => return shape_five(height + 1),
        _ => panic!("Shape not available"),
    }
}

fn move_rock_down(rock: &HashSet<Pos>) -> HashSet<Pos> {
    let mut new_rock: HashSet<Pos> = HashSet::new();
    for pos in rock {
        new_rock.insert(Pos(pos.0, pos.1 - 1));
    }
    return new_rock;
}

fn move_rock_left(rock: &HashSet<Pos>) -> HashSet<Pos> {
    let mut new_rock: HashSet<Pos> = HashSet::new();
    for pos in rock {
        new_rock.insert(Pos(pos.0 - 1, pos.1));
        if pos.0 - 1 < 0 {
            return rock.clone();
        }
    }
    return new_rock;
}

fn move_rock_right(rock: &HashSet<Pos>, width: usize) -> HashSet<Pos> {
    let mut new_rock: HashSet<Pos> = HashSet::new();
    for pos in rock {
        new_rock.insert(Pos(pos.0 + 1, pos.1));
        if pos.0 + 1 >= width as i32 {
            return rock.clone();
        }
    }
    return new_rock;
}

fn move_rock(rock: &HashSet<Pos>, jet: char, width: usize) -> HashSet<Pos> {
    match jet {
        '<' => return move_rock_left(rock),
        '>' => return move_rock_right(rock, width),
        _ => return rock.clone(),
    }
}

fn cave_to_hashset(cave: &Vec<usize>) -> HashSet<Pos> {
    let mut cave_hashset: HashSet<Pos> = HashSet::new();
    for (col, height) in cave.iter().enumerate() {
        cave_hashset.insert(Pos(col as i32, *height as i32));
    }
    return cave_hashset;
}

fn merge_rock_with_cave(cave_hashset: &HashSet<Pos>, rock_hashset: &HashSet<Pos>) -> HashSet<Pos> {
    return HashSet::from_iter(cave_hashset.union(&rock_hashset).cloned());
}

fn check_collision(cave_hashset: &HashSet<Pos>, rock_hashset: &HashSet<Pos>) -> bool {
    return cave_hashset.intersection(&rock_hashset).count() != 0;
}

fn get_cave_height(cave_hashset: &HashSet<Pos>) -> usize {
    let mut height = 0;
    for pos in cave_hashset {
        if pos.1 > height {
            height = pos.1;
        }
    }
    return height as usize;
}

fn truncate_cave(cave_hashset: &HashSet<Pos>, width: usize) -> (HashSet<Pos>, usize) {
    let mut new_cave_hashset: HashSet<Pos> = HashSet::new();
    let mut height_map: HashMap<i32, i32> = HashMap::new();
    for pos in cave_hashset {
        if height_map.contains_key(&pos.1) {
            let old_value = height_map.get(&pos.1).unwrap();
            height_map.insert(pos.1, old_value + 1);
        } else {
            height_map.insert(pos.1, 1);
        }
    }
    let mut max_height_full_row = 0;
    for (height, tiles) in height_map {
        if tiles as usize == width && height > max_height_full_row {
            max_height_full_row = height
        }
    }
    for pos in cave_hashset {
        if pos.1 >= max_height_full_row {
            new_cave_hashset.insert(Pos(pos.0, pos.1 - max_height_full_row));
        }
    }

    return (new_cave_hashset, max_height_full_row as usize);
}

// fn render_hash(cave_hashset: &HashSet<Pos>, width: usize) {
//     let height = get_cave_height(&cave_hashset);
//     for y in (0..height + 1).rev() {
//         for x in 0..width {
//             if cave_hashset.contains(&Pos(x as i32, y as i32)) {
//                 print!("#");
//             } else {
//                 print!(".");
//             }
//         }
//         println!();
//     }
// }

fn matches_previous_states(
    states: &Vec<(HashSet<Pos>, usize, usize, usize)>,
    state: &(HashSet<Pos>, usize, usize, usize),
) -> (bool, usize, usize) {
    let (hash_state, num_stone_state, jet_index_state, height_state) = state;
    for (hash, num_stone, jet_index, height) in states {
        if hash == hash_state
            && num_stone % 5 == num_stone_state % 5
            && jet_index == jet_index_state
        {
            return (true, num_stone_state - num_stone, height_state - height);
        }
    }
    return (false, 0, 0);
}

fn find_pattern(height_pattern: &Vec<(usize, usize)>) -> (bool, usize, usize) {
    let mut repeat = 1;
    while repeat + repeat < height_pattern.len() {
        let mut i = repeat;
        let (mut found_pattern, stone_skip, height_skip) = (
            true,
            height_pattern[repeat].0 - height_pattern[0].0,
            height_pattern[repeat].1 - height_pattern[0].1,
        );
        // println!("stone_skip:{}, height_skip:{}", stone_skip, height_skip);
        while i + repeat < height_pattern.len() {
            let (stone_diff, height_diff) = (
                height_pattern[repeat + i].0 - height_pattern[i].0,
                height_pattern[repeat + i].1 - height_pattern[i].1,
            );
            // println!("stone_diff:{}, height_diff:{}", stone_diff, height_diff);
            if stone_skip != stone_diff || height_skip != height_diff {
                found_pattern = false;
                break;
            }
            i += repeat;
        }
        if found_pattern {
            // println!("found pattern");
            return (found_pattern, stone_skip, height_skip);
        }
        repeat += 1;
    }
    return (false, 0, 0);
}

fn check_pattern(
    patterns: &mut HashMap<(usize, usize), Vec<(usize, usize)>>,
    i: usize,
    jet_index: usize,
    height: usize,
    num_rocks: usize,
) -> (usize, usize) {
    if patterns.contains_key(&(i % 5, jet_index)) {
        let height_pattern = patterns.get_mut(&(i % 5, jet_index)).unwrap();
        height_pattern.push((i, height));
        let (matches, stone_skip, height_skip) = find_pattern(&height_pattern);
        if matches {
            let skip = (num_rocks - i) / stone_skip;
            return (skip * stone_skip, skip * height_skip);
        }
    } else {
        patterns.insert((i % 5, jet_index), vec![(i, height)]);
    }
    return (0, 0);
}

fn solve_with_skipping(
    cave: &Vec<usize>,
    jet_pattern: &Vec<char>,
    num_rocks: usize,
    width: usize,
) -> usize {
    let mut cave_hashset = cave_to_hashset(&cave);
    let mut jet_index = 0;
    let mut height = 0;
    let mut overall_height = 0;
    let mut states: Vec<(HashSet<Pos>, usize, usize, usize)> = Vec::new();
    let mut i = 0;
    let mut patterns: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    let mut skipped = false;
    while i < num_rocks {
        // println!("rock: {i}, height: {height}/{overall_height}, cave: {}", cave_hashset.len());

        if !skipped {
            let (stone_skip, height_skip) =
                check_pattern(&mut patterns, i, jet_index, height, num_rocks);
            if stone_skip > 0 {
                i += stone_skip;
                overall_height += height_skip;
                skipped = true;
                continue;
            }
        }

        // let old_height = get_cave_height(&cave_hashset);
        if !skipped {
            let truncated_height: usize;
            (cave_hashset, truncated_height) = truncate_cave(&cave_hashset, width);
            if truncated_height != 0 {
                overall_height += truncated_height;
                let new_state = (cave_hashset.clone(), i, jet_index, overall_height);
                let (matches, stone_skip, height_skip) =
                    matches_previous_states(&states, &new_state);
                states.push(new_state);
                height = get_cave_height(&cave_hashset);
                // println!("Truncated Cave from height {old_height} -> {height}, truncated: {truncated_height}/{overall_height}");

                if matches {
                    let skip = (num_rocks - i) / stone_skip;
                    i += skip * stone_skip;
                    overall_height += skip * height_skip;
                    // println!("Skip stone: {}, Skip height: {}", skip * stone_skip, skip * height_skip);
                    skipped = true;
                    continue;
                }
            }
        }

        // if i % 1000 == 0 {
        //     println!(
        //         "rock: {i}, height: {height}, truncated -> {}",
        //         cave_hashset.len()
        //     );
        // }
        let mut rock = shape(i % 5, height as i32);
        loop {
            let jet = jet_pattern[jet_index];
            jet_index = (jet_index + 1) % jet_pattern.len();
            // println!("Move: {jet}");
            let rock_move = move_rock(&rock, jet, width);
            let collision = check_collision(&cave_hashset, &rock_move);
            if !collision {
                rock = rock_move;
            }
            let rock_down = move_rock_down(&rock);
            let collision = check_collision(&cave_hashset, &rock_down);
            if collision {
                cave_hashset = merge_rock_with_cave(&cave_hashset, &rock);
                height = get_cave_height(&cave_hashset);
                break;
            }
            rock = rock_down;
        }
        i += 1;
    }
    // render_hash(&cave_hashset, width);
    return overall_height + height;
}

pub fn solver() {
    let input = fs::read_to_string("./src/day17/input.txt")
        .expect("Should have been able to read the file");
    let jet_pattern: Vec<char> = input.chars().collect();
    let width = 7;
    let cave: Vec<usize> = vec![0; width];
    let num_rocks_part_one = 2022;
    let num_rocks_part_two = 1000000000000;
    println!("Day17:");
    let height_part_one = solve_with_skipping(&cave, &jet_pattern, num_rocks_part_one, width);
    println!("Part one: Cave height {height_part_one} after {num_rocks_part_one} rocks");

    let height_part_two = solve_with_skipping(&cave, &jet_pattern, num_rocks_part_two, width);
    println!("Part two: Cave height {height_part_two} after {num_rocks_part_two} rocks");
}
