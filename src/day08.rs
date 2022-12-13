use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn load_trees(path: &str) -> anyhow::Result<Vec<Vec<i32>>> {
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

#[derive(Debug)]
struct CrossnNeighbour {
    up: Vec<i32>,
    down: Vec<i32>,
    left: Vec<i32>,
    right: Vec<i32>,
}

fn get_cross_neighbour(trees: &[Vec<i32>], row: usize, col: usize) -> CrossnNeighbour {
    CrossnNeighbour {
        up: trees[0..row].iter().map(|r| r[col]).collect(),
        down: trees[row + 1..].iter().map(|r| r[col]).collect(),
        left: trees[row][0..col].to_vec(),
        right: trees[row][col + 1..].to_vec(),
    }
}

fn is_visible(tree: i32, other: i32) -> bool {
    other >= tree
}

fn tree_is_visible(tree: i32, others: Vec<i32>) -> bool {
    !others.iter().any(|t| is_visible(tree, *t))
}

fn calculate_scenic_score(size: i32, others: Vec<i32>) -> i32 {
    let mut cpt = 0;
    for other in others.iter() {
        cpt += 1;
        if is_visible(size, *other) {
            break;
        }
    }

    cpt
}

fn calculate_tree_scenic_score(tree: i32, cross_neighbour: CrossnNeighbour) -> i32 {
    let mut cross_neighbour = cross_neighbour;
    cross_neighbour.up.reverse();
    cross_neighbour.left.reverse();
    let mut cpt = 0;
    cpt += calculate_scenic_score(tree, cross_neighbour.up);
    cpt *= calculate_scenic_score(tree, cross_neighbour.left);
    cpt *= calculate_scenic_score(tree, cross_neighbour.down);
    cpt *= calculate_scenic_score(tree, cross_neighbour.right);

    cpt
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

            let cross_neighbour = get_cross_neighbour(trees, row, col);

            if tree_is_visible(*tree, cross_neighbour.up)
                || tree_is_visible(*tree, cross_neighbour.down)
                || tree_is_visible(*tree, cross_neighbour.left)
                || tree_is_visible(*tree, cross_neighbour.right)
            {
                count += 1;
                continue;
            }
        }
    }

    count
}

fn highest_scenic_score(trees: &[Vec<i32>]) -> i32 {
    let mut highest = 0;
    for (row, line) in trees.iter().enumerate() {
        for (col, tree) in line.iter().enumerate() {
            let cross_neighbour = get_cross_neighbour(trees, row, col);
            let score = calculate_tree_scenic_score(*tree, cross_neighbour);
            if score > highest {
                highest = score;
            }
        }
    }

    highest
}

pub fn day08() -> anyhow::Result<()> {
    let trees_v2 = load_trees("data/day08.txt")?;
    println!("Day 08 part 1 - {}", count_visible_trees(&trees_v2));
    println!("Day 08 part 2 - {}", highest_scenic_score(&trees_v2));

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn calculate_tree_scenic_score_with_sample() {
        let score = calculate_tree_scenic_score(
            5,
            CrossnNeighbour {
                up: vec![3],
                down: vec![4, 5, 3],
                left: vec![2, 5],
                right: vec![1, 2],
            },
        );

        assert_eq!(score, 4);

        let score = calculate_tree_scenic_score(
            5,
            CrossnNeighbour {
                up: vec![3, 5, 3],
                down: vec![3],
                left: vec![3, 3],
                right: vec![4, 9],
            },
        );

        assert_eq!(score, 8);
    }
}
