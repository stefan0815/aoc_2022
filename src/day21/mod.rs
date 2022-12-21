use std::{collections::HashMap, fs, cmp::max};

// fn print_vec(name: String, vec: &Vec<i128>) {
//     print!("{name}: [");
//     for val in vec {
//         print!("{val},");
//     }
//     println!("]");
// }

fn apply_operator(left: i128, right: i128, operator: String) -> i128 {
    match operator.as_str() {
        "+" => return left + right,
        "-" => return left - right,
        "*" => return left * right,
        "/" => return left / right,
        _ => return 0,
    }
}

fn apply(
    number_monkeys: &HashMap<String, i128>,
    expression_monkeys: &HashMap<String, (String, String, String)>,
) -> (
    HashMap<String, i128>,
    HashMap<String, (String, String, String)>,
) {
    let mut new_number_monkeys: HashMap<String, i128> = number_monkeys.clone();
    let mut new_expression_monkeys: HashMap<String, (String, String, String)> = HashMap::new();
    for (monkey_name, (left_monkey, operator, right_right)) in expression_monkeys {
        if new_number_monkeys.contains_key(left_monkey)
            && new_number_monkeys.contains_key(right_right)
        {
            let left = new_number_monkeys.get(left_monkey).unwrap();
            let right = new_number_monkeys.get(right_right).unwrap();
            let result = apply_operator(*left, *right, operator.to_string());
            new_number_monkeys.insert(monkey_name.to_string(), result);
        } else {
            new_expression_monkeys.insert(
                monkey_name.to_string(),
                (
                    left_monkey.to_string(),
                    operator.to_string(),
                    right_right.to_string(),
                ),
            );
        }
    }
    return (new_number_monkeys, new_expression_monkeys);
}

fn solve_part_one(
    number_monkeys_in: &HashMap<String, i128>,
    expression_monkeys_in: &HashMap<String, (String, String, String)>,
) -> i128 {
    let mut number_monkeys = number_monkeys_in.clone();
    let mut expression_monkeys = expression_monkeys_in.clone();
    while !number_monkeys.contains_key("root") {
        (number_monkeys, expression_monkeys) = apply(&number_monkeys, &expression_monkeys);
    }
    return *number_monkeys.get("root").unwrap();
}

fn evaluate_root(
    number_monkeys_in: &HashMap<String, i128>,
    expression_monkeys_in: &HashMap<String, (String, String, String)>,
    root: &(String, String),
) -> (i128, i128) {
    let mut number_monkeys = number_monkeys_in.clone();
    let mut expression_monkeys = expression_monkeys_in.clone();
    while !number_monkeys.contains_key(&root.0) || !number_monkeys.contains_key(&root.1) {
        (number_monkeys, expression_monkeys) = apply(&number_monkeys, &expression_monkeys);
    }
    return (
        *number_monkeys.get(&root.0).unwrap(),
        *number_monkeys.get(&root.1).unwrap(),
    );
}

fn expand_root(
    number_monkeys_in: &HashMap<String, i128>,
    expression_monkeys_in: &HashMap<String, (String, String, String)>,
    root: &(String, String),
) -> (
    HashMap<String, i128>,
    HashMap<String, (String, String, String)>,
) {
    let mut number_monkeys = number_monkeys_in.clone();
    let mut expression_monkeys = expression_monkeys_in.clone();
    while !number_monkeys.contains_key(&root.0) && !number_monkeys.contains_key(&root.1) {
        (number_monkeys, expression_monkeys) = apply(&number_monkeys, &expression_monkeys);
    }

    return (number_monkeys, expression_monkeys);
}

fn solve_part_two(
    number_monkeys_in: &HashMap<String, i128>,
    expression_monkeys_in: &HashMap<String, (String, String, String)>,
    root: &(String, String),
) -> i128 {
    let mut number_monkeys = number_monkeys_in.clone();
    let mut expression_monkeys = expression_monkeys_in.clone();
    number_monkeys.remove("humn");
    expression_monkeys.remove("root");
    let (mut expanded_number_monkeys, expanded_expression_monkeys) =
        expand_root(&number_monkeys, &expression_monkeys, root);

    let mut range = (i64::MIN as i128, i64::MAX as i128);
    loop {
        let mut metrics: Vec<i128> = Vec::new();
        let increment = max((range.1 - range.0) / 10, 1);
        let mut ranges: Vec<i128> = Vec::new();
        ranges.push(range.0);
        for i in 1..10 {
            ranges.push(range.0 + increment * i);
        }
        ranges.push(range.1);

        for human_yells in &ranges {
            expanded_number_monkeys.insert("humn".to_owned(), *human_yells);
            let (left, right) =
                evaluate_root(&expanded_number_monkeys, &expanded_expression_monkeys, root);
            // println!("human_yells: {human_yells}: {left} == {right}");
            if left == right {
                return *human_yells;
            }
            metrics.push(left - right);
        }
        for i in 0..metrics.len() - 1 {
            if metrics[i].signum() != metrics[i + 1].signum() {
                range = (ranges[i], ranges[i + 1]);
                println!(
                    "range: [{}..{}] metric: [{},{}]",
                    range.0, range.1, metrics[i], metrics[i + 1]
                );
                break;
            }
        }
    }
}

pub fn solver(_debug: bool) {
    let input = fs::read_to_string("./src/day21/input.txt")
        .expect("Should have been able to read the file");
    let monkeys_str: Vec<&str> = input.split("\r\n").collect();

    let mut number_monkeys: HashMap<String, i128> = HashMap::new();
    let mut expression_monkeys: HashMap<String, (String, String, String)> = HashMap::new();
    let mut root: (String, String) = ("".to_owned(), "".to_owned());
    for monkey_str in monkeys_str {
        let split: Vec<&str> = monkey_str.split(": ").collect();
        let monkey_name = split[0];
        let split1: Vec<&str> = split[1].split(" ").collect();
        if split1.len() == 1 {
            let monkey_number = split1[0].parse::<i128>().unwrap();
            number_monkeys.insert(monkey_name.to_string(), monkey_number);
            continue;
        }
        if monkey_name == "root" {
            root = (split1[0].to_string(), split1[2].to_string());
        }
        expression_monkeys.insert(
            monkey_name.to_string(),
            (
                split1[0].to_string(),
                split1[1].to_string(),
                split1[2].to_string(),
            ),
        );
    }

    println!("Day21:");
    let root_yells = solve_part_one(&number_monkeys, &expression_monkeys);
    println!("Monkey named root yells: {root_yells}");

    let human_yells = solve_part_two(&number_monkeys, &expression_monkeys, &root);
    println!("Human should yell: {human_yells}");
}
