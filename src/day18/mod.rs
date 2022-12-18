use std::{
    cmp::{max, min},
    collections::HashMap,
    fs,
};

enum Direction {
    Left = 0,
    Right = 1,
    Down = 2,
    Up = 3,
    Forward = 4,
    Back = 5,
}

fn get_total_surface(cubes: &HashMap<(i32, i32, i32), i32>) -> i32 {
    let mut total_surface = 0;
    for (_, surface) in cubes {
        total_surface += surface;
    }
    return total_surface;
}

fn get_coord_for_direction(pos: &(i32, i32, i32), dir: &Direction) -> i32 {
    match dir {
        Direction::Left | Direction::Right => return pos.0,
        Direction::Down | Direction::Up => return pos.1,
        Direction::Forward | Direction::Back => return pos.2,
    }
}

fn get_limit_for_direction(bounding_box: &(i32, i32, i32, i32, i32, i32), dir: &Direction) -> i32 {
    match dir {
        Direction::Left => return bounding_box.0,
        Direction::Right => return bounding_box.1,
        Direction::Down => return bounding_box.2,
        Direction::Up => return bounding_box.3,
        Direction::Forward => return bounding_box.4,
        Direction::Back => return bounding_box.5,
    }
}

fn is_outside_bounding_box_in_direction(
    pos: &(i32, i32, i32),
    bounding_box: &(i32, i32, i32, i32, i32, i32),
    dir: &Direction,
) -> bool {
    let coord_in_direction = get_coord_for_direction(pos, dir);
    let limit_in_direction = get_limit_for_direction(bounding_box, dir);
    match dir {
        Direction::Left | Direction::Down | Direction::Forward => {
            // println!("is_outside_bounding_box_in_direction: {coord_in_direction} < {limit_in_direction}");
            return coord_in_direction < limit_in_direction;
        }
        Direction::Right | Direction::Up | Direction::Back => {
            // println!("is_outside_bounding_box_in_direction: {coord_in_direction} > {limit_in_direction}");
            return coord_in_direction > limit_in_direction;
        }
    }
}

fn get_bounding_box(cubes: &HashMap<(i32, i32, i32), i32>) -> (i32, i32, i32, i32, i32, i32) {
    let mut bounding_box = (i32::MAX, i32::MIN, i32::MAX, i32::MIN, i32::MAX, i32::MIN);
    for (pos, _) in cubes {
        bounding_box.0 = min(bounding_box.0, pos.0);
        bounding_box.1 = max(bounding_box.1, pos.0);
        bounding_box.2 = min(bounding_box.2, pos.1);
        bounding_box.3 = max(bounding_box.3, pos.1);
        bounding_box.4 = min(bounding_box.4, pos.2);
        bounding_box.5 = max(bounding_box.5, pos.2);
    }
    return bounding_box;
}

fn get_trapped_air(cubes: &HashMap<(i32, i32, i32), i32>) -> HashMap<(i32, i32, i32), i32> {
    let bounding_box = get_bounding_box(cubes);
    let (left, right, down, up, forward, back) = bounding_box;
    let mut trapped_air: HashMap<(i32, i32, i32), i32> = HashMap::new();
    for x in left..right + 1 {
        for y in down..up + 1 {
            for z in forward..back + 1 {
                let pos = (x, y, z);
                if cubes.contains_key(&pos) {
                    continue;
                }
                if is_trapped(&cubes, &pos, &bounding_box) {
                    insert_pos(&mut trapped_air, &pos);
                }
            }
        }
    }
    return trapped_air;
}

fn move_pos(pos: &(i32, i32, i32), dir: &Direction) -> (i32, i32, i32) {
    match dir {
        Direction::Left => return (pos.0 - 1, pos.1, pos.2),
        Direction::Right => return (pos.0 + 1, pos.1, pos.2),
        Direction::Down => return (pos.0, pos.1 - 1, pos.2),
        Direction::Up => return (pos.0, pos.1 + 1, pos.2),
        Direction::Forward => return (pos.0, pos.1, pos.2 - 1),
        Direction::Back => return (pos.0, pos.1, pos.2 + 1),
    }
}

fn is_reachable_in_direction(
    cubes: &HashMap<(i32, i32, i32), i32>,
    pos: &(i32, i32, i32),
    bounding_box: &(i32, i32, i32, i32, i32, i32),
    dir: &Direction,
) -> bool {
    let mut new_pos = pos.clone();
    loop {
        new_pos = move_pos(&new_pos, &dir);
        if cubes.contains_key(&new_pos) {
            return false;
        }
        if is_outside_bounding_box_in_direction(&new_pos, bounding_box, &dir) {
            // print_pos("is not trapped: ", pos);
            return true;
        }
    }
}

fn is_trapped(
    cubes: &HashMap<(i32, i32, i32), i32>,
    pos: &(i32, i32, i32),
    bounding_box: &(i32, i32, i32, i32, i32, i32),
) -> bool {
    for dir in [
        Direction::Left,
        Direction::Right,
        Direction::Down,
        Direction::Up,
        Direction::Forward,
        Direction::Back,
    ] {
        if is_reachable_in_direction(cubes, pos, bounding_box, &dir) {
            return false;
        }
    }
    // print_pos("is trapped: ", pos);
    return true;
}

fn neighbors(pos: &(i32, i32, i32)) -> Vec<(i32, i32, i32)> {
    let mut neighbors: Vec<(i32, i32, i32)> = Vec::new();
    for dir in [
        Direction::Left,
        Direction::Right,
        Direction::Down,
        Direction::Up,
        Direction::Forward,
        Direction::Back,
    ] {
        neighbors.push(move_pos(&pos, &dir));
    }
    return neighbors;
}

fn print_pos(name: &str, pos: &(i32, i32, i32)) {
    println!("{name}: ({},{},{})", pos.0, pos.1, pos.2);
}

fn insert_pos(cubes: &mut HashMap<(i32, i32, i32), i32>, pos: &(i32, i32, i32)) {
    let neighbors = neighbors(pos);
    let mut num_neighbors = 0;
    // print_pos("Pos: ", pos);
    for neighbor in neighbors {
        // print_pos("neighbor: ", neighbor);
        if cubes.contains_key(&neighbor) {
            let neighbor_hashvalue = cubes.get(&neighbor).unwrap();
            let new_neighbor_value = neighbor_hashvalue - 1;
            cubes.insert(neighbor, new_neighbor_value);
            num_neighbors += 1;
        }
    }
    cubes.insert(*pos, 6 - num_neighbors);
}

pub fn solver() {
    let input = fs::read_to_string("./src/day18/input.txt")
        .expect("Should have been able to read the file");
    let cube_positions: Vec<&str> = input.split("\r\n").collect();

    let mut cubes: HashMap<(i32, i32, i32), i32> = HashMap::new();

    for cube_pos in cube_positions {
        let position: Vec<i32> = cube_pos
            .split(",")
            .map(|coord| coord.parse::<i32>().unwrap())
            .collect();
        let pos = (position[0], position[1], position[2]);
        insert_pos(&mut cubes, &pos);
    }

    println!("Day18:");
    let total_surface = get_total_surface(&cubes);
    println!("Part one: Surface: {total_surface}");

    let trapped_air = get_trapped_air(&cubes);
    let trapped_air_surface = get_total_surface(&trapped_air);
    let total_surface_part_two = total_surface - trapped_air_surface;
    println!(
        "Part two: Exterior Surface trapped air approach: {}",
        total_surface_part_two
    );
}
