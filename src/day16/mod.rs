use std::{collections::HashMap, fs};

#[derive(Clone, PartialEq, Eq, Hash)]
struct Valve {
    name: String,
    opened: bool,
    flow_rate: usize,
    tunnels: Vec<String>,
}

fn open_valve(valves: &mut HashMap<String, Valve>, valve: &String, time: &mut usize) -> usize {
    *time -= 1;
    let value = valves[valve].flow_rate * *time;
    valves.get_mut(valve).unwrap().opened = true;
    return value;
}

// fn path_from_to(came_from: &HashMap<String, String>, from: &String, to: &String) -> Vec<String> {
//     let mut current_node = to.to_string();
//     let mut path: Vec<String> = Vec::new();
//     loop {
//         path.push(current_node.to_string());
//         if current_node == *from {
//             path.reverse();
//             return path;
//         }
//         current_node = came_from[&current_node].to_string();
//     }
// }

fn a_star_search(
    valves: &HashMap<String, Valve>,
    start: &String, /*,
                    end: &String,*/
) -> (HashMap<String, usize>, HashMap<String, String>) {
    let mut frontier: Vec<Valve> = Vec::new();
    let mut cost_so_far: HashMap<String, usize> = HashMap::new();
    let mut came_from: HashMap<String, String> = HashMap::new();
    came_from.insert(start.to_string(), start.to_string());
    cost_so_far.insert(start.to_string(), 0);
    frontier.push(valves[start].clone());

    while !frontier.is_empty() {
        let current = frontier.pop().unwrap();
        let current_name = current.name.to_string();

        // if current_name == *end { //we do not have a priority queue as we can not estimate distance therfore all paths have to be searched
        //     break;
        // }

        for next in current.tunnels {
            let new_cost = cost_so_far.get(&current_name.to_string()).unwrap() + 1;
            if !cost_so_far.contains_key(&next) || new_cost < cost_so_far[&next] {
                cost_so_far.insert(next.to_string(), new_cost);
                frontier.push(valves[&next].clone());
                came_from.insert(next.to_string(), current_name.to_string());
            }
        }
    }

    return (cost_so_far, came_from);
}

impl Valve {
    fn value(&self, distance: usize, time: usize) -> usize {
        if self.opened {
            return 0;
        }
        let time_left_after_opening = time as i32 - distance as i32 - 1;
        if time_left_after_opening < 0 {
            return 0;
        }
        return time_left_after_opening as usize * self.flow_rate;
    }
}

fn get_remaining_valves_sorted(
    valves: &HashMap<String, Valve>,
    distances: &HashMap<String, usize>,
    time: usize,
    limit: usize,
) -> Vec<(String, usize)> {
    let mut valve_values: Vec<(String, usize)> = Vec::new();
    for (valve_name, valve) in valves {
        let distance = distances[valve_name];
        if valve.opened || valve.flow_rate == 0 || distance as i32 >= time as i32 {
            continue;
        }
        valve_values.push((valve_name.to_string(), valve.value(distance, time)));
        //distance
    }
    valve_values.sort_by(|a, b| a.1.cmp(&b.1));
    valve_values.reverse();
    if limit != 0 && valve_values.len() > limit {
        return valve_values[..limit].to_vec();
    }
    return valve_values;
}

fn get_available_pressure_release(valves: &HashMap<String, Valve>, time: usize) -> i32 {
    let mut accumulated_value = 0;
    for (_, valve) in valves {
        accumulated_value += valve.value(0, time);
    }
    return accumulated_value as i32;
}

fn solve_part_one_step(
    valves: &mut HashMap<String, Valve>,
    current: &String,
    time: usize,
    early_break_limit: i32,
    limit: usize,
    depth: usize,
) -> (usize, Vec<String>) {
    let releasable_pressure = get_available_pressure_release(&valves, time);
    // println!("{releasable_pressure}/{early_break_limit}");
    if releasable_pressure < early_break_limit {
        return (0, [].to_vec());
    }
    // println!("{}, {}", time, depth);
    let (distances, _) = a_star_search(&valves, &current);
    // for (new_node, dist) in &distances{
    //     println!("Distance from: {current} to {new_node} is {dist}min");
    // }
    let remaining_valves = get_remaining_valves_sorted(&valves, &distances, time, limit);
    let mut pressure_release_max = 0;
    let mut opened_valves_max: Vec<String> = Vec::new();
    for (new_valve, _) in &remaining_valves {
        let mut new_valves = valves.clone();
        let mut new_time = time - distances[&new_valve.to_string()];
        let pressure_valve = open_valve(&mut new_valves, new_valve, &mut new_time);
        // println!("Distance: {}, Open valve: {new_valve}, releasing pressure: {pressure}, new_time: {new_time}",  distances[&new_valve.to_string()]);
        let (pressure, mut opened_valves) = solve_part_one_step(
            &mut new_valves.clone(),
            &new_valve,
            new_time,
            early_break_limit - pressure_valve as i32,
            limit,
            depth + 1,
        );
        if pressure + pressure_valve > pressure_release_max {
            pressure_release_max = pressure + pressure_valve;
            opened_valves_max = Vec::new();
            opened_valves_max.push(new_valve.to_string());
            opened_valves_max.append(&mut opened_valves);
        }
    }

    return (pressure_release_max, opened_valves_max);
}

pub fn solver() {
    let input = fs::read_to_string("./src/day16/input.txt")
        .expect("Should have been able to read the file");
    let values_and_paths: Vec<&str> = input.split("\r\n").collect();

    let mut valves: HashMap<String, Valve> = HashMap::new();
    let mut start = "";
    for valvue_and_path in values_and_paths {
        let split: Vec<&str> = valvue_and_path[6..].split(" has flow rate=").collect();
        let name = split[0];
        if start == "" {
            start = name;
        }
        let mut split_at = "; tunnel leads to valve ";
        if split[1].contains("tunnels") {
            split_at = "; tunnels lead to valves "
        }
        let split_two: Vec<&str> = split[1].split(split_at).collect();
        let flow_rate: usize = split_two[0].parse().unwrap();
        let tunnels_str: Vec<&str> = split_two[1].split(", ").collect();
        let mut tunnels: Vec<String> = Vec::new();

        // println!("Name: {}, Flowrate: {}, Tunnels:", name, flow_rate);
        for tunnel in &tunnels_str {
            // print!("{tunnel}, ");
            tunnels.push(tunnel.to_string());
        }
        println!();
        valves.insert(
            name.to_string(),
            Valve {
                name: name.to_string(),
                opened: false,
                flow_rate,
                tunnels,
            },
        );
    }

    let time = 30;
    let mut early_break_limit = 0;
    let mut max_total_pressure_released = 0;
    for limit in [1, 3, 5, 7, 9, 0] {
        let (total_pressure_released, _) = solve_part_one_step(
            &mut valves,
            &start.to_string(),
            time,
            early_break_limit,
            limit,
            0,
        );
        if total_pressure_released > max_total_pressure_released {
            max_total_pressure_released = total_pressure_released;
            early_break_limit = total_pressure_released as i32;
        }
        println!("total_pressure_released: {total_pressure_released} with early_break_limit: {early_break_limit} and limit: {limit}");
        // for valve in &opened_valves {
        //     println!("Opened: {valve}");
        // }
    }

    println!("Day16:");
    println!("Recursive valve search: {max_total_pressure_released}");
}
