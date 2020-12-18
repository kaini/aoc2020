extern crate regex;

use std::io::Read;
use std::collections::{HashMap, HashSet};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let initial: HashSet<(i32, i32)> = input.lines().enumerate().flat_map(|(x, line)| {
        line.chars().enumerate().filter_map(move |(y, cell)| {
            if cell == '#' {
                Some((x as i32, y as i32))
            } else {
                None
            }
        })
    }).collect();

    println!("{}", solve(&initial, 3));
    println!("{}", solve(&initial, 4));
    println!("for fun: {}", solve(&initial, 5));
    println!("for fun: {}", solve(&initial, 6));
}

fn solve(initial: &HashSet<(i32, i32)>, dims: usize) -> usize {
    let mut curr_state: HashSet<Vec<i32>> = initial.iter().map(|&(x, y)| {
        let mut p = Vec::with_capacity(dims);
        p.push(x);
        p.push(y);
        while p.len() < dims {
            p.push(0);
        }
        p
    }).collect();
    let mut next_state = HashSet::new();
    let mut active_neighbours = HashMap::new();
    for _ in 0..6 {
        for p in &curr_state {
            for_each_neighbor(p, |np| {
                let np = Vec::from(np);
                if !active_neighbours.contains_key(&np) {
                    active_neighbours.insert(np.clone(), 0);
                }
                *active_neighbours.get_mut(&np).unwrap() += 1;
            });
        }

        for (p, &count) in &active_neighbours {
            if curr_state.contains(p) {
                if count == 2 || count == 3 {
                    next_state.insert(p.clone());
                }
            } else {
                if count == 3 {
                    next_state.insert(p.clone());
                }
            }
        }

        active_neighbours.clear();
        std::mem::swap(&mut curr_state, &mut next_state);
        next_state.clear();
    }
    curr_state.len()
}

fn for_each_neighbor<F: FnMut(&[i32])>(p: &[i32], mut fun: F) {
    for_each_neighbor_rec(p, &mut fun, 0, &mut vec![0; p.len()]);
}

fn for_each_neighbor_rec<F: FnMut(&[i32])>(p: &[i32], fun: &mut F, at: usize, result: &mut [i32]) {
    if at == p.len() {
        if p != result {
            fun(result);
        }
    } else {
        for i in (p[at]-1)..=(p[at]+1) {
            result[at] = i;
            for_each_neighbor_rec(p, fun, at + 1, result);
        }
    }
}
