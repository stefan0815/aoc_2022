use std::{fs};

fn calculate_scenic_scores(trees: &Vec<Vec<u32>>) -> Vec<Vec<u32>>{
    let rows =  trees.len();
    let cols =  trees.first().unwrap().len();
    let mut scenic_scores: Vec<Vec<u32>> = Vec::new();
    for _ in 0..rows{
        scenic_scores.push(vec![1; cols]);
    }
    for x in 0..rows {
        for y in 0..cols {
            if x == 0 || y == 0 || x == rows - 1  || y == cols - 1{
                scenic_scores[x][y] = 0;
                continue;
            }

            let mut num_visible_trees = 0;           
            for blocking_tree in (&trees[x][..y]).into_iter().rev() {
                num_visible_trees += 1;
                if blocking_tree >= &trees[x][y]{
                    break;
                }
            }    
            scenic_scores[x][y] *= num_visible_trees;

            num_visible_trees = 0;
            for blocking_tree in (&trees[x][y+1..]).into_iter() {
                num_visible_trees += 1;
                if blocking_tree >= &trees[x][y]{
                    break;
                }
            }
            scenic_scores[x][y] *= num_visible_trees;

            num_visible_trees = 0;
            for blocking_tree in (&trees[..x]).into_iter().rev() {
                num_visible_trees += 1;
                if blocking_tree[y] >= trees[x][y]{
                    break;
                }
            }
            scenic_scores[x][y] *= num_visible_trees;
            
            num_visible_trees = 0;
            for blocking_tree in (&trees[x+1..]).into_iter() {
                num_visible_trees += 1;
                if blocking_tree[y] >= trees[x][y]{
                    break;
                }
            }
            scenic_scores[x][y] *= num_visible_trees;
        }
    }

    return scenic_scores;
}

fn check_tree_visibility(trees: &mut Vec<Vec<u32>>)-> Vec<Vec<bool>>{
    let rows =  trees.len();
    let cols =  trees.first().unwrap().len();
    let mut visible: Vec<Vec<bool>> = Vec::new();
    for _ in 0..rows{
        visible.push(vec![false; cols]);
    }
    for x in 0..rows {
        let mut largest_tree_in_row: u32 = 0;
        for y in 0..cols {
            if trees[x][y] > largest_tree_in_row{
                largest_tree_in_row = trees[x][y];
                visible[x][y] = true;
            }
            if x == 0 || y == 0 || x == rows - 1  || y == cols - 1{
                visible[x][y] = true;
            }
        }
    }

    for x in 0..rows {
        let mut largest_tree_in_row: u32 = 0;
        for y in (0..cols).rev() {
            if trees[x][y] > largest_tree_in_row{
                largest_tree_in_row = trees[x][y];
                visible[x][y] = true;
            }
            if x == 0 || y == 0 || x == rows - 1  || y == cols - 1{
                visible[x][y] = true;
            }
        }
    }

    for y in 0..cols {
        let mut largest_tree_in_col: u32 = 0;
        for x in 0..rows {
            if trees[x][y] > largest_tree_in_col{
                largest_tree_in_col = trees[x][y];
                visible[x][y] = true;
            }
            if x == 0 || y == 0 || x == rows - 1  || y == cols - 1{
                visible[x][y] = true;
            }
        }
    }

    for y in 0..cols {
        let mut largest_tree_in_col: u32 = 0;
        for x in (0..rows).rev() {
            if trees[x][y] > largest_tree_in_col{
                largest_tree_in_col = trees[x][y];
                visible[x][y] = true;
            }
            if x == 0 || y == 0 || x == rows - 1  || y == cols - 1{
                visible[x][y] = true;
            }
        }
    }
    return visible;
}

pub fn solver() {
    let input =
        fs::read_to_string("./src/day8/input.txt").expect("Should have been able to read the file");
    let tree_rows_str: Vec<&str> = input.split("\r\n").collect();

    let mut trees: Vec<Vec<u32>> = Vec::new();
    for tree_row_str in tree_rows_str {
        let tree_row: Vec<u32> = tree_row_str
            .chars()
            .map(|tree| tree.to_digit(10).unwrap())
            .collect();
        trees.push(tree_row);
    }
 
    let visible = check_tree_visibility(&mut trees);
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
