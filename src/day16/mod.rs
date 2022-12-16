use std::{collections::HashMap, fs};

#[derive(Clone, PartialEq, Eq, Hash)]
struct Valve {
    name: String,
    activated: bool,
    flow_rate: usize,
    tunnels: Vec<String>,
}

fn open_valve(valves: &mut HashMap<String, Valve>, valve: &String, time: usize) -> usize {
    let value = valves[valve].value(0, time - 1);
    valves.get_mut(valve).unwrap().activated = true;
    return value;
}

fn path_from_to(came_from: &HashMap<String, String>, from: &String, to: &String) -> Vec<String>{
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
        if self.activated {
            return 0;
        }
        let time_left_after_opening = time as i32 - distance as i32 - 1;
        if time_left_after_opening < 0 {
            return 0;
        }
        return time_left_after_opening as usize * self.flow_rate;
    }

    // fn distance(&self, valves: &HashMap<String, Valve>, to: &String) -> usize {
    //     let (cost_so_far, _) = a_star_search(valves, &self.name.to_string()/*, to*/);
    //     return cost_so_far[to];
    // }
}

fn find_best_valve(valves: &HashMap<String,Valve>, costs: &HashMap<String, usize>, time: usize) -> String{
    let mut best_value = 0;
    let mut best_valve: String = "".to_string();
    for cost in costs {
        if *cost.1 > time {
            continue;
        }
        let value = valves[cost.0].value(*cost.1, time);
        if value > best_value {
            best_value = value;
            best_valve = valves[cost.0].name.to_string();
            // println!("From {current} to {} distance is {} min, value: {}", cost.0, cost.1, value);
        }           
    }
    return best_valve
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

        println!("Name: {}, Flowrate: {}, Tunnels:", name, flow_rate);
        for tunnel in &tunnels_str {
            print!("{tunnel}, ");
            tunnels.push(tunnel.to_string());
        }
        println!();
        valves.insert(
            name.to_string(),
            Valve {
                name: name.to_string(),
                activated: false,
                flow_rate,
                tunnels,
            },
        );
    }

    let mut current = start.to_string();
    let mut time = 30;
    let mut total_pressure_released = 0;
    while time > 0 {
        let value_of_current = valves[&current].value(0, time - 1);
        println!("{value_of_current}");
        if value_of_current > 0 {
            total_pressure_released += open_valve(&mut valves, &current, time);
            time -= 1;
            continue;
        }
        let (costs, came_from) = a_star_search(&valves, &current);       
        let next_valve = find_best_valve(&valves, &costs, time - 1);
        if next_valve == ""{
            break;
        }

        let path = path_from_to(&came_from, &current, &next_valve);
        current = path[1..].first().unwrap().to_string();
        time -= 1;
    }

    println!("Day16:");
    println!("Greedy valve search: {total_pressure_released}");
}
