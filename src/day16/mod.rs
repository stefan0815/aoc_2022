use std::{
    cmp::max,
    collections::{HashMap, VecDeque},
    fs,
};

#[derive(Clone, PartialEq, Eq, Hash)]
struct Valve {
    name: String,
    opened: bool,
    flow_rate: usize,
    tunnels: Vec<String>,
}

fn open_valve(valves: &mut HashMap<String, Valve>, valve: &String, time: &mut usize) -> usize {
    if valves[valve].opened {
        return 0;
    }
    // println!("Open valve: {valve}");
    *time -= 1;
    let value = valves[valve].flow_rate * *time;
    valves.get_mut(valve).unwrap().opened = true;
    return value;
}

fn path_from_to(came_from: &HashMap<String, String>, from: &String, to: &String) -> Vec<String> {
    let mut current_node = to.to_string();
    let mut path: Vec<String> = Vec::new();
    loop {
        path.push(current_node.to_string());
        if current_node == *from {
            path.reverse();
            return path;
        }
        current_node = came_from[&current_node].to_string();
    }
}

fn a_star_search(
    valves: &HashMap<String, Valve>,
    start: &String,
) -> (HashMap<String, usize>, HashMap<String, String>) {
    let mut frontier: VecDeque<Valve> = VecDeque::new();
    let mut cost_so_far: HashMap<String, usize> = HashMap::new();
    let mut came_from: HashMap<String, String> = HashMap::new();
    came_from.insert(start.to_string(), start.to_string());
    cost_so_far.insert(start.to_string(), 0);
    frontier.push_back(valves[start].clone());

    while !frontier.is_empty() {
        let current = frontier.pop_front().unwrap();
        let current_name = current.name.to_string();

        for next in current.tunnels {
            let new_cost = cost_so_far.get(&current_name.to_string()).unwrap() + 1;
            if !cost_so_far.contains_key(&next) || new_cost < cost_so_far[&next] {
                cost_so_far.insert(next.to_string(), new_cost);
                frontier.push_back(valves[&next].clone());
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
        if valve.opened || valve.flow_rate == 0 || distance as i32 + 1 >= time as i32 {
            continue;
        }
        valve_values.push((
            valve_name.to_string(),
            valve.value(distance, time) / distance,
        ));
    }
    valve_values.sort_by(|a, b| a.1.cmp(&b.1));
    valve_values.reverse();
    if limit != 0 && valve_values.len() > limit {
        return valve_values[..limit].to_vec();
    }
    return valve_values;
}

fn get_available_pressure_release(valves: &HashMap<String, Valve>, time: usize) -> usize {
    let mut accumulated_value = 0;
    for (_, valve) in valves {
        accumulated_value += valve.value(0, time);
    }
    return accumulated_value;
}

fn solve_part_one_step(
    valves: &HashMap<String, Valve>,
    current: &String,
    time: usize,
    early_break_limit: usize,
    limit: usize,
    depth: usize,
) -> (usize, Vec<String>) {
    let releasable_pressure = get_available_pressure_release(&valves, time);
    // println!("{releasable_pressure}/{early_break_limit}");
    if releasable_pressure <= early_break_limit {
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
        // let path_from_to = path_from_to(&came_from, &current, &new_valve);
        // println!("From {current} to {new_valve} distance: {}", distances[&new_valve.to_string()]);
        // for node in path_from_to{
        //     print!("{node},");
        // }
        // println!();
        // println!("Distance: {}, Open valve: {new_valve}, releasing pressure: {pressure}, new_time: {new_time}",  distances[&new_valve.to_string()]);
        let (pressure, mut opened_valves) = solve_part_one_step(
            &mut new_valves.clone(),
            &new_valve.to_string(),
            new_time.clone(),
            max(0, early_break_limit as i32 - pressure_valve as i32) as usize,
            limit,
            depth + 1,
        );
        if pressure + pressure_valve > pressure_release_max {
            pressure_release_max = pressure + pressure_valve;
            opened_valves_max.clear();
            opened_valves_max.push(new_valve.to_string());
            opened_valves_max.append(&mut opened_valves);
        }
    }

    return (pressure_release_max, opened_valves_max);
}

fn solve_part_two_step(
    valves: &HashMap<String, Valve>,
    current: &String,
    current_elephant: &String,
    time: usize,
    time_elephant: usize,
    early_break_limit: usize,
    limit: usize,
    depth: usize,
) -> (usize, usize, Vec<String>, Vec<String>) {
    let releasable_pressure = get_available_pressure_release(&valves, time);
    if releasable_pressure <= early_break_limit {
        return (0, 0, [].to_vec(), [].to_vec());
    }

    let (distances, _) = a_star_search(&valves, &current);
    let (distances_elephant, _) = a_star_search(&valves, &current_elephant);

    // let distance_between = distances[current_elephant];
    // if distance_between < ((26 - time) + (26 - time_elephant)) / 2 {
    //     return (0, 0, [].to_vec(), [].to_vec());
    // }

    let remaining_valves = get_remaining_valves_sorted(&valves, &distances, time, limit);
    let remaining_valves_elephant =
        get_remaining_valves_sorted(&valves, &distances_elephant, time_elephant, limit);
    let mut pressure_release_max = 0;
    let mut pressure_release_max_me = 0;
    let mut pressure_release_max_elephant = 0;
    let mut opened_valves_max: Vec<String> = Vec::new();
    let mut opened_valves_elephant_max: Vec<String> = Vec::new();
    for (new_valve, _) in &remaining_valves {
        for (new_valve_elephant, _) in &remaining_valves_elephant {
            if new_valve == new_valve_elephant {
                continue;
            }

            let mut new_valves = valves.clone();

            let mut new_time = time - distances[&new_valve.to_string()];
            let mut new_time_elephant =
                time_elephant - distances_elephant[&new_valve_elephant.to_string()];

            let pressure_valve = open_valve(&mut new_valves, new_valve, &mut new_time);
            let pressure_valve_elephant =
                open_valve(&mut new_valves, new_valve_elephant, &mut new_time_elephant);
            let (pressure, pressure_elephant, mut opened_valves, mut opened_valves_elephant) =
                solve_part_two_step(
                    &mut new_valves.clone(),
                    &new_valve.to_string(),
                    &new_valve_elephant.to_string(),
                    new_time.clone(),
                    new_time_elephant.clone(),
                    max(
                        0,
                        early_break_limit as i32
                            - (pressure_valve + pressure_valve_elephant) as i32,
                    ) as usize,
                    limit,
                    depth + 1,
                );
            if pressure + pressure_elephant + pressure_valve + pressure_valve_elephant
                > pressure_release_max
            {
                pressure_release_max =
                    pressure + pressure_elephant + pressure_valve + pressure_valve_elephant;
                pressure_release_max_me = pressure + pressure_valve;
                pressure_release_max_elephant = pressure_elephant + pressure_valve_elephant;
                opened_valves_max.clear();
                opened_valves_max.push(new_valve.to_string());
                opened_valves_max.append(&mut opened_valves);
                opened_valves_elephant_max.clear();
                opened_valves_elephant_max.push(new_valve_elephant.to_string());
                opened_valves_elephant_max.append(&mut opened_valves_elephant);
            }
        }
    }

    // if (pressure_release_max_me as i32 - pressure_release_max_elephant as i32).abs() as usize > (pressure_release_max_me + pressure_release_max_elephant) / 2{
    //     return (0, 0, [].to_vec(), [].to_vec());
    // }

    return (
        pressure_release_max_me,
        pressure_release_max_elephant,
        opened_valves_max,
        opened_valves_elephant_max,
    );
}

fn solve_part_one(valves: &HashMap<String, Valve>) -> usize {
    let start = "AA".to_string();
    let time = 30;
    let mut early_break_limit = 0;
    let mut max_total_pressure_released = 0;
    for limit in [1, 3, 7, 0] {
        let (total_pressure_released, _) = solve_part_one_step(
            &mut valves.clone(),
            &start.to_string(),
            time,
            early_break_limit,
            limit,
            0,
        );
        if total_pressure_released > max_total_pressure_released {
            max_total_pressure_released = total_pressure_released;
            early_break_limit = total_pressure_released;
        }
        println!("total_pressure_released: {total_pressure_released} with early_break_limit: {early_break_limit} and limit: {limit}");
        // for valve in &opened_valves {
        //     println!("Opened: {valve}");
        // }
    }
    return max_total_pressure_released;
}

fn solve_part_two(valves: &HashMap<String, Valve>) -> usize {
    let start = "AA".to_string();
    let time = 26;
    let mut early_break_limit = 0;
    let mut max_total_pressure_released = 0;
    for limit in [2, 3, 5, 7, 0] {
        let (total_pressure_released, total_pressure_released_elephant, _, _) = solve_part_two_step(
            &mut valves.clone(),
            &start.to_string(),
            &start.to_string(),
            time,
            time,
            early_break_limit,
            limit,
            0,
        );
        if total_pressure_released + total_pressure_released_elephant > max_total_pressure_released {
            max_total_pressure_released = total_pressure_released + total_pressure_released_elephant;
            early_break_limit = total_pressure_released + total_pressure_released_elephant;
        }
        println!("total_pressure_released: {max_total_pressure_released} with early_break_limit: {early_break_limit} and limit: {limit}");
        // for valve in &opened_valves {
        //     println!("Opened: {valve}");
        // }
    }
    return max_total_pressure_released;
}

fn find_best_valve(
    valves: &HashMap<String, Valve>,
    distances: &HashMap<String, usize>,
    time: usize,
) -> String {
    let mut best_heuristic = 0;
    let mut best_valve: String = "".to_string();
    for (name, distance) in distances {
        let valve = &valves[name];
        if valve.flow_rate == 0 || valve.opened || *distance + 1 >= time {
            continue;
        }
        let value = valve.value(*distance, time);
        if value == 0 {
            continue;
        }

        let heuristic = max(
            valve.value(0, time) / ((*distance + 1) * (*distance + 1)),
            1,
        );
        println!(
            "To {} distance is {} min, value: {}, heuristic: {}",
            name, distance, value, heuristic
        );

        if heuristic > best_heuristic {
            best_heuristic = heuristic;
            best_valve = valve.name.to_string();
        }
    }
    return best_valve;
}

fn solve_part_one_heuristic(valves: &mut HashMap<String, Valve>) -> usize {
    let mut current = "AA".to_string();
    let mut time = 30;
    let mut total_pressure_released = 0;
    let mut next_valve = current.to_string();
    while time > 0 {
        let (distances, came_from) = a_star_search(&valves, &current);
        if current == next_valve {
            next_valve = find_best_valve(&valves, &distances, time);
        }
        println!("{} -> {}", current, next_valve);

        if next_valve == "" {
            total_pressure_released += open_valve(valves, &current, &mut time);
            break;
        }

        let distance = distances[&next_valve];
        let valve = &valves[&current];
        let value_of_current = valve.value(0, time);
        if value_of_current > 0
            && (current == next_valve
                || valve.flow_rate * (2 * distance + 1) >= valves[&next_valve].flow_rate)
        {
            total_pressure_released += open_valve(valves, &current, &mut time);
            continue;
        }

        let path = path_from_to(&came_from, &current, &next_valve);
        if path.len() < 2 {
            break;
        }
        current = (&path[1]).to_string();
        time -= 1;
    }
    return total_pressure_released;
}

pub fn solver() {
    let input = fs::read_to_string("./src/day16/input.txt")
        .expect("Should have been able to read the file");
    let values_and_paths: Vec<&str> = input.split("\r\n").collect();

    let mut valves: HashMap<String, Valve> = HashMap::new();

    for valvue_and_path in values_and_paths {
        let split: Vec<&str> = valvue_and_path[6..].split(" has flow rate=").collect();
        let name = split[0];
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
        // println!();
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
    println!("Day16:");

    // let max_total_pressure_released_part_one_heuristic =
    //     solve_part_one_heuristic(&mut valves.clone());
    // println!("Heuristic valve search part one: {max_total_pressure_released_part_one_heuristic}");

    // let max_total_pressure_released_part_one = solve_part_one(&valves.clone());
    // println!("Recursive valve search part one: {max_total_pressure_released_part_one}");

    let max_total_pressure_released_part_two = solve_part_two(&valves.clone());
    println!("Recursive valve search part two: {max_total_pressure_released_part_two}");
}
