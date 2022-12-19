use std::{
    cmp::max,
    collections::{HashMap, VecDeque},
    fs,
};

fn get_best_build_permutations(
    blueprint: &[[usize; 3]; 4],
    production: &[usize; 4],
    storage: &[usize; 4],
    time: usize,
    early_break_limit: usize,
    depth: usize,
) -> Vec<(usize, Vec<usize>)> {
    let mut all_possibilities: Vec<(usize, Vec<usize>)> = Vec::new();
    // let possible_robots = possible_robots(&blueprint, &production, &storage, time);
    // for robot in &possible_robots {

    // }
    return all_possibilities;
}

fn get_cost_from_string(cost_as_string: String) -> [usize; 3] {
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
    println!("{cost_as_string}");
    println!("[{},{},{}]", cost[0],cost[1],cost[2]);
    return cost;
}

pub fn solver() {
    let input = fs::read_to_string("./src/day19/input.txt")
        .expect("Should have been able to read the file");
    let blueprints_string: Vec<&str> = input.split("\r\n").collect();

    let mut blueprints: Vec<[[usize; 3]; 4]> = Vec::new();

    for bluesprint_string in blueprints_string {
        let split1: Vec<&str> = bluesprint_string.split(": Each ore robot costs ").collect();
        let split2: Vec<&str> = split1[1].split(". Each clay robot costs ").collect();
        let split3: Vec<&str> = split2[1].split(". Each obsidian robot costs ").collect();
        let split4: Vec<&str> = split3[1].split(". Each geode robot costs ").collect();
        let ore_robot_cost: [usize; 3] = get_cost_from_string(split2[0].to_string());
        let clay_robot_cost: [usize; 3] = get_cost_from_string(split3[0].to_string());
        let obsidian_robot_cost: [usize; 3] = get_cost_from_string(split4[0].to_string());
        let geode_robot_cost: [usize; 3] =
            get_cost_from_string((split4[1][..split4[1].len() - 1]).to_string());
        blueprints.push([
            ore_robot_cost,
            clay_robot_cost,
            obsidian_robot_cost,
            geode_robot_cost,
        ]);
    }
    println!("Day19:");

    let mut max_quality = 0;
    for (id, blueprint) in blueprints.iter().enumerate() {
        let production: [usize; 4] = [1, 0, 0, 0];
        let storage: [usize; 4] = [0, 0, 0, 0];
        let time = 24;
        let early_break_limit = 0;
        let mut all_permutations =
            get_best_build_permutations(&blueprint, &production, &storage, time, early_break_limit, 0);
        all_permutations.sort_by(|a,b| b.0.cmp(&a.0));
        let max_geode = all_permutations.first().unwrap().0;
        let quality = id * max_geode;
        if quality > max_quality {
            max_quality = quality;
        }
    }
    println!("Max Quality: {max_quality}");

}
