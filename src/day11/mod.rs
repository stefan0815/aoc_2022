use std::fs;

struct Monkey {
    items: Vec<usize>,
    worry_increase: String,
    test: usize,
    throw_true: usize,
    throw_false: usize,
    inspected_items: usize,
}

impl Clone for Monkey {
    fn clone(&self) -> Monkey {
        return Monkey {
            items: self.items.clone(),
            worry_increase: self.worry_increase.clone(),
            test: self.test,
            throw_true: self.throw_true,
            throw_false: self.throw_false,
            inspected_items: self.inspected_items,
        };
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    return gcd(b, a % b);
}

fn lcm(a: usize, b: usize) -> usize {
    return a * b / gcd(a, b);
}

fn apply_reduced_worry_increase(old: usize, worry_increase: &String) -> usize {
    return apply_worry_increase(old, worry_increase) / 3;
}

fn apply_worry_increase(old: usize, worry_increase: &String) -> usize {
    let split: Vec<&str> = worry_increase.split(" ").collect();
    let increase: usize;

    match split[1] {
        "old" => increase = old,
        &_ => increase = split[1].parse().unwrap(),
    }

    match split[0] {
        "+" => return old + increase,
        "*" => return old * increase,
        &_ => return old,
    }
}

fn simulate_monkeys(monkeys: &mut Vec<Monkey>, rounds: usize) {
    for _ in 0..rounds {
        for monkey_num in 0..monkeys.len() {
            let monkey = monkeys[monkey_num].clone();
            for item in &monkey.items {
                let new_item = apply_reduced_worry_increase(*item, &monkey.worry_increase);
                if new_item % monkey.test == 0 {
                    monkeys[monkey.throw_true].items.push(new_item);
                } else {
                    monkeys[monkey.throw_false].items.push(new_item);
                }
            }
            monkeys[monkey_num].inspected_items += monkeys[monkey_num].items.len();
            monkeys[monkey_num].items.clear();
        }
    }
}

fn simulate_monkeys_part_two(monkeys: &mut Vec<Monkey>, rounds: usize) {
    let mut lcm_monkey_test = 1;
    for monkey in monkeys.iter() {
        lcm_monkey_test = lcm(lcm_monkey_test, monkey.test);
    }

    for _ in 0..rounds {
        for monkey_num in 0..monkeys.len() {
            let monkey = monkeys[monkey_num].clone();
            for item in &monkey.items {
                let new_item =
                    apply_worry_increase(*item, &monkey.worry_increase) % lcm_monkey_test;
                if new_item % monkey.test == 0 {
                    monkeys[monkey.throw_true].items.push(new_item);
                } else {
                    monkeys[monkey.throw_false].items.push(new_item);
                }
            }
            monkeys[monkey_num].inspected_items += monkeys[monkey_num].items.len();
            monkeys[monkey_num].items.clear();
        }
    }
}

fn calculate_monkey_business(monkeys: Vec<Monkey>) -> usize {
    let mut inspected_items: Vec<usize> = monkeys
        .iter()
        .map(|monkey| monkey.inspected_items as usize)
        .collect();
    inspected_items.sort();
    inspected_items.reverse();
    return inspected_items[0] * inspected_items[1];
}

pub fn solver() {
    let input = fs::read_to_string("./src/day11/input.txt")
        .expect("Should have been able to read the file");
    let monkeys_str: Vec<&str> = input.split("\r\n\r\n").collect();

    let mut monkeys: Vec<Monkey> = Vec::new();
    for monkey_str in monkeys_str {
        let split: Vec<&str> = monkey_str.split("\r\n").collect();
        let monkey = Monkey {
            items: split[1][18..]
                .split(", ")
                .map(|item| item.parse::<usize>().unwrap())
                .collect(),
            worry_increase: split[2][23..].to_owned(),
            test: split[3][21..].parse().unwrap(),
            throw_true: split[4][29..].parse().unwrap(),
            throw_false: split[5][30..].parse().unwrap(),
            inspected_items: 0,
        };
        monkeys.push(monkey);
    }

    let mut monkeys_part_one = monkeys.clone();
    simulate_monkeys(&mut monkeys_part_one, 20);
    let solution_of_part_one = calculate_monkey_business(monkeys_part_one);

    let mut monkeys_part_two = monkeys.clone();
    simulate_monkeys_part_two(&mut monkeys_part_two, 10000);
    let solution_of_part_two = calculate_monkey_business(monkeys_part_two);

    println!("Day11:");
    println!("Solution of part one: {}", solution_of_part_one);
    println!("Solution of part two: {}", solution_of_part_two);
}
