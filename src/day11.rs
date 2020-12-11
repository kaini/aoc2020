extern crate regex;

use std::io::Read;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Cell {
    Floor, Empty, Occupied,
}

impl Cell {
    fn occuiped(&self) -> usize {
        if *self == Cell::Occupied { 1 } else { 0 }
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let grid: Vec<Vec<Cell>> = input
        .lines()
        .map(|line| line.chars().map(|cell| match cell {
            'L' => Cell::Empty,
            '#' => Cell::Occupied,
            '.' => Cell::Floor,
            other => panic!("Unknown symbol {}", other),
        }).collect())
        .collect();
    let len0 = grid.len();
    let len1 = grid[0].len();
    
    let mut state = grid.clone();
    let mut next_state = vec![vec![Cell::Floor; len1]; len0];
    loop {
        for i in 0..len0 {
            for j in 0..len1 {
                next_state[i][j] = {
                    let mut occupied_neighbours = 0;
                    occupied_neighbours += state.get(i.wrapping_sub(1)).and_then(|line| line.get(j.wrapping_sub(1))).map(Cell::occuiped).unwrap_or(0);
                    occupied_neighbours += state.get(i.wrapping_sub(1)).and_then(|line| line.get(j)).map(Cell::occuiped).unwrap_or(0);
                    occupied_neighbours += state.get(i.wrapping_sub(1)).and_then(|line| line.get(j + 1)).map(Cell::occuiped).unwrap_or(0);
                    occupied_neighbours += state.get(i).and_then(|line| line.get(j.wrapping_sub(1))).map(Cell::occuiped).unwrap_or(0);
                    occupied_neighbours += state.get(i).and_then(|line| line.get(j + 1)).map(Cell::occuiped).unwrap_or(0);
                    occupied_neighbours += state.get(i + 1).and_then(|line| line.get(j.wrapping_sub(1))).map(Cell::occuiped).unwrap_or(0);
                    occupied_neighbours += state.get(i + 1).and_then(|line| line.get(j)).map(Cell::occuiped).unwrap_or(0);
                    occupied_neighbours += state.get(i + 1).and_then(|line| line.get(j + 1)).map(Cell::occuiped).unwrap_or(0);
                    match state[i][j] {
                        Cell::Empty if occupied_neighbours == 0 => Cell::Occupied,
                        Cell::Occupied if occupied_neighbours >= 4 => Cell::Empty,
                        other => other,
                    }
                };
            }
        }
        if next_state == state {
            break;
        }
        std::mem::swap(&mut state, &mut next_state);
    }
    println!("{}", state.iter().map(|row| row.iter().filter(|&&c| c == Cell::Occupied).count()).sum::<usize>());

    let mut state = grid.clone();
    let mut next_state = vec![vec![Cell::Floor; len1]; len0];
    loop {
        for i in 0..len0 {
            for j in 0..len1 {
                next_state[i][j] = {
                    let mut occupied_neighbours = 0;
                    occupied_neighbours += occupied_walk(&state, -1, -1, i, j);
                    occupied_neighbours += occupied_walk(&state, -1,  0, i, j);
                    occupied_neighbours += occupied_walk(&state, -1,  1, i, j);
                    occupied_neighbours += occupied_walk(&state,  0, -1, i, j);
                    occupied_neighbours += occupied_walk(&state,  0,  1, i, j);
                    occupied_neighbours += occupied_walk(&state,  1, -1, i, j);
                    occupied_neighbours += occupied_walk(&state,  1,  0, i, j);
                    occupied_neighbours += occupied_walk(&state,  1,  1, i, j);
                    match state[i][j] {
                        Cell::Empty if occupied_neighbours == 0 => Cell::Occupied,
                        Cell::Occupied if occupied_neighbours >= 5 => Cell::Empty,
                        other => other,
                    }
                };
            }
        }
        if next_state == state {
            break;
        }
        std::mem::swap(&mut state, &mut next_state);
    }
    println!("{}", state.iter().map(|row| row.iter().filter(|&&c| c == Cell::Occupied).count()).sum::<usize>());
}

fn occupied_walk(grid: &Vec<Vec<Cell>>, xoff: isize, yoff: isize, x: usize, y: usize) -> usize {
    let mut x = x as isize;
    let mut y = y as isize;
    
    x += xoff;
    y += yoff;
    while x >= 0 && x < grid.len() as isize && y >= 0 && y < grid[0].len() as isize {
        match grid[x as usize][y as usize] {
            Cell::Occupied => { return 1; }
            Cell::Empty => { return 0; }
            _ => {}
        };
        x += xoff;
        y += yoff;
    }
    return 0;
}
