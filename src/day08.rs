use std::fmt::Debug;
use std::{
    fmt::Formatter,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

#[derive(Clone)]
struct Point {
    col: i32,
    row: i32,
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(row : {}, Col :{})", self.row, self.col,)
    }
}

#[derive(Debug, Clone)]
struct Tree {
    position: Point,
    size: i32,
}

fn load_trees(path: &str) -> anyhow::Result<Vec<Tree>> {
    let mut trees: Vec<Tree> = vec![];
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    for (row, line) in reader.lines().enumerate() {
        for (col, char) in line?.chars().enumerate() {
            trees.push(Tree {
                position: Point {
                    col: (col as i32) + 1,
                    row: (row as i32) + 1,
                },
                size: char.to_digit(10).unwrap() as i32,
            });
        }
    }

    Ok(trees)
}

fn find_neighbors(
    trees: &Vec<Tree>,
    tree: &Tree,
) -> (Option<Tree>, Option<Tree>, Option<Tree>, Option<Tree>) {
    let mut neighbors: (Option<Tree>, Option<Tree>, Option<Tree>, Option<Tree>) =
        (None, None, None, None);
    for t in trees {
        if t.position.col == tree.position.col && t.position.row == tree.position.row - 1 {
            neighbors.0 = Some(t.clone());
        }
        if t.position.col == tree.position.col && t.position.row == tree.position.row + 1 {
            neighbors.1 = Some(t.clone());
        }
        if t.position.col == tree.position.col - 1 && t.position.row == tree.position.row {
            neighbors.2 = Some(t.clone());
        }
        if t.position.col == tree.position.col + 1 && t.position.row == tree.position.row {
            neighbors.3 = Some(t.clone());
        }
    }

    neighbors
}

fn is_visible(trees: &Vec<Tree>, tree: &Tree) -> bool {
    let tree_in_same_row: Vec<Tree> = trees
        .iter()
        .filter(|t| t.position.row == tree.position.row)
        .map(|t| t.to_owned())
        .collect();
    let tree_in_same_col: Vec<Tree> = trees
        .iter()
        .filter(|t| t.position.col == tree.position.col)
        .map(|t| t.to_owned())
        .collect();

    let before_on_row: Vec<Tree> = tree_in_same_row
        .iter()
        .filter(|t| t.position.col < tree.position.col)
        .map(|t| t.to_owned())
        .collect();
    let after_on_row: Vec<Tree> = tree_in_same_row
        .iter()
        .filter(|t| t.position.col > tree.position.col)
        .map(|t| t.to_owned())
        .collect();
    let before_on_col: Vec<Tree> = tree_in_same_col
        .iter()
        .filter(|t| t.position.row < tree.position.row)
        .map(|t| t.to_owned())
        .collect();
    let after_on_col: Vec<Tree> = tree_in_same_col
        .iter()
        .filter(|t| t.position.row > tree.position.row)
        .map(|t| t.to_owned())
        .collect();

    let predicate = |t: &&Tree| t.size < tree.size;

    if before_on_row.iter().filter(predicate).count() == before_on_row.len()
        || after_on_row.iter().filter(predicate).count() == after_on_row.len()
        || before_on_col.iter().filter(predicate).count() == before_on_col.len()
        || after_on_col.iter().filter(predicate).count() == after_on_col.len()
    {
        return true;
    }

    false
}

fn is_the_edge(trees: &Vec<Tree>, tree: &Tree) -> bool {
    let neighbors = find_neighbors(&trees, &tree);

    if neighbors.0.is_none()
        || neighbors.1.is_none()
        || neighbors.2.is_none()
        || neighbors.3.is_none()
    {
        return true;
    }

    false
}

pub fn day08() -> anyhow::Result<()> {
    let trees = load_trees("data/day08.txt")?;

    let visible_on_edges: i32 = trees
        .iter()
        .filter(|tree| is_the_edge(&trees, tree))
        .map(|_| 1)
        .sum();

    let visible_on_grid_trees: Vec<&Tree> = trees
        .iter()
        .filter(|tree| is_the_edge(&trees, tree) == false)
        .filter(|tree| is_visible(&trees, tree))
        .collect_vec();

    let visibles_on_grid: i32 = visible_on_grid_trees.iter().map(|_| 1).sum();

    let total = visibles_on_grid + visible_on_edges;
    println!(
        "Day 08 part 1  - edges({}) + grid({}) = {} ",
        visible_on_edges, visibles_on_grid, total
    );
    Ok(())
}
