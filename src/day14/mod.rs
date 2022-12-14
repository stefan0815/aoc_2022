use std::{collections::HashMap, fs};

#[derive(Clone)]
enum Tile {
    STONE,
    SAND,
}

fn parse_stone_corner(stone_corner: &str) -> (i32, i32) {
    let split: Vec<&str> = stone_corner.split(",").collect();
    return (split[0].parse().unwrap(), split[1].parse().unwrap());
}

fn line_from_to(from: (i32, i32), to: (i32, i32)) -> Vec<(i32, i32)> {
    let mut line: Vec<(i32, i32)> = Vec::new();
    if from.0 == to.0 {
        if from.1 < to.1 {
            for y in from.1..to.1 + 1 {
                line.push((from.0, y))
            }
        } else {
            for y in to.1..from.1 + 1 {
                line.push((from.0, y))
            }
        }
    }
    if from.1 == to.1 {
        if from.0 < to.0 {
            for x in from.0..to.0 + 1 {
                line.push((x, from.1))
            }
        } else {
            for x in to.0..from.0 + 1 {
                line.push((x, from.1))
            }
        }
    }
    return line;
}

fn simulate_sand_part_one(stone_layout: &HashMap<(i32, i32), Tile>) -> HashMap<(i32, i32), Tile> {
    let mut layout = stone_layout.clone();
    let sand_spawner = (500, 0);
    loop {
        let mut sand = sand_spawner.clone();
        loop {
            if sand.1 >= 9999 {
                return layout;
            }
            if !layout.contains_key(&(sand.0, sand.1 + 1)) {
                sand = (sand.0, sand.1 + 1);
                continue;
            }
            if !layout.contains_key(&(sand.0 - 1, sand.1 + 1)) {
                sand = (sand.0 - 1, sand.1 + 1);
                continue;
            }
            if !layout.contains_key(&(sand.0 + 1, sand.1 + 1)) {
                sand = (sand.0 + 1, sand.1 + 1);
                continue;
            }
            layout.insert((sand.0, sand.1), Tile::SAND);
            break;
        }
    }
}

fn simulate_sand_part_two(
    stone_layout: &HashMap<(i32, i32), Tile>,
    floor: i32,
) -> HashMap<(i32, i32), Tile> {
    let mut layout = stone_layout.clone();
    let sand_spawner = (500, 0);
    loop {
        let mut sand = sand_spawner.clone();
        loop {
            if sand.1 + 1 == floor {
                layout.insert((sand.0, sand.1), Tile::SAND);
                break;
            }
            if !layout.contains_key(&(sand.0, sand.1 + 1)) {
                sand = (sand.0, sand.1 + 1);
                continue;
            }
            if !layout.contains_key(&(sand.0 - 1, sand.1 + 1)) {
                sand = (sand.0 - 1, sand.1 + 1);
                continue;
            }
            if !layout.contains_key(&(sand.0 + 1, sand.1 + 1)) {
                sand = (sand.0 + 1, sand.1 + 1);
                continue;
            }
            layout.insert((sand.0, sand.1), Tile::SAND);
            if sand == sand_spawner {
                return layout;
            }
            break;
        }
    }
}

pub fn solver() {
    let input = fs::read_to_string("./src/day14/input.txt")
        .expect("Should have been able to read the file");
    let stone_structures: Vec<&str> = input.split("\r\n").collect();

    let mut layout: HashMap<(i32, i32), Tile> = HashMap::new();
    let mut floor = 0;
    for stone_structure in stone_structures {
        let stone_corners_str: Vec<&str> = stone_structure.split(" -> ").collect();
        let stone_corners: Vec<(i32, i32)> = stone_corners_str
            .iter()
            .map(|stone_corner| parse_stone_corner(*stone_corner))
            .collect();
        for (index, to) in stone_corners[1..].iter().enumerate() {
            let from = stone_corners[index];
            let line = line_from_to(from, *to);
            for stone in line {
                layout.insert(stone, Tile::STONE);
                if stone.1 > floor {
                    floor = stone.1;
                }
            }
        }
    }
    floor += 2;

    let layout_part_one = simulate_sand_part_one(&layout);
    let sum_part_one = layout_part_one.keys().count() - layout.keys().count();

    let layout_part_two = simulate_sand_part_two(&layout, floor);
    let sum_part_two = layout_part_two.keys().count() - layout.keys().count();

    println!("Day14:");
    println!("Number of resting sand part one: {}", sum_part_one);
    println!("Number of resting sand part two: {} with floor: {}", sum_part_two, floor);
}
