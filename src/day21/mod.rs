use std::{cmp::max, collections::HashMap, fs};


fn print_vec<T:std::fmt::Display>(name: String, vec: &Vec<T>) {
    print!("{name}: [");
    for val in vec {
        print!("{val},");
    }
    println!("]");
}

fn apply_operator(left: f64, right: f64, operator: String) -> f64 {
    match operator.as_str() {
        "+" => return left + right,
        "-" => return left - right,
        "*" => return left * right,
        "/" => return left / right,
        _ => return 0 as f64,
    }
}

fn apply(
    number_monkeys: &HashMap<String, f64>,
    expression_monkeys: &HashMap<String, (String, String, String)>,
) -> (
    HashMap<String, f64>,
    HashMap<String, (String, String, String)>,
) {
    let mut new_number_monkeys: HashMap<String, f64> = number_monkeys.clone();
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
    number_monkeys_in: &HashMap<String, f64>,
    expression_monkeys_in: &HashMap<String, (String, String, String)>,
) -> f64 {
    let mut number_monkeys = number_monkeys_in.clone();
    let mut expression_monkeys = expression_monkeys_in.clone();
    while !number_monkeys.contains_key("root") {
        (number_monkeys, expression_monkeys) = apply(&number_monkeys, &expression_monkeys);
    }
    return *number_monkeys.get("root").unwrap();
}

fn evaluate_root(
    number_monkeys_in: &HashMap<String, f64>,
    expression_monkeys_in: &HashMap<String, (String, String, String)>,
    root: &(String, String),
) -> (f64, f64) {
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
    number_monkeys_in: &HashMap<String, f64>,
    expression_monkeys_in: &HashMap<String, (String, String, String)>,
    root: &(String, String),
) -> (
    HashMap<String, f64>,
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
    number_monkeys_in: &HashMap<String, f64>,
    expression_monkeys_in: &HashMap<String, (String, String, String)>,
    root: &(String, String),
    num_ranges: i128,
    debug: bool,
) -> f64 {
    let mut number_monkeys = number_monkeys_in.clone();
    let mut expression_monkeys = expression_monkeys_in.clone();
    number_monkeys.remove("humn");
    expression_monkeys.remove("root");
    let (mut expanded_number_monkeys, expanded_expression_monkeys) =
        expand_root(&number_monkeys, &expression_monkeys, root);

    let mut range = (i64::MIN as i128, i64::MAX as i128);
    let num_increments = max(2, num_ranges - 1);
    loop {
        let mut metrics: Vec<f64> = Vec::new();
        let increment = max((range.1 - range.0) / num_increments, 1);
        let mut ranges: Vec<i128> = Vec::new();
        ranges.push(range.0 as i128);
        for i in 1..num_increments {
            ranges.push(range.0 + increment * i);
        }
        ranges.push(range.1 as i128);

        for human_yells in &ranges {
            expanded_number_monkeys.insert("humn".to_owned(), *human_yells  as f64);
            let (left, right) =
                evaluate_root(&expanded_number_monkeys, &expanded_expression_monkeys, root);
            if  left == right {
                return *human_yells as f64;
            }
            metrics.push(left - right);
        }
        if debug {
            print_vec("ranges: ".to_string(), &ranges);
            print_vec("metrics: ".to_string(), &metrics);
        }
        for i in 0..metrics.len() - 1 {
            if metrics[i].signum() != metrics[i + 1].signum() {
                range = (ranges[i], ranges[i + 1]);
                if debug {
                    println!(
                        "new range: [{}..{}] metric: [{},{}]",
                        range.0,
                        range.1,
                        metrics[i],
                        metrics[i + 1]
                    );
                }
                break;
            }
        }
    }
}

fn get_input(
    file: &str,
) -> (
    HashMap<String, f64>,
    HashMap<String, (String, String, String)>,
    (String, String),
) {
    let input = fs::read_to_string(file).expect("Should have been able to read the file");
    let monkeys_str: Vec<&str> = input.split("\r\n").collect();

    let mut number_monkeys: HashMap<String, f64> = HashMap::new();
    let mut expression_monkeys: HashMap<String, (String, String, String)> = HashMap::new();
    let mut root: (String, String) = ("".to_owned(), "".to_owned());
    for monkey_str in monkeys_str {
        let split: Vec<&str> = monkey_str.split(": ").collect();
        let monkey_name = split[0];
        let split1: Vec<&str> = split[1].split(" ").collect();
        if split1.len() == 1 {
            let monkey_number = split1[0].parse::<f64>().unwrap();
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
    return (number_monkeys, expression_monkeys, root);
}

pub fn solver(debug: bool) {
    let (number_monkeys, expression_monkeys, root) = get_input("./src/day21/input.txt");
    println!("Day21:");
    let root_yells = solve_part_one(&number_monkeys, &expression_monkeys);
    println!("Monkey named root yells: {root_yells}");

    let human_yells = solve_part_two(&number_monkeys, &expression_monkeys, &root, 5, debug);
    println!("Human should yell: {human_yells}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day21_solve_part_two_example() {
        let (number_monkeys, expression_monkeys, root) = get_input("./src/day21/example_input.txt");
        let human_yells = solve_part_two(&number_monkeys, &expression_monkeys, &root, 5, false);

        assert_eq!(301.0, human_yells);
    }

    #[test]
    fn day21_solve_part_two_10_ranges() {
        let (number_monkeys, expression_monkeys, root) = get_input("./src/day21/input.txt");
        let human_yells = solve_part_two(&number_monkeys, &expression_monkeys, &root, 10, false);

        assert_eq!(3469704905529.0, human_yells);
    }

    #[test]
    fn day21_check_part_two_solution() {
        let (mut number_monkeys, expression_monkeys, root) = get_input("./src/day21/input.txt");
        number_monkeys.insert("humn".to_owned(), 3469704905529.0);
        let (left_yells, right_yells) = evaluate_root(&number_monkeys, &expression_monkeys, &root);

        assert_eq!(24376746909942.0, left_yells);
        assert_eq!(24376746909942.0, right_yells);
        assert_eq!(left_yells, right_yells);
    }

    #[test]
    fn day21_check_part_two_integer_division_solution() {
        let (mut number_monkeys, expression_monkeys, root) = get_input("./src/day21/input.txt");
        number_monkeys.insert("humn".to_owned(), 3469704905531.0);
        let (left_yells, right_yells) = evaluate_root(&number_monkeys, &expression_monkeys, &root);

        assert_ne!(left_yells, right_yells);
    }
}
