use std::fs;

enum CraneType {
    CrateMover9000,
    CrateMover9001,
}

// fn print_state(states: &Vec<Vec<&str>>) {
//     println!("");
//     for state in states {
//         for s in state {
//             print!("{s}");
//         }
//         println!("");
//     }
// }

fn print_top_row(states: &Vec<Vec<&str>>){
    for state in states {
        if let Some(val) = state.last() {
            print!("{}", val);
        }
    }
    println!("");
}

fn move_crates(states: &mut Vec<Vec<&str>>, actions: &Vec<&str>, crane_type: CraneType) {
    for action in actions {
        let action_split: Vec<&str> = action.split(" ").collect();
        let num: usize = action_split[1].parse().unwrap();
        let mut from: usize = action_split[3].parse().unwrap();
        let mut to: usize = action_split[5].parse().unwrap();
        from -= 1;
        to -= 1;
        let from_len = states[from].len();
        let mut move_elements: Vec<&str> = states[from].clone()[from_len - num..].to_vec();
        if matches!(crane_type, CraneType::CrateMover9000) {
            move_elements.reverse(); //remove for second part;
        }
        for move_element in move_elements {
            states[to].push(move_element);
            states[from].pop();
        }
    }
}
pub fn solver() {
    let input =
        fs::read_to_string("./src/day5/input.txt").expect("Should have been able to read the file");
    let states_and_actions: Vec<&str> = input.split("\r\n\r\n").collect();
    let actions_str = states_and_actions[1];
    let actions: Vec<&str> = actions_str.split("\r\n").collect();

    let mut states: Vec<Vec<&str>> = Vec::new();
    // states.push(["Z", "N"].to_vec());
    // states.push(["M", "C", "D"].to_vec());
    // states.push(["P"].to_vec());

    states.push(["R", "P", "C", "D", "B", "G"].to_vec());
    states.push(["H", "V", "G"].to_vec());
    states.push(["N", "S", "Q", "D", "J", "P", "M"].to_vec());
    states.push(["P", "S", "L", "G", "D", "C", "N", "M"].to_vec());
    states.push(["J", "B", "N", "C", "P", "F", "L", "S"].to_vec());
    states.push(["Q", "B", "D", "Z", "V", "G", "T", "S"].to_vec());
    states.push(["B", "Z", "M", "H", "F", "T", "Q"].to_vec());
    states.push(["C", "M", "D", "B", "F"].to_vec());
    states.push(["F", "C", "Q", "G"].to_vec());

    // print_state(&states);

    let mut states_first_part : Vec<Vec<&str>>= states.clone();
    let mut states_second_part: Vec<Vec<&str>> = states.clone();
    
    move_crates(&mut states_first_part, &actions, CraneType::CrateMover9000);
    move_crates(&mut states_second_part, &actions, CraneType::CrateMover9001);

    // print_state(&states_first_part);
    // print_state(&states_second_part);

    println!("Day5:");
    print!("First part: ");
    print_top_row(&states_first_part);
    print!("Second Part: ");
    print_top_row(&states_second_part);
}
