use std::fs;

fn convert_action(action_string: &str) -> i32 {
    let mut action: i32 = 0;
    match action_string {
        "A" | "X" => action = 0,
        "B" | "Y" => action = 1,
        "C" | "Z" => action = 2,
        &_ => (),
    }
    return action;
}

fn evaluate_points(enemy_action: i32, my_action: i32) -> i32 {
    if my_action == (enemy_action + 1) % 3 {
        return 6;
    }
    if my_action == enemy_action {
        return 3;
    }
    return 0;
}

// A or X Rock
// B or Y Paper
// C or Z Scissor
fn evaluate_match_part1(enemy_action_string: &str, my_action_string: &str) -> i32 {
    let enemy_action: i32 = convert_action(enemy_action_string);
    let my_action: i32 = convert_action(my_action_string);
    return my_action + 1 + evaluate_points(enemy_action, my_action);
}

// A Rock
// B Paper
// C Scissor
// X lose
// Y draw
// Z win
fn evaluate_match_part2(enemy_action_string: &str, result: &str) -> i32 {
    let enemy_action: i32 = convert_action(enemy_action_string);
    let mut my_action: i32 = 0;

    match result {
        "X" => my_action = (enemy_action + 2) % 3,
        "Y" => my_action = enemy_action,
        "Z" => my_action = (enemy_action + 1) % 3,
        &_ => (),
    }

    return my_action + 1 + evaluate_points(enemy_action, my_action);
}

pub fn solver() {
    let input =
        fs::read_to_string("./src/day2/input.txt").expect("Should have been able to read the file");
    let matches: Vec<&str> = input.split("\r\n").collect();
    let mut points_part1: i32 = 0;
    let mut points_part2: i32 = 0;
    for one_match in matches {
        let split: Vec<&str> = one_match.split(" ").collect();
        let first_col = split[0];
        let second_col = split[1];
        points_part1 += evaluate_match_part1(first_col, second_col);
        points_part2 += evaluate_match_part2(first_col, second_col);
    }
    println!("Day2:");
    println!("Points with first strategy: {points_part1}");
    println!("Points with ultimate strategy: {points_part2}");
}
