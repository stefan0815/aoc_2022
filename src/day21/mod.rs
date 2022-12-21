use std::{collections::HashMap, fs};

// fn print_vec(name: &str, vec: &Vec<i128>) {
//     print!("{name}: [");
//     for val in vec {
//         print!("{val},");
//     }
//     println!("]");
// }

fn apply_operator(left: i128, right: i128, operator: &str) -> i128 {
    match operator {
        "+" => return left + right,
        "-" => return left - right,
        "*" => return left * right,
        "/" => return left / right,
        _ => return 0,
    }
}

fn solve_part_one(
    number_monkeys_in: &HashMap<&str, i128>,
    expression_monkeys_in: &HashMap<&str, (&str, &str, &str)>,
) -> i128 {
    let mut number_monkeys = number_monkeys_in.clone();
    let mut expression_monkeys = expression_monkeys_in.clone();
    while !number_monkeys.contains_key("root") {
        let mut new_expression_monkeys: HashMap<&str, (&str, &str, &str)> = HashMap::new();
        for (monkey_name, (left_monkey, operator, right_right)) in &expression_monkeys {
            if number_monkeys.contains_key(left_monkey) && number_monkeys.contains_key(right_right)
            {
                let left = number_monkeys.get(left_monkey).unwrap();
                let right = number_monkeys.get(right_right).unwrap();
                let result = apply_operator(*left, *right, operator);
                number_monkeys.insert(monkey_name, result);
            } else {
                new_expression_monkeys.insert(monkey_name, (left_monkey, operator, right_right));
            }
        }
        expression_monkeys = new_expression_monkeys;
    }
    return *number_monkeys.get("root").unwrap();
}

fn solve_part_two(
    number_monkeys_in: &HashMap<&str, i128>,
    expression_monkeys_in: &HashMap<&str, (&str, &str, &str)>,
    _root: (&str, &str),
) -> i128 {
    let mut number_monkeys = number_monkeys_in.clone();
    let mut expression_monkeys = expression_monkeys_in.clone();
    number_monkeys.remove("humn");
    expression_monkeys.remove("root");
    let mut number_monkeys_len_old = number_monkeys.len();
    let mut expression_monkeys_len_old = expression_monkeys.len();

    loop {
        let mut new_expression_monkeys: HashMap<&str, (&str, &str, &str)> = HashMap::new();
        for (monkey_name, (left_monkey, operator, right_right)) in &expression_monkeys {
            if number_monkeys.contains_key(left_monkey) && number_monkeys.contains_key(right_right)
            {
                let left = number_monkeys.get(left_monkey).unwrap();
                let right = number_monkeys.get(right_right).unwrap();
                let result = apply_operator(*left, *right, operator);
                number_monkeys.insert(monkey_name, result);
            } else {
                new_expression_monkeys.insert(monkey_name, (left_monkey, operator, right_right));
            }
        }
        expression_monkeys = new_expression_monkeys;
        if number_monkeys_len_old == number_monkeys.len()
            && expression_monkeys_len_old == expression_monkeys.len()
        {
            break;
        }

        number_monkeys_len_old = number_monkeys.len();
        expression_monkeys_len_old = expression_monkeys.len();
    }

    return 0;
}

pub fn solver(_debug: bool) {
    let input = fs::read_to_string("./src/day21/input.txt")
        .expect("Should have been able to read the file");
    let monkeys_str: Vec<&str> = input.split("\r\n").collect();

    let mut number_monkeys: HashMap<&str, i128> = HashMap::new();
    let mut expression_monkeys: HashMap<&str, (&str, &str, &str)> = HashMap::new();
    let mut root: (&str, &str) = ("", "");
    for monkey_str in monkeys_str {
        let split: Vec<&str> = monkey_str.split(": ").collect();
        let monkey_name = split[0];
        let split1: Vec<&str> = split[1].split(" ").collect();
        if split1.len() == 1 {
            let monkey_number = split1[0].parse::<i128>().unwrap();
            number_monkeys.insert(monkey_name, monkey_number);
            continue;
        }
        if monkey_name == "root" {
            root = (split[0], split[2]);
        }
        expression_monkeys.insert(monkey_name, (split1[0], split1[1], split1[2]));
    }

    println!("Day21:");
    let root_yells = solve_part_one(&number_monkeys, &expression_monkeys);
    println!("Monkey named root yells: {root_yells}");

    let human_yells = solve_part_two(&number_monkeys, &expression_monkeys, root);
    println!("Human should yell: {human_yells}");
}
