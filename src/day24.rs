extern crate regex;

use std::io::Read;
use std::collections::{HashSet, HashMap};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    
    let mut result = HashSet::new();
    for line in input.lines() {
        let mut i = 0;
        let mut x = 0;
        let mut y = 0;
        while i < line.len() {
            match &line[i..=i] {
                "w" => { x -= 2; }
                "e" => { x += 2; }
                "n" => {
                    i += 1;
                    y += 1;
                    match &line[i..=i] {
                        "w" => { x -= 1; }
                        "e" => { x += 1; }
                        _ => panic!()
                    }
                }
                "s" => {
                    i += 1;
                    y -= 1;
                    match &line[i..=i] {
                        "w" => { x -= 1; }
                        "e" => { x += 1; }
                        _ => panic!()
                    }
                }
                _ => panic!()
            }
            i += 1;
        }
        if !result.insert((x, y)) {
            result.remove(&(x, y));
        }
    }
    println!("{}", result.len());

    let mut state = result;
    let mut next_state = HashSet::new();
    let mut black_neighbours = HashMap::new();
    for _ in 0..100 {
        for (sx, sy) in &state {
            for (offx, offy) in &[(-2, 0), (2, 0), (-1, 1), (-1, -1), (1, 1), (1, -1)] {
                *black_neighbours.entry((sx + offx, sy + offy)).or_insert(0) += 1;
            }
        }

        for &tile in &state {
            if !(!black_neighbours.contains_key(&tile) || black_neighbours[&tile] > 2) {
                next_state.insert(tile);
            }
        }
        for (&tile, &count) in &black_neighbours {
            if count == 2 && !state.contains(&tile) {
                next_state.insert(tile);
            }
        }

        black_neighbours.clear();
        std::mem::swap(&mut state, &mut next_state);
        next_state.clear();
    }
    println!("{}", state.len());
}
