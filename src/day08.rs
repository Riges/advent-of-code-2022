use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn load_trees_v2(path: &str) -> anyhow::Result<Vec<Vec<i32>>> {
    let mut trees: Vec<Vec<i32>> = vec![];
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let row: Vec<i32> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();

        trees.push(row);
    }

    Ok(trees)
}

fn tree_is_visible(tree: i32, others: Vec<i32>) -> bool {
    !others.iter().any(|t| *t >= tree)
}

fn count_visible_trees(trees: &Vec<Vec<i32>>) -> i32 {
    let mut count = 0;
    for (row, line) in trees.iter().enumerate() {
        if row == 0 || row == trees.len() - 1 {
            count += line.len() as i32;
            continue;
        }
        for (col, tree) in line.iter().enumerate() {
            if col == 0 || col == line.len() - 1 {
                count += 1;
                continue;
            }

            let previous_col: Vec<i32> = trees[row][0..col].to_vec();
            if tree_is_visible(*tree, previous_col) {
                count += 1;
                continue;
            }

            let next_col: Vec<i32> = trees[row][col + 1..].to_vec();
            if tree_is_visible(*tree, next_col) {
                count += 1;
                continue;
            }

            let previous_rows_in_same_col: Vec<i32> =
                trees[0..row].iter().map(|r| r[col]).collect();
            if tree_is_visible(*tree, previous_rows_in_same_col) {
                count += 1;
                continue;
            }

            let next_rows_in_same_col: Vec<i32> = trees[row + 1..].iter().map(|r| r[col]).collect();
            if tree_is_visible(*tree, next_rows_in_same_col) {
                count += 1;
                continue;
            }
        }
    }

    count
}

pub fn day08() -> anyhow::Result<()> {
    let trees_v2 = load_trees_v2("data/day08.txt")?;
    println!("Day 08 part 2 - {}", count_visible_trees(&trees_v2));

    Ok(())
}
