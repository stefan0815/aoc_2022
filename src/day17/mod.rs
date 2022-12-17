use std::{cmp::max, collections::HashSet, fs};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos(i32, i32);

fn shape_one(height: i32) -> HashSet<Pos> {
    let mut shape: HashSet<Pos> = HashSet::new();
    shape.insert(Pos(2, height + 3));
    shape.insert(Pos(3, height + 3));
    shape.insert(Pos(4, height + 3));
    shape.insert(Pos(5, height + 3));
    return shape;
}

fn shape_two(height: i32) -> HashSet<Pos> {
    let mut shape: HashSet<Pos> = HashSet::new();
    shape.insert(Pos(3, height + 5));

    shape.insert(Pos(2, height + 4));
    shape.insert(Pos(3, height + 4));
    shape.insert(Pos(4, height + 4));

    shape.insert(Pos(3, height + 3));

    return shape;
}

fn shape_three(height: i32) -> HashSet<Pos> {
    let mut shape: HashSet<Pos> = HashSet::new();
    shape.insert(Pos(4, height + 5));

    shape.insert(Pos(4, height + 4));

    shape.insert(Pos(2, height + 3));
    shape.insert(Pos(3, height + 3));
    shape.insert(Pos(4, height + 3));

    return shape;
}

fn shape_four(height: i32) -> HashSet<Pos> {
    let mut shape: HashSet<Pos> = HashSet::new();
    shape.insert(Pos(2, height + 6));
    shape.insert(Pos(2, height + 5));
    shape.insert(Pos(2, height + 4));
    shape.insert(Pos(2, height + 3));

    return shape;
}

fn shape_five(height: i32) -> HashSet<Pos> {
    let mut shape: HashSet<Pos> = HashSet::new();
    shape.insert(Pos(2, height + 4));
    shape.insert(Pos(3, height + 4));
    shape.insert(Pos(2, height + 3));
    shape.insert(Pos(3, height + 3));

    return shape;
}

fn shape(shape_type: usize, height: i32) -> HashSet<Pos> {
    match shape_type {
        0 => return shape_one(height),
        1 => return shape_two(height),
        2 => return shape_three(height),
        3 => return shape_four(height),
        4 => return shape_five(height),
        _ => panic!("Shape not available"),
    }
}

fn move_rock_down(rock: &HashSet<Pos>) -> HashSet<Pos> {
    let mut new_rock: HashSet<Pos> = HashSet::new();
    for pos in rock {
        new_rock.insert(Pos(pos.0, pos.1 - 1));
    }
    return new_rock;
}

fn move_rock_left(rock: &HashSet<Pos>) -> HashSet<Pos> {
    let mut new_rock: HashSet<Pos> = HashSet::new();
    for pos in rock {
        new_rock.insert(Pos(pos.0 - 1, pos.1));
        if pos.0 - 1 < 0 {
            return rock.clone();
        }
    }
    return new_rock;
}

fn move_rock_right(rock: &HashSet<Pos>, width: usize) -> HashSet<Pos> {
    let mut new_rock: HashSet<Pos> = HashSet::new();
    for pos in rock {
        new_rock.insert(Pos(pos.0 + 1, pos.1));
        if pos.0 + 1 >= width as i32 {
            return rock.clone();
        }
    }
    return new_rock;
}

fn move_rock(rock: &HashSet<Pos>, jet: char, width: usize) -> HashSet<Pos> {
    match jet {
        '<' => return move_rock_left(rock),
        '>' => return move_rock_right(rock, width),
        _ => return rock.clone(),
    }
}

fn cave_to_hashset(cave: &Vec<usize>) -> HashSet<Pos> {
    let mut cave_hashset: HashSet<Pos> = HashSet::new();
    for (col, height) in cave.iter().enumerate() {
        cave_hashset.insert(Pos(col as i32, *height as i32));
    }
    return cave_hashset;
}

fn hashset_to_cave(cave_hashset: &HashSet<Pos>, width: usize) -> Vec<usize> {
    let mut cave: Vec<usize> = vec![0; width];

    for pos in cave_hashset {
        cave[pos.0 as usize] = pos.1 as usize;
    }
    return cave;
}

fn merge_rock_with_cave(
    cave_hashset: &HashSet<Pos>,
    rock_hashset: &HashSet<Pos>,
    width: usize,
) -> Vec<usize> {
    let mut cave: Vec<usize> = hashset_to_cave(cave_hashset, width);
    let rock: Vec<usize> = hashset_to_cave(rock_hashset, width);

    for i in 0..width {
        println!("Col: {i}, Cave: {}, Rock: {}", cave[i], rock[i]);
        cave[i] = max(cave[i], rock[i]);
    }

    return cave;
}

pub fn solver() {
    let input = fs::read_to_string("./src/day17/input.txt")
        .expect("Should have been able to read the file");
    let jet_pattern: Vec<char> = input.chars().collect();
    let width = 7;
    let mut cave: Vec<usize> = vec![0; width];

    let num_rocks = 1;
    let mut jet_index = 0;
    let mut height = 0;
    for i in 0..num_rocks {
        let mut rock = shape(i % 5, height as i32);
        let cave_hashset = cave_to_hashset(&cave);
        loop {
            let jet = jet_pattern[jet_index];
            jet_index = (jet_index + 1) % jet_pattern.len();
            rock = move_rock(&rock, jet, width);
            let rock_down = move_rock_down(&rock);
            let no_intersection = cave_hashset.intersection(&rock_down).count() == 0;
            if no_intersection {
                println!("no intersection");

                rock = rock_down;
            } else {
                println!("intersection");
                for pos in &rock{
                    println!("Pos: {},{}", pos.0,pos.1);
                }
                cave = merge_rock_with_cave(&cave_hashset, &rock, width);
                height = *cave.iter().max().unwrap();
                break;
            }
        }
    }
    println!("Day17:");
    println!("Part one: Cave height {height} after {num_rocks}");
}
