extern crate regex;

use std::io::Read;
use regex::Regex;
use std::collections::{HashMap, HashSet};

const TILE_SIZE: usize = 10;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Dir {
    North, South, East, West,
}

#[derive(Debug)]
struct Tile {
    data: Vec<bool>,
}

impl Tile {
    fn new() -> Tile {
        Tile {
            data: vec![false; TILE_SIZE * TILE_SIZE],
        }
    }

    fn get(&self, x: usize, y: usize) -> bool {
        assert!(x < TILE_SIZE);
        assert!(y < TILE_SIZE);
        self.data[y * TILE_SIZE + x]
    }

    fn set(&mut self, x: usize, y: usize, value: bool) {
        assert!(x < TILE_SIZE);
        assert!(y < TILE_SIZE);
        self.data[y * TILE_SIZE + x] = value;
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct SolutionCell {
    id: usize,
    rotation: i32,
    flip_x: bool,
    flip_y: bool,
}

impl SolutionCell {
    fn get_transformed(&self, tiles: &HashMap<usize, Tile>, dir: Dir) -> Vec<bool> {
        let tile = tiles.get(&self.id).unwrap();
        let (mut start_x, mut start_y, mut x_off, mut y_off) = match dir {
            Dir::North => (0_usize,       0_usize,       1_isize, 0_isize),
            Dir::South => (0,             TILE_SIZE - 1, 1,       0      ),
            Dir::West =>  (0,             0,             0,       1      ),
            Dir::East =>  (TILE_SIZE - 1, 0,             0,       1      ),
        };
        for _ in 0..(self.rotation / 90) {
            std::mem::swap(&mut start_x, &mut start_y);
            start_x = if start_x == 0 { TILE_SIZE - 1 } else { 0 };
            std::mem::swap(&mut x_off, &mut y_off);
            x_off *= -1;
        }
        if self.flip_x {
            start_x = if start_x == 0 { TILE_SIZE - 1 } else { 0 };
            x_off *= -1;
        }
        if self.flip_y {
            start_y = if start_y == 0 { TILE_SIZE - 1 } else { 0 };
            y_off *= -1;
        }

        let mut result = Vec::new();
        for i in 0..(TILE_SIZE as isize) {
            result.push(tile.get((start_x as isize + i * x_off) as usize, (start_y as isize + i * y_off) as usize))
        }
        result
    }
}

#[derive(Debug, Clone)]
struct Solution {
    data: Vec<HashSet<SolutionCell>>,
    size: usize,
}

impl Solution {
    fn new(size: usize) -> Solution {
        Solution {
            data: vec![HashSet::new(); size * size],
            size,
        }
    }

    fn get_mut(&mut self, x: usize, y: usize) -> &mut HashSet<SolutionCell> {
        assert!(x < self.size);
        assert!(y < self.size);
        &mut self.data[y * self.size + x]
    }

    fn get(&self, x: usize, y: usize) -> &HashSet<SolutionCell> {
        assert!(x < self.size);
        assert!(y < self.size);
        &self.data[y * self.size + x]
    }

    fn hash(&self) -> usize {
        [(0, 0), (0, self.size - 1), (self.size - 1, 0), (self.size - 1, self.size - 1)].iter().map(|&(x, y)| {
            assert_eq!(self.get(x, y).len(), 1);
            self.get(x, y).iter().next().unwrap().id
        }).product()
    }

    fn render(&self, tiles: &HashMap<usize, Tile>) -> Vec<String> {
        let mut result = Vec::new();
        for y in 0..self.size {
            for yy in 1..TILE_SIZE - 1 {
                result.push(String::new());
                for x in 0..self.size {
                    for xx in 1..TILE_SIZE - 1 {
                        let cell = self.get(x, y).iter().next().unwrap();
                        let (mut real_x, mut real_y) = (xx, yy);
                        for _ in 0..(cell.rotation / 90) {
                            std::mem::swap(&mut real_x, &mut real_y);
                            real_x = TILE_SIZE - 1 - real_x;
                        }
                        if cell.flip_x {
                            real_x = TILE_SIZE - 1 - real_x;
                        }
                        if cell.flip_y {
                            real_y = TILE_SIZE - 1 - real_y;
                        }
                        *result.last_mut().unwrap() += if tiles.get(&cell.id).unwrap().get(real_x, real_y) { "#" } else { "." };
                    }
                }
            }
        }
        result
    }
}

fn main() {
    ////////////////
    // Parse
    ////////////////
    let tiles = {
        let tile_header_regex = Regex::new(r"^Tile (\d+):$").unwrap();

        let mut input = String::new();
        std::io::stdin().read_to_string(&mut input).unwrap();
        let mut tiles: HashMap<usize, Tile> = HashMap::new();
        let mut tile_header = true;
        let mut tile_id = 0;
        let mut tile_y = 0;
        for line in input.lines() {
            if tile_header {
                tile_header = false;
                let caps = tile_header_regex.captures(line).unwrap();
                tile_id = caps.get(1).unwrap().as_str().parse().unwrap();
                tiles.insert(tile_id, Tile::new());
            } else {
                if line.is_empty() {
                    assert_eq!(tile_y, TILE_SIZE);
                    tile_header = true;
                    tile_y = 0;
                } else {
                    let mut last_x = 0;
                    for (x, c) in line.chars().enumerate() {
                        tiles.get_mut(&tile_id).unwrap().set(x, tile_y, c == '#');
                        last_x = x;
                    }
                    assert_eq!(last_x, TILE_SIZE - 1);
                    tile_y += 1;
                }
            }
        }
        tiles
    };

    ////////////////
    // Part 1
    ////////////////
    let solution = find_solution(&tiles).unwrap();
    println!("{}", solution.hash());

    ////////////////
    // Part 2
    ////////////////
    let rendered = solution.render(&tiles);
    let monster = &[
        "                  # ",
        "#    ##    ##    ###",
        " #  #  #  #  #  #   ",
    ];

    let mut all_monster_coords = HashSet::new();
    'out: for &flip_x in &[true, false] {
        for &flip_y in &[true, false] {
            for &rot in &[0, 90, 180, 270] {
                let mut count = 0;
                for start_x in 0..solution.size * (TILE_SIZE - 2) - (monster[0].len() - 1) {
                    for start_y in 0..solution.size * (TILE_SIZE - 2) - (monster.len() - 1) {
                        let mut matches = true;
                        let mut monster_coords = HashSet::new();
                        'inner: for mx in 0..monster[0].len() {
                            for my in 0..monster.len() {
                                if monster[my].chars().nth(mx).unwrap() == '#' {
                                    let mut x = start_x + mx;
                                    let mut y = start_y + my;
                                    for _ in 0..(rot / 90) {
                                        std::mem::swap(&mut x, &mut y);
                                        x = solution.size * (TILE_SIZE - 2) - 1 - x;
                                    }
                                    if flip_x {
                                        x = solution.size * (TILE_SIZE - 2) - 1 - x;
                                    }
                                    if flip_y {
                                        y = solution.size * (TILE_SIZE - 2) - 1 - y;
                                    }
                                    if rendered[y].chars().nth(x).unwrap() != '#' {
                                        matches = false;
                                        break 'inner;
                                    }
                                    monster_coords.insert((x, y));
                                }
                            }
                        }
                        if matches {
                            count += 1;
                            all_monster_coords.extend(monster_coords);
                        }
                    }
                }
                if count > 0 {
                    println!("monsters = {}", count);
                    break 'out;
                }
            }
        }
    }

