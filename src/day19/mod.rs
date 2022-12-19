use std::{
    cmp::{max, min},
    fs,
};

fn possible_robots(
    blueprint: &[[usize; 3]; 4],
    production: &[usize; 4],
    production_limit: &[usize; 4],
) -> Vec<usize> {
    let mut possible_robots: Vec<usize> = Vec::new();
    for (robot_type, robot_cost) in blueprint.iter().enumerate() {
        if production_limit[robot_type] > 0
            && production[robot_type] >= production_limit[robot_type]
        {
            continue;
        }
        let mut buildable = true;
        for (material, cost) in robot_cost.iter().enumerate() {
            if *cost > 0 && production[material] == 0 {
                buildable = false;
                break;
            }
        }
        if buildable {
            possible_robots.push(robot_type);
        }
    }
    return possible_robots;
}

fn produce_resource(production: &[usize; 4], storage: &[usize; 4], time: usize) -> [usize; 4] {
    return [
        storage[0] + production[0] * time,
        storage[1] + production[1] * time,
        storage[2] + production[2] * time,
        storage[3] + production[3] * time,
    ];
}

fn build_robot(
    blueprint: &[[usize; 3]; 4],
    robot_type: usize,
    production: &mut [usize; 4],
    storage: &mut [usize; 4],
    time: &mut usize,
) -> bool {
    let cost = blueprint[robot_type];
    let mut max_time_needed = 0;
    for (material_type, material_cost) in cost.iter().enumerate() {
        if *material_cost <= storage[material_type] {
            continue;
        }
        let material_needed = material_cost - storage[material_type];
        let mut time_needed = material_needed / production[material_type];
        if material_needed % production[material_type] > 0 {
            time_needed += 1;
        }
        if time_needed > max_time_needed {
            max_time_needed = time_needed;
        }
    }

    if max_time_needed + 2 > *time {
        // return if robot would never produce any material (+1 build_time +1produce_time)
        return false;
    }
    // println!("robot_type: {robot_type}, storage: [{},{},{}], production: [{},{},{}], cost: [{},{},{}], {max_time_needed}", storage[0], storage[1], storage[2], production[0], production[1], production[2], cost[0], cost[1], cost[2]);
    // do not produce for max_time_needed + 1 to ensure that we have enough resources underflow would panic
    let mut new_storage = produce_resource(&production, &storage, max_time_needed);
    new_storage = [
        new_storage[0] - cost[0],
        new_storage[1] - cost[1],
        new_storage[2] - cost[2],
        new_storage[3],
    ];

    new_storage = produce_resource(&production, &new_storage, 1);
    *time -= max_time_needed + 1;
    *storage = new_storage;
    production[robot_type] += 1;
    return true;
}

fn get_best_build_permutations(
    blueprint: &[[usize; 3]; 4],
    production: &[usize; 4],
    storage: &[usize; 4],
    time: usize,
    production_limit: &[usize; 4],
    early_break_limit: usize,
    depth: usize,
) -> Vec<(usize, Vec<usize>)> {
    let mut all_possibilities: Vec<(usize, Vec<usize>)> = Vec::new();
    let new_geode_gain: usize = (1..time).sum();
    let max_geode_gain = new_geode_gain + (time * production[3]);
    // println!("{time}:{max_geode_gain}");
    if storage[3] + max_geode_gain <= early_break_limit {
        all_possibilities.push((storage[3] + max_geode_gain, vec![]));
        return all_possibilities;
    }
    let possible_robots = possible_robots(&blueprint, &production, &production_limit);
    for robot_type in &possible_robots {
        let mut local_production = production.clone();
        let mut local_storage = storage.clone();
        let mut local_time = time.clone();
        let successful = build_robot(
            &blueprint,
            *robot_type,
            &mut local_production,
            &mut local_storage,
            &mut local_time,
        );
        if !successful {
            local_storage = produce_resource(&local_production, &local_storage, local_time);
            all_possibilities.push((local_storage[3], vec![]));
            continue;
        }
        let local_possiblities = get_best_build_permutations(
            &blueprint,
            &local_production,
            &local_storage,
            local_time,
            production_limit,
            early_break_limit,
            depth + 1,
        );

        for local_possibility in local_possiblities {
            // let mut this_step = (local_storage[3], vec![*robot_type]);
            // this_step.0 += local_possibility.0;
            let mut this_step = (local_possibility.0, vec![*robot_type]);
            this_step.1.append(&mut local_possibility.1.clone());
            all_possibilities.push(this_step);
        }
    }
    return all_possibilities;
}

fn print_build_order(build_order: &Vec<usize>) {
    print!("    Build_order: [");
    for robot in build_order {
        print!("{robot},");
    }
    println!("]");
}

