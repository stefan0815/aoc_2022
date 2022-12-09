use std::{collections::HashSet, fs};

struct Pos {
    x: i32,
    y: i32,
}

fn move_head(head: &mut Pos, dir: &str) {
    match dir {
        "R" => head.x += 1,
        "L" => head.x -= 1,
        "U" => head.y += 1,
        "D" => head.y -= 1,
        _ => (),
    }
}

fn move_tail(head: &Pos, tail: &mut Pos) {
    let x_diff = head.x - tail.x;
    let y_diff = head.y - tail.y;
    if x_diff.abs() <= 1 && y_diff.abs() <= 1 {
        return;
    }

    if x_diff.abs() == 2 {
        tail.x += x_diff / 2;
        if y_diff.abs() == 1 {
            tail.y = head.y;
        }
    }

    if y_diff.abs() == 2 {
        tail.y += y_diff / 2;
        if x_diff.abs() == 1 {
            tail.x = head.x;
        }
    }
}

impl Clone for Pos {
    fn clone(&self) -> Pos {
        return Pos {
            x: self.x,
            y: self.y,
        };
    }
}

pub fn solver() {
    let input =
        fs::read_to_string("./src/day9/input.txt").expect("Should have been able to read the file");
    let commands: Vec<&str> = input.split("\r\n").collect();

    let mut head: Pos = Pos { x: 0, y: 0 };
    let mut tails: Vec<Pos> = vec![head.clone(); 9];

    let mut hash_first_tail = HashSet::new();
    let mut hash_last_tail = HashSet::new();

    hash_first_tail.insert((head.x, head.y));
    hash_last_tail.insert((head.x, head.y));

    for command in commands {
        let split: Vec<&str> = command.split(" ").collect();
        let dir: &str = split[0];
        let dist: i32 = split[1].parse::<i32>().unwrap();
        for _ in 0..dist {
            move_head(&mut head, dir);
            move_tail(&head, &mut tails[0]);
            for i in 1..tails.len() {
                move_tail(&tails[i - 1].clone(), &mut tails[i]);
            }
            let first_tail = tails.first().unwrap();
            let last_tail = tails.first().unwrap();
            hash_first_tail.insert((first_tail.x, first_tail.y));
            hash_last_tail.insert((last_tail.x, last_tail.y));
        }
    }
    println!("Day9:");
    println!("Visited positions tail: {}", hash_first_tail.len());
    println!("Visited positions last tail: {}", hash_last_tail.len());
}
