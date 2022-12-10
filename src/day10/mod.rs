use std::fs;

fn increase_cycle(crt: &mut Vec<Vec<&str>>, signals: &mut Vec<i32>, cycle: &mut usize, x: i32) {
    let line = *cycle / 40;
    let row_cycle = (*cycle % 40) as i32;
    println!("Line: {}, Cycle: {}, x: {}, Mult: {}", line, *cycle, x, *cycle as i32 * x);
    if row_cycle - 1 <= x && x <= row_cycle + 1 {
        crt[line][row_cycle as usize] = "#";
    }

    *cycle += 1;
    if (*cycle + 20) % 40 == 0 {
        println!("Cycle: {}, x: {}, Mult: {}", *cycle, x, *cycle as i32 * x);
        signals.push(*cycle as i32 * x);
    }
}

pub fn solver() {
    let input = fs::read_to_string("./src/day10/input.txt")
        .expect("Should have been able to read the file");
    let commands: Vec<&str> = input.split("\r\n").collect();

    let mut cycle: usize = 0;
    let mut x: i32 = 1;
    let mut signals: Vec<i32> = Vec::new();
    let mut crt: Vec<Vec<&str>> = Vec::new();
    crt.push(vec!["."; 40]);
    crt.push(vec!["."; 40]);
    crt.push(vec!["."; 40]);
    crt.push(vec!["."; 40]);
    crt.push(vec!["."; 40]);
    crt.push(vec!["."; 40]);

    for command in commands {
        let split: Vec<&str> = command.split(" ").collect();
        let op: &str = split[0];
        if op == "noop" {
            increase_cycle(&mut crt, &mut signals, &mut cycle, x);
            continue;
        }

        let add: i32 = split[1].parse::<i32>().unwrap();
        for _ in 0..2 {
            increase_cycle(&mut crt, &mut signals, &mut cycle, x);
        }
        x += add;
    }

    let sum_part_one: i32 = signals.iter().sum();
    println!("Day10:");
    println!("Sum of first 6 signals: {}", sum_part_one);
    for line in 0..crt.len() {
        for symbol in &crt[line] {
            print!("{}", symbol);
        }
        println!();
    }
}
