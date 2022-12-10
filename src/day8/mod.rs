extern crate ndarray;

use ndarray::prelude::*;
use std::fs;

fn calculate_scenic_score(trees: &Array2<u32>, x: usize, y: usize) -> usize {
    let rows = trees.rows().into_iter().count();
    let cols = trees.columns().into_iter().count();
    let mut scenic_score = 1;
    let visible_trees_left = trees
        .slice(s![x, ..y])
        .iter()
        .rev()
        .position(|tree_height| tree_height >= &trees[[x, y]]);

    if visible_trees_left.is_none() {
        scenic_score *= y;
    } else {
        scenic_score *= visible_trees_left.unwrap() + 1;
    }

    let visible_trees_right = trees
        .slice(s![x, y + 1..])
        .iter()
        .position(|tree_height| tree_height >= &trees[[x, y]]);

    if visible_trees_right.is_none() {
        scenic_score *= cols - y - 1;
    } else {
        scenic_score *= visible_trees_right.unwrap() + 1;
    }

    let visible_trees_up = trees
        .slice(s![..x, y])
        .iter()
        .rev()
        .position(|tree_height| tree_height >= &trees[[x, y]]);

    if visible_trees_up.is_none() {
        scenic_score *= x;
    } else {
        scenic_score *= visible_trees_up.unwrap() + 1;
    }

    let visible_trees_down = trees
        .slice(s![x + 1.., y])
        .iter()
        .position(|tree_height| tree_height >= &trees[[x, y]]);

    if visible_trees_down.is_none() {
        scenic_score *= rows - x - 1;
    } else {
        scenic_score *= visible_trees_down.unwrap() + 1;
    }
    return scenic_score;
}

fn calculate_scenic_scores_ndarray(trees: &Array2<u32>) -> Array2<usize> {
    let rows = trees.rows().into_iter().count();
    let cols = trees.columns().into_iter().count();
    let mut scenic_scores = Array2::<usize>::ones((rows, cols));
    for x in 1..rows - 1 {
        for y in 1..cols - 1 {
            scenic_scores[[x, y]] = calculate_scenic_score(trees, x, y);
        }
    }
    return scenic_scores;
}

fn check_tree_visibility_ndarray(trees: &Array2<u32>) -> Array2<u32> {
    let rows = trees.rows().into_iter().count();
    let cols = trees.columns().into_iter().count();
    let mut visible: Array2<u32> = Array2::<u32>::zeros((rows, cols));

    for x in 0..rows {
        visible[[x, 0]] = 0;
        visible[[x, cols - 1]] = 0;
    }
    for y in 0..cols {
        visible[[0, y]] = 0;
        visible[[rows - 1, y]] = 0;
    }

    for x in 1..rows - 1 {
        let mut largest_tree_left = trees[[x, 0]];
        let mut largest_tree_right = trees[[x, cols - 1]];
        for y in 1..cols - 1 {
            if trees[[x, y]] > largest_tree_left {
                visible[[x, y]] = 1;
                largest_tree_left = trees[[x, y]];
            }
        }
        for y in (1..cols - 1).rev() {
            if trees[[x, y]] > largest_tree_right {
                visible[[x, y]] = 1;
                largest_tree_right = trees[[x, y]];
            }
        }
    }

    for y in 1..cols - 1 {
        let mut largest_tree_up = trees[[0, y]];
        let mut largest_tree_down = trees[[cols - 1, y]];
        for x in 1..rows - 1 {
            if trees[[x, y]] > largest_tree_up {
                visible[[x, y]] = 1;
                largest_tree_up = trees[[x, y]];
            }
        }
        for x in (1..rows - 1).rev() {
            if trees[[x, y]] > largest_tree_down {
                visible[[x, y]] = 1;
                largest_tree_down = trees[[x, y]];
            }
        }
    }

    // for x in 1..rows - 1 {
    //     for y in 1..cols - 1 {
    //         let slice_left = trees.slice(s![x, ..y]);
    //         let slice_right = trees.slice(s![x, y + 1..]);
    //         let slice_up = trees.slice(s![..x, y]);
    //         let slice_down = trees.slice(s![x + 1.., y]);
    //         let slices = [slice_left, slice_right, slice_up, slice_down];
    //         let largest_trees = slices.map(|slice| *slice.iter().max().unwrap());
    //         visible[[x, y]] = largest_trees
    //             .iter()
    //             .any(|largest_tree| trees[[x, y]] > *largest_tree)
    //             as u32;
    //     }
    // }

    return visible;
}

pub fn solver() {
    let input =
        fs::read_to_string("./src/day8/input.txt").expect("Should have been able to read the file");
    let tree_rows_str: Vec<&str> = input.split("\r\n").collect();
    let rows = tree_rows_str.len();
    let cols = tree_rows_str.first().unwrap().chars().count();
    let mut trees_array: Array2<u32> = Array2::<u32>::zeros((rows, cols));
    let mut trees: Vec<Vec<u32>> = Vec::new();
    let mut row = 0;
    for mut tree_row in trees_array.rows_mut() {
        let tree_row_vec: Vec<u32> = tree_rows_str[row]
            .chars()
            .map(|tree| tree.to_digit(10).unwrap())
            .collect();
        for col in 0..cols {
            tree_row[col] = tree_row_vec[col];
        }
        trees.push(tree_row_vec);
        row += 1;
    }

    let visible = check_tree_visibility_ndarray(&trees_array);
    let sum_part_one = visible.iter().filter(|&v| *v != 0).count();

    let scenic_scores = calculate_scenic_scores_ndarray(&trees_array);
    let max_part_two = scenic_scores.iter().max().unwrap();

    println!("Day8:");
    println!("Visible trees: {}", sum_part_one);
    println!("Highest Scenic Score: {}", max_part_two);
}
