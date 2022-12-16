use std::{collections::HashMap, fs};

#[derive(Clone, PartialEq, Eq, Hash)]
struct Valve {
    name: String,
    activated: bool,
    flow_rate: usize,
    tunnels: Vec<String>,
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
        return (time - distance - 1) * self.flow_rate;
    }

    // fn distance(&self, valves: &HashMap<String, Valve>, to: &String) -> usize {
    //     let (cost_so_far, _) = a_star_search(valves, &self.name.to_string()/*, to*/);
    //     return cost_so_far[to];
    // }
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

    // let valve = valves.get("OS").unwrap();
    // println!("Distance from OS to KY: {}", valve.distance(&valves, &"KY".to_string()));
    let current = start.to_string();
    let mut time = 30;
    while time > 0 {
        let (cost, came_from) = a_star_search(&valves, &current);
        time -= 1;
    }

    println!("Day16:");
}
