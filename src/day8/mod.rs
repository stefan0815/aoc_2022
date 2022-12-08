extern crate ndarray;

use ndarray::prelude::*;
use std::fs;

fn calculate_scenic_scores(trees: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let rows = trees.len();
    let cols = trees.first().unwrap().len();
    let mut scenic_scores: Vec<Vec<u32>> = Vec::new();
    for _ in 0..rows {
        scenic_scores.push(vec![1; cols]);
    }
    for x in 0..rows {
        for y in 0..cols {
            if x == 0 || y == 0 || x == rows - 1 || y == cols - 1 {
                scenic_scores[x][y] = 0;
                continue;
            }

            let mut num_visible_trees = 0;
            for blocking_tree in (&trees[x][..y]).into_iter().rev() {
                num_visible_trees += 1;
                if blocking_tree >= &trees[x][y] {
                    break;
                }
            }
            scenic_scores[x][y] *= num_visible_trees;

            num_visible_trees = 0;
            for blocking_tree in (&trees[x][y + 1..]).into_iter() {
                num_visible_trees += 1;
                if blocking_tree >= &trees[x][y] {
                    break;
                }
            }
            scenic_scores[x][y] *= num_visible_trees;

            num_visible_trees = 0;
            for blocking_tree in (&trees[..x]).into_iter().rev() {
                num_visible_trees += 1;
                if blocking_tree[y] >= trees[x][y] {
                    break;
                }
            }
            scenic_scores[x][y] *= num_visible_trees;

            num_visible_trees = 0;
            for blocking_tree in (&trees[x + 1..]).into_iter() {
                num_visible_trees += 1;
                if blocking_tree[y] >= trees[x][y] {
                    break;
                }
            }
            scenic_scores[x][y] *= num_visible_trees;
        }
    }

    return scenic_scores;
}

fn check_tree_visibility_ndarray(trees: &Array2<u32>) -> Vec<Vec<bool>> {
    let rows = trees.rows().into_iter().count();
    let cols = trees.columns().into_iter().count();
    let mut visible: Vec<Vec<bool>> = Vec::new();
    for _ in 0..rows {
        visible.push(vec![true; cols]);
    }

    for x in 1..rows - 1 {
        for y in 1..cols - 1 {
            let slice_left = trees.slice(s![x, ..y]);
            let slice_right = trees.slice(s![x, y + 1..]);
            let slice_up = trees.slice(s![..x, y]);
            let slice_down = trees.slice(s![x + 1.., y]);
            let slices = [slice_left, slice_right, slice_up, slice_down];
            let largest_trees = slices.map(|slice| *slice.iter().max().unwrap());
            visible[x][y] = largest_trees
                .iter()
                .any(|largest_tree| trees[[x, y]] > *largest_tree);
        }
    }

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
    let mut sum_part_one = 0;
    for visible_row in visible {
        sum_part_one += visible_row.iter().filter(|&v| *v).count();
    }

    let scenic_scores = calculate_scenic_scores(&trees);
    let mut max_part_two = 0 as u32;
    for scenic_score_row in scenic_scores {
        let max_value = scenic_score_row.iter().max().unwrap();
        if *max_value > max_part_two {
            max_part_two = *max_value;
        }
    }
    println!("Day8:");
    println!("Visible trees: {}", sum_part_one);
    println!("Highest Scenic Score: {}", max_part_two);
}
