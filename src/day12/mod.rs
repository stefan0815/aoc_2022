extern crate pathfinding;
use pathfinding::astar;
use std::fs;
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos(i32, i32);

impl Pos {
    fn distance(&self, other: &Pos) -> usize {
        ((self.0 - other.0).abs() + (self.1 - other.1).abs()) as usize
    }
}

fn in_bounds(height_map: &Vec<Vec<usize>>, position: &Pos) -> bool {
    return 0 <= position.0
        && position.0 < height_map.len() as i32
        && 0 <= position.1
        && position.1 < height_map.first().unwrap().len() as i32;
}

fn reachable(height_map: &Vec<Vec<usize>>, from: &Pos, to: &Pos) -> bool {
    return height_map[from.0 as usize][from.1 as usize] + 1
        >= height_map[to.0 as usize][to.1 as usize];
}

fn neighbors(height_map: &Vec<Vec<usize>>, position: &Pos) -> Vec<(Pos,usize)> {
    let up = Pos(position.0, position.1 + 1);
    let down = Pos(position.0, position.1 - 1);
    let left = Pos(position.0 - 1, position.1);
    let right = Pos(position.0 + 1, position.1);
    let mut neighbors = vec![up, down, left, right];
    if (position.0 + position.1) % 2 == 0 {
        neighbors.reverse();
    }
    let filtered_neightbors: Vec<Pos> = neighbors.into_iter().filter(|neighbor| in_bounds(height_map, neighbor) && reachable(height_map, position, neighbor)).collect();
    return filtered_neightbors.into_iter().map(|neighbor| (neighbor,1)).collect();
}

fn convert_letter_to_height(letter: char) -> usize {
    match letter {
        'S' => return 0,
        'E' => return 25,
        _ => return letter as usize - 97,
    }
}

pub fn solver() {
    let input = fs::read_to_string("./src/day12/input.txt")
        .expect("Should have been able to read the file");
    let height_map_rows_str: Vec<&str> = input.split("\r\n").collect();
    let mut height_map: Vec<Vec<usize>> = Vec::new();
    let mut start: Pos = Pos(0, 0);
    let mut exit: Pos = Pos(0, 0);
    for (x, row) in height_map_rows_str.iter().enumerate() {
        let char_row: Vec<char> = row.chars().collect();
        let start_pos = char_row.iter().position(|l| *l == 'S');
        if !start_pos.is_none() {
            start = Pos(x as i32, start_pos.unwrap() as i32);
        }
        let exit_pos = char_row.iter().position(|l| *l == 'E');
        if !exit_pos.is_none() {
            exit = Pos(x as i32, exit_pos.unwrap() as i32);
        }
        let height_map_row: Vec<usize> = char_row
            .iter()
            .map(|l| convert_letter_to_height(*l))
            .collect();
        height_map.push(height_map_row)
    }

    println!("Start: ({},{})", start.0, start.1);
    println!("Exit: ({},{})", exit.0, exit.1);

    let (_, steps_part_one) = astar(
        &start,
        |p| neighbors(&height_map, p),
        |p| p.distance(&exit) / 3,
        |p| *p == exit,
    )
    .unwrap();

    let mut min_steps_part_two = usize::MAX;
    for (x, row) in height_map.iter().enumerate() {
        for y in row {
            if height_map[x][*y] == 0 {
                let (_, steps) = astar(
                    &Pos(x as i32,*y as i32),
                    |p| neighbors(&height_map, p),
                    |p| p.distance(&exit) / 3,
                    |p| *p == exit,
                )
                .unwrap();
                if steps < min_steps_part_two{
                    min_steps_part_two = steps;
                }
            }         
        }  
    }

    println!("Day12:");
    println!("Number of Steps: {}", steps_part_one);
    println!("Min Steps for part two: {}", min_steps_part_two);
}
