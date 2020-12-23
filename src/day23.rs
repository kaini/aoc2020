extern crate regex;

use std::convert::TryInto;

fn main() {
    //let input = "389125467";
    let input = "327465189";
    let cups: Vec<usize> = input.chars().map(|c| c.to_digit(10).unwrap().try_into().unwrap()).collect();

    let result = play_game(&cups, 100);
    let mut output = String::new();
    let mut at = 1;
    for _ in 0..input.len() - 1 {
        output += &format!("{}", result[at]);
        at = result[at];
    }
    println!("{}", output);

    let mut cups2 = cups.clone();
    for n in (cups2.len() + 1)..=1_000_000 {
        cups2.push(n);
    }
    assert_eq!(cups2.len(), 1_000_000);
    let result = play_game(&cups2, 10_000_000);
    println!("{}", result[1] * result[result[1]]);
}

fn play_game(cups: &[usize], rounds: usize) -> Vec<usize> {
    let max_num = cups.len();

    // Mapping from number -> successor in clockwise order (index 0 is unused)
    let mut linked_list = vec![0; cups.len() + 1];
    for i in 0..(cups.len() - 1) {
        linked_list[cups[i]] = cups[i + 1];
    }
    linked_list[cups[cups.len() - 1]] = cups[0];

    let mut current_num = cups[0];
    for _ in 0..rounds {
        // Find the three cups
        let a = linked_list[current_num];
        let b = linked_list[a];
        let c = linked_list[b];

        // Remove the three cups
        linked_list[current_num] = linked_list[c];

        // Find the destination number
        let mut dest_num = current_num - 1;
        if dest_num == 0 {
            dest_num = max_num;
        }
        while dest_num == a || dest_num == b || dest_num == c {
            dest_num -= 1;
            if dest_num == 0 {
                dest_num = max_num;
            }
        }

        // Insert
        linked_list[c] = linked_list[dest_num];
        linked_list[dest_num] = a;

        // Next cup
        current_num = linked_list[current_num];
    }

    linked_list
}
