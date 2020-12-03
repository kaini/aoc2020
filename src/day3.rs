use std::io::Read;

const TREE: char = '#';

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let trees: Vec<Vec<_>> = input
        .lines()
        .map(|row| row.chars().map(|cell| cell == TREE).collect())
        .collect();

    println!("{}", count_trees(3, 1, &trees));

    println!("{}", count_trees(1, 1, &trees) * count_trees(3, 1, &trees) * count_trees(5, 1, &trees) * count_trees(7, 1, &trees) * count_trees(1, 2, &trees))
}

fn count_trees(x_offset: usize, y_offset: usize, trees: &Vec<Vec<bool>>) -> usize {
    let mut at_x = 0;
    let mut at_y = 0;
    let mut tree_count = 0;
    while at_y < trees.len() {
        if trees[at_y][at_x % trees[at_y].len()] {
            tree_count += 1;
        }
        at_x += x_offset;
        at_y += y_offset;
    }
    tree_count
}