fn solve(blueprints: &Vec<[[usize; 3]; 4]>, time: usize, debug: bool) -> (usize, Vec<usize>) {
    let production: [usize; 4] = [1, 0, 0, 0];
    let storage: [usize; 4] = [0, 0, 0, 0];
    let mut quality_sum = 0;
    let mut geodes_per_blueprint: Vec<usize> = Vec::new();
    for (id, blueprint) in blueprints.iter().enumerate() {
        let mut max_costs = [0, 0, 0];
        for material_costs in blueprint {
            max_costs[0] = max(max_costs[0], material_costs[0]);
            max_costs[1] = max(max_costs[1], material_costs[1]);
            max_costs[2] = max(max_costs[2], material_costs[2]);
        }
        if debug {
            println!("Blueprint {}", id + 1);
        }
        let mut max_geode = 0;
        let mut max_build_order: Vec<usize> = Vec::new();
        for limit in [2, 5, 7, time] {
            let production_limit: [usize; 4] = [
                min(max_costs[0], limit),
                min(max_costs[1], limit),
                min(max_costs[2], limit),
                0,
            ];
            let mut all_permutations = get_best_build_permutations(
                &blueprint,
                &production,
                &storage,
                time,
                &production_limit,
                max_geode,
                0,
            );
            all_permutations.sort_by(|a, b| b.0.cmp(&a.0));
            // for permutation in &all_permutations{
            //     println!("{}", permutation.0);
            // }
            let (best_geode, best_build_order) = all_permutations.first().unwrap();
            if *best_geode > max_geode || max_geode == 0{
                max_geode = *best_geode;
                max_build_order = best_build_order.to_vec();
            }
            if debug {
                println!(
                    "    max_geode: {best_geode}, production_limit:[{},{},{}]",
                    production_limit[0], production_limit[1], production_limit[2]
                );
            }
        }
        let quality = (id + 1) * max_geode;
        if debug {
            println!(
                "Blueprint {}: max_geode: {max_geode}, quality: {quality}",
                id + 1
            );
            print_build_order(&max_build_order);
        }

        geodes_per_blueprint.push(max_geode);
        quality_sum += quality;
        // break;
    }
    return (quality_sum, geodes_per_blueprint);
}

fn get_cost_from_string(cost_as_string: String, debug: bool) -> [usize; 3] {
    let mut cost = [0, 0, 0];
    let cost_per_material = cost_as_string.split(" and ");

    for material_cost in cost_per_material {
        let split: Vec<&str> = material_cost.split(" ").collect();
        match split[1] {
            "ore" => cost[0] += split[0].parse::<usize>().unwrap(),
            "clay" => cost[1] += split[0].parse::<usize>().unwrap(),
            "obsidian" => cost[2] += split[0].parse::<usize>().unwrap(),
            _ => (),
        }
    }
    if debug {
        println!("{cost_as_string}");
        println!("[{},{},{}]", cost[0], cost[1], cost[2]);
    }
    return cost;
}

pub fn solver(debug: bool) {
    let input = fs::read_to_string("./src/day19/input.txt")
        .expect("Should have been able to read the file");
    let blueprints_string: Vec<&str> = input.split("\r\n").collect();

    let mut blueprints: Vec<[[usize; 3]; 4]> = Vec::new();

    for bluesprint_string in blueprints_string {
        let split1: Vec<&str> = bluesprint_string.split(": Each ore robot costs ").collect();
        let split2: Vec<&str> = split1[1].split(". Each clay robot costs ").collect();
        let split3: Vec<&str> = split2[1].split(". Each obsidian robot costs ").collect();
        let split4: Vec<&str> = split3[1].split(". Each geode robot costs ").collect();
        let ore_robot_cost: [usize; 3] = get_cost_from_string(split2[0].to_string(), debug);
        let clay_robot_cost: [usize; 3] = get_cost_from_string(split3[0].to_string(), debug);
        let obsidian_robot_cost: [usize; 3] = get_cost_from_string(split4[0].to_string(), debug);
        let geode_robot_cost: [usize; 3] =
            get_cost_from_string((split4[1][..split4[1].len() - 1]).to_string(), debug);
        blueprints.push([
            ore_robot_cost,
            clay_robot_cost,
            obsidian_robot_cost,
            geode_robot_cost,
        ]);
    }

    let (max_quality, _) = solve(&blueprints, 24, debug);

    let (_, max_geodes) = solve(&(blueprints[..3].to_vec()), 32, debug);
    let mut multiplication_of_max_geodes = 1;
    for max_geode in max_geodes {
        multiplication_of_max_geodes *= max_geode;
    }

    println!("Day19:");
    println!("Part one max quality: {max_quality}");
    println!("Part two multiplication of max geodes: {multiplication_of_max_geodes}");
}
