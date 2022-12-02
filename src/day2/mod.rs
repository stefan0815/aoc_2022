use std::fs;

// A or X Rock
// B or Y Paper
// C or Z Scissor

fn evaluate_match_part1(enemy_action: &str, my_action: &str) -> i32{
    let mut enemy_action_int :i32 = 0;
    let mut my_action_int :i32 = 0;

    match enemy_action {
        "A" => enemy_action_int = 0,
        "B" => enemy_action_int = 1,
        "C" => enemy_action_int = 2,
        &_ => (),
    }
    match my_action {
        "X" => my_action_int = 0,
        "Y" => my_action_int = 1,
        "Z" => my_action_int = 2,
        &_ => (),
    }

    let mut points = my_action_int + 1;
   
    // println!("{enemy_action}: {enemy_action_int}");
    // println!("{my_action}: {my_action_int}");
    if my_action_int == (enemy_action_int + 1) % 3 {
        points += 6;
    } 
    else if my_action_int == enemy_action_int {
        points += 3;
    }
    return points;
}

// A Rock
// B Paper
// C Scissor
// X lose
// Y draw
// Z win
fn evaluate_match_part2(enemy_action: &str, result: &str) -> i32{
    let mut enemy_action_int :i32 = 0;
    let mut my_action_int :i32 = 0;

    match enemy_action {
        "A" => enemy_action_int = 0,
        "B" => enemy_action_int = 1,
        "C" => enemy_action_int = 2,
        &_ => (),
    }

    match result {
        "X" => my_action_int = (enemy_action_int + 2) % 3,
        "Y" => my_action_int = enemy_action_int,
        "Z" => my_action_int = (enemy_action_int + 1) % 3,
        &_ => (),
    }

    let mut points = my_action_int + 1;
   
    // println!("{enemy_action}: {enemy_action_int}");
    // println!("{my_action}: {my_action_int}");
    if my_action_int == (enemy_action_int + 1) % 3 {
        points += 6;
    } 
    else if my_action_int == enemy_action_int {
        points += 3;
    }
    return points;
}

pub fn solver() {
    let input = fs::read_to_string("./src/day2/input.txt")
    .expect("Should have been able to read the file");
    let matches : Vec<&str> = input.split("\r\n").collect();
    let mut points_part1 :i32 = 0;
    let mut points_part2 :i32 = 0;
    for one_match in matches{
        let split : Vec<&str> = one_match.split(" ").collect();
        let first_col = split[0];
        let second_col = split[1];
        points_part1 += evaluate_match_part1(first_col, second_col);
        points_part2 += evaluate_match_part2(first_col, second_col);
    }
    println!("{points_part1}");
    println!("{points_part2}");
}