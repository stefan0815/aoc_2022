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

    let mut rope: Vec<Pos> = vec![Pos { x: 0, y: 0 }; 10];

    let mut hash_first_tail = HashSet::new();
    let mut hash_last_tail = HashSet::new();

    hash_first_tail.insert((0, 0));
    hash_last_tail.insert((0, 0));

    for command in commands {
        let split: Vec<&str> = command.split(" ").collect();
        let dir: &str = split[0];
        let dist: i32 = split[1].parse::<i32>().unwrap();
        for _ in 0..dist {
            move_head(&mut rope[0], dir);
            for i in 1..rope.len() {
                move_tail(&rope[i - 1].clone(), &mut rope[i]);
            }
            let last_tail = rope.last().unwrap();
            hash_first_tail.insert((rope[1].x, rope[1].y));
            hash_last_tail.insert((last_tail.x, last_tail.y));
        }
    }
    println!("Day9:");
    println!("Visited positions tail: {}", hash_first_tail.len());
    println!("Visited positions last tail: {}", hash_last_tail.len());
}
