use std::{
    cmp::{max, min},
    collections::HashMap,
    fs,
};

fn get_total_surface(cubes: &HashMap<(i32, i32, i32), i32>) -> i32 {
    let mut total_surface = 0;
    for (_, surface) in cubes {
        total_surface += surface;
    }
    return total_surface;
}

fn get_limits(cubes: &HashMap<(i32, i32, i32), i32>) -> ((i32,i32), (i32,i32), (i32,i32)) {
    let mut limit_x = (i32::MAX, i32::MIN);
    let mut limit_y = (i32::MAX, i32::MIN);
    let mut limit_z = (i32::MAX, i32::MIN);
    for (pos, _) in cubes {
        limit_x.0 = min(limit_x.0, pos.0);
        limit_x.1 = max(limit_x.1, pos.0);
        limit_y.0 = min(limit_y.0, pos.1);
        limit_y.1 = max(limit_y.1, pos.1);
        limit_z.0 = min(limit_z.0, pos.2);
        limit_z.1 = max(limit_z.1, pos.2);
    }
    return (limit_x, limit_y, limit_z);
}

fn get_trapped_air(
    cubes: &HashMap<(i32, i32, i32), i32>,
    limit: i32,
) -> HashMap<(i32, i32, i32), i32> {
    let (limit_x, limit_y, limit_z) = get_limits(cubes);
    let mut trapped_air: HashMap<(i32, i32, i32), i32> = HashMap::new();
    for x in limit_x.0..limit_x.1 + 1 {
        for y in limit_y.0..limit_y.1 + 1 {
            for z in limit_z.0..limit_z.1 + 1 {
                let pos = (x, y, z);
                if cubes.contains_key(&pos) {
                    continue;
                }
                if !is_reachable(&cubes, &pos, limit) {
                    insert_pos(&mut trapped_air, &pos);
                }
            }
        }
    }
    return trapped_air;
}

fn is_reachable(cubes: &HashMap<(i32, i32, i32), i32>, pos: &(i32, i32, i32), limit: i32) -> bool {
    let rays = ray_casting(pos, limit);
    for ray in rays {
        // print_pos("Ray: ", *ray.last().unwrap());
        if cubes.contains_key(&ray[0]) {
            continue;
        }
        let mut ray_collision = false;
        for ray_pos in &ray[1..] {
            if cubes.contains_key(ray_pos) {
                ray_collision = true;
                break;
            }
        }
        if !ray_collision {
            return true;
        }
    }
    return false;
}



fn ray_casting(pos: &(i32, i32, i32), limit: i32) -> Vec<Vec<(i32, i32, i32)>> {
    let mut rays: Vec<Vec<(i32, i32, i32)>> = Vec::new();

    let mut ray_right: Vec<(i32, i32, i32)> = Vec::new();
    let mut ray_left: Vec<(i32, i32, i32)> = Vec::new();
    for x in 1..limit {
        ray_right.push((pos.0 + x, pos.1, pos.2));
        ray_left.push((pos.0 - x, pos.1, pos.2));
    }
    rays.push(ray_right);
    rays.push(ray_left);

    let mut ray_up: Vec<(i32, i32, i32)> = Vec::new();
    let mut ray_down: Vec<(i32, i32, i32)> = Vec::new();
    for y in 1..limit {
        ray_up.push((pos.0, pos.1 + y, pos.2));
        ray_down.push((pos.0, pos.1 - y, pos.2));
    }
    rays.push(ray_up);
    rays.push(ray_down);

    let mut ray_back: Vec<(i32, i32, i32)> = Vec::new();
    let mut ray_forward: Vec<(i32, i32, i32)> = Vec::new();
    for z in 1..limit {
        ray_back.push((pos.0, pos.1, pos.2 + z));
        ray_forward.push((pos.0, pos.1, pos.2 - z));
    }
    rays.push(ray_back);
    rays.push(ray_forward);

    return rays;
}

fn neighbors(pos: (i32, i32, i32)) -> Vec<(i32, i32, i32)> {
    let mut neighbors: Vec<(i32, i32, i32)> = Vec::new();
    for x in [pos.0 - 1, pos.0 + 1] {
        neighbors.push((x, pos.1, pos.2));
    }
    for y in [pos.1 - 1, pos.1 + 1] {
        neighbors.push((pos.0, y, pos.2));
    }
    for z in [pos.2 - 1, pos.2 + 1] {
        neighbors.push((pos.0, pos.1, z));
    }

    return neighbors;
}

fn print_pos(name: &str, pos: (i32, i32, i32)) {
    println!("{name}: ({},{},{})", pos.0, pos.1, pos.2);
}

fn insert_pos(cubes: &mut HashMap<(i32, i32, i32), i32>, pos: &(i32, i32, i32)) {
    let neighbors = neighbors(*pos);
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

    let total_surface = get_total_surface(&cubes);

    let limit = 100;
    let trapped_air = get_trapped_air(&cubes, limit);
    let trapped_air_surface = get_total_surface(&trapped_air);

    let mut new_cubes = cubes.clone();
    for iter in 0..10 {
        let trapped = get_trapped_air(&new_cubes, limit);
        println!("{}", get_total_surface(&trapped));
        new_cubes = trapped;
    }
    let total_surface_part_two = total_surface - trapped_air_surface;
    println!("Day18:");
    println!("Part one: Surface: {total_surface}");
    println!("Part two: Exterior Surface: {}", total_surface_part_two);
}
