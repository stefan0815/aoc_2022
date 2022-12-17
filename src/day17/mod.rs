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
        0 => return shape_one(height + 1),
        1 => return shape_two(height + 1),
        2 => return shape_three(height + 1),
        3 => return shape_four(height + 1),
        4 => return shape_five(height + 1),
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

// fn hashset_to_cave(cave_hashset: &HashSet<Pos>, width: usize) -> Vec<usize> {
//     let mut cave: Vec<usize> = vec![0; width];

//     for pos in cave_hashset {
//         cave[pos.0 as usize] = max(cave[pos.0 as usize], pos.1 as usize);
//     }
//     return cave;
// }

// fn merge_rock_with_cave(
//     cave_hashset: &HashSet<Pos>,
//     rock_hashset: &HashSet<Pos>,
//     width: usize,
// ) -> Vec<usize> {
//     let mut cave: Vec<usize> = hashset_to_cave(cave_hashset, width);
//     let rock: Vec<usize> = hashset_to_cave(rock_hashset, width);

//     for i in 0..width {
//         // println!("Col: {i}, Cave: {}, Rock: {}", cave[i], rock[i]);
//         cave[i] = max(cave[i], rock[i]);
//     }

//     return cave;
// }

fn merge_rock_with_cave(cave_hashset: &HashSet<Pos>, rock_hashset: &HashSet<Pos>) -> HashSet<Pos> {
    return HashSet::from_iter(cave_hashset.union(&rock_hashset).cloned());
}

fn check_collision(cave_hashset: &HashSet<Pos>, rock_hashset: &HashSet<Pos>) -> bool {
    return cave_hashset.intersection(&rock_hashset).count() != 0;
}

fn get_cave_height(cave_hashset: &HashSet<Pos>) -> usize {
    let mut height = 0;
    for pos in cave_hashset {
        if pos.1 > height {
            height = pos.1;
        }
    }
    return height as usize;
}

fn render_hash(cave_hashset: &HashSet<Pos>, width: usize) {
    let height = get_cave_height(&cave_hashset);
    for y in (0..height + 1).rev() {
        for x in 0..width {
            if cave_hashset.contains(&Pos(x as i32,y as i32)){
                print!("#");
            }else {
                print!(".");
            }
        }
        println!();
    }
}

pub fn solver() {
    let input = fs::read_to_string("./src/day17/input.txt")
        .expect("Should have been able to read the file");
    let jet_pattern: Vec<char> = input.chars().collect();
    let width = 7;
    let cave: Vec<usize> = vec![0; width];
    let mut cave_hashset = cave_to_hashset(&cave);
    let num_rocks = 2022;
    let mut jet_index = 0;
    let mut height = 0;
    for i in 0..num_rocks {
        println!("rock: {i}, height: {height}");
        let mut rock = shape(i % 5, height as i32);
        // println!("new Rock Shape:{}: ", i%5);
        // for pos in &rock{
        //     println!("Pos: {},{}", pos.0,pos.1);
        // }
        loop {
            let jet = jet_pattern[jet_index];
            jet_index = (jet_index + 1) % jet_pattern.len();
            // println!("Move: {jet}");
            let rock_move = move_rock(&rock, jet, width);
            let collision = check_collision(&cave_hashset, &rock_move);
            if !collision {
                rock = rock_move;
            }
            let rock_down = move_rock_down(&rock);
            let collision = check_collision(&cave_hashset, &rock_down);
            if collision {
                cave_hashset = merge_rock_with_cave(&cave_hashset, &rock);
                height = get_cave_height(&cave_hashset);
                break;
            }
            rock = rock_down;
        }
    }
    // render_hash(&cave_hashset, width);
    println!("Day17:");
    println!("Part one: Cave height {height} after {num_rocks}");
}