    let mut count = 0;
    for (y, line) in rendered.iter().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            if cell == '#' && !all_monster_coords.contains(&(x, y)) {
                count += 1;
            }
        }
    }
    println!("{}", count);
}

fn find_solution(tiles: &HashMap<usize, Tile>) -> Option<Solution> {
    let solution_size = (tiles.len() as f64).sqrt() as usize;
    println!("input size = {}, solution size = {}x{}", tiles.len(), solution_size, solution_size);

    let mut can_match: HashMap<(SolutionCell, Dir), HashSet<SolutionCell>> = HashMap::new();
    for &a_id in tiles.keys() {
        for &a_flip_x in &[true, false] {
            for &a_flip_y in &[true, false] {
                for &a_rot in &[0, 90, 180, 270] {
                    let a_cell = SolutionCell { id: a_id, rotation: a_rot, flip_x: a_flip_x, flip_y: a_flip_y };
                    for &(a_dir, b_dir) in &[(Dir::North, Dir::South), (Dir::South, Dir::North), (Dir::West, Dir::East), (Dir::East, Dir::West)] {
                        let mut result = HashSet::new();
                        for &b_id in tiles.keys() {
                            if a_id != b_id {
                                for &b_flip_x in &[true, false] {
                                    for &b_flip_y in &[true, false] {
                                        for &b_rot in &[0, 90, 180, 270] {
                                            let b_cell = SolutionCell { id: b_id, rotation: b_rot, flip_x: b_flip_x, flip_y: b_flip_y };
                                            if a_cell.get_transformed(tiles, a_dir) == b_cell.get_transformed(tiles, b_dir) {
                                                result.insert(b_cell);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        can_match.insert((a_cell.clone(), a_dir), result);
                    }
                }
            }
        }
    }
    let can_match = can_match;
    println!("can match len = {}", can_match.len());

    let mut solution = Solution::new(solution_size);
    for x in 0..solution_size {
        for y in 0..solution_size {
            solution.get_mut(x, y).extend(can_match.keys().filter(|k| k.1 == Dir::North).map(|k| k.0.clone()));
        }
    }
    println!("solution space = {}", solution.data.iter().map(|d| d.len() as f64).product::<f64>());

    // Remove pieces that have to be at the border
    for x in 0..solution.size {
        for y in 0..solution.size {
            *solution.get_mut(x, y) = solution.get(x, y).iter().filter(|&c|
                (x == 0 || !can_match[&(c.clone(), Dir::West)].is_empty()) &&
                (x == solution.size - 1 || !can_match[&(c.clone(), Dir::East)].is_empty()) &&
                (y == 0 || !can_match[&(c.clone(), Dir::North)].is_empty()) &&
                (y == solution.size - 1 || !can_match[&(c.clone(), Dir::South)].is_empty())
            ).map(|c| c.clone()).collect();
        }
    }
    println!("solution space = {}", solution.data.iter().map(|d| d.len() as f64).product::<f64>());

    // Decision tree
    find_solution_rec(solution, &can_match)
}

fn find_solution_rec(mut solution: Solution, can_match: &HashMap<(SolutionCell, Dir), HashSet<SolutionCell>>) -> Option<Solution> {
    // Remove pieces that cannot match their remaining neighbours
    for x in 0..solution.size {
        for y in 0..solution.size {
            *solution.get_mut(x, y) = solution.get(x, y).iter().filter(|&c| {
                (x == 0 || !can_match[&(c.clone(), Dir::West)].is_disjoint(solution.get(x - 1, y))) &&
                (x == solution.size - 1 || !can_match[&(c.clone(), Dir::East)].is_disjoint(solution.get(x + 1, y))) &&
                (y == 0 || !can_match[&(c.clone(), Dir::North)].is_disjoint(solution.get(x, y - 1))) &&
                (y == solution.size - 1 || !can_match[&(c.clone(), Dir::South)].is_disjoint(solution.get(x, y + 1)))
            }).map(|c| c.clone()).collect();
            if solution.get(x, y).is_empty() {
                return None;
            }
        }
    }
    println!("solution space = {}", solution.data.iter().map(|d| d.len() as f64).product::<f64>());

    // Check if we are done
    let mut all_len_one = true;
    'outer: for x in 0..solution.size {
        for y in 0..solution.size {
            if solution.get(x, y).len() != 1 {
                all_len_one = false;
                break 'outer;
            }
        }
    }
    if all_len_one {
        return Some(solution);
    }

    // Make a decision
    for x in 0..solution.size {
        for y in 0..solution.size {
            if solution.get(x, y).len() > 1 {
                for possibility in solution.get(x, y) {
                    let mut next_solution = solution.clone();
                    next_solution.get_mut(x, y).clear();
                    next_solution.get_mut(x, y).insert(possibility.clone());
                    for xx in 0..solution.size {
                        for yy in 0..solution.size {
                            if (xx, yy) != (x, y) {
                                *next_solution.get_mut(xx, yy) = next_solution.get(xx, yy).iter().filter(|c| c.id != possibility.id).map(|c| c.clone()).collect();
                            }
                        }
                    }
                    if let Some(result) = find_solution_rec(next_solution, can_match) {
                        return Some(result);
                    }
                }
            }
        }
    }

    None
}
