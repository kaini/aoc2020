extern crate regex;

use std::io::Read;
use regex::Regex;

fn main() {
    let command_regex = Regex::new(r"^(.)(\d+)$").unwrap();

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let commands: Vec<(char, i32)> = input
        .lines()
        .map(|line| {
            let caps = command_regex.captures(line).unwrap();
            (
                caps.get(1).unwrap().as_str().chars().next().unwrap(),
                caps.get(2).unwrap().as_str().parse().unwrap(),
            )
        })
        .collect();

    let mut x = 0;
    let mut y = 0;
    let mut look_x = 1;
    let mut look_y = 0;
    for &(c, n) in &commands {
        match c {
            'N' => { y -= n; }
            'S' => { y += n; }
            'W' => { x -= n; }
            'E' => { x += n; }
            'F' => { x += look_x * n; y += look_y * n; }
            'L' => { rotate(&mut look_x, &mut look_y, -n); }
            'R' => { rotate(&mut look_x, &mut look_y, n); }
            _ => { panic!(); }
        }
    }
    println!("{:?}", x.abs() + y.abs());

    let mut x = 0;
    let mut y = 0;
    let mut wp_x = 10;
    let mut wp_y = -1;
    for &(c, n) in &commands {
        match c {
            'N' => { wp_y -= n; }
            'S' => { wp_y += n; }
            'W' => { wp_x -= n; }
            'E' => { wp_x += n; }
            'F' => { x += wp_x * n; y += wp_y * n; }
            'L' => { rotate(&mut wp_x, &mut wp_y, -n); }
            'R' => { rotate(&mut wp_x, &mut wp_y, n); }
            _ => { panic!(); }
        }
    }
    println!("{:?}", x.abs() + y.abs());
}

fn rotate(look_x: &mut i32, look_y: &mut i32, mut degrees: i32) {
    assert!(degrees % 90 == 0);
    while degrees != 0 {
        let new_x = -degrees.signum() * *look_y;
        let new_y =  degrees.signum() * *look_x;
        *look_x = new_x;
        *look_y = new_y;
        degrees -= degrees.signum() * 90;
    }
}
