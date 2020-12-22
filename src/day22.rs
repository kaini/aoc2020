extern crate regex;

use std::io::Read;
use std::collections::HashSet;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut init_player_a_deck: Vec<usize> = Vec::new();
    let mut init_player_b_deck: Vec<usize> = Vec::new();
    let mut state = 0;
    for line in input.lines() {
        match state {
            0 => {
                assert_eq!(line, "Player 1:");
                state = 1;
            }
            1 => {
                if line.is_empty() {
                    state = 2;
                } else {
                    init_player_a_deck.push(line.parse().unwrap());
                }
            }
            2 => {
                assert_eq!(line, "Player 2:");
                state = 3;
            }
            3 => {
                init_player_b_deck.push(line.parse().unwrap());
            }
            _ => panic!(),
        }
    }

    let mut player_a_deck = init_player_a_deck.clone();
    let mut player_b_deck = init_player_b_deck.clone();
    while !player_a_deck.is_empty() && !player_b_deck.is_empty() {
        let a = player_a_deck.remove(0);
        let b = player_b_deck.remove(0);
        if a > b {
            player_a_deck.push(a);
            player_a_deck.push(b);
        } else {
            player_b_deck.push(b);
            player_b_deck.push(a);
        }
    }
    let winner = if player_a_deck.is_empty() { player_b_deck } else { player_a_deck };
    println!("{:?}", winner);
    println!("{}", winner.iter().rev().enumerate().map(|(i, card)| (i + 1) * card).sum::<usize>());

    let (_, winner) = game_rec(&init_player_a_deck, &init_player_b_deck);
    println!("{:?}", winner);
    println!("{}", winner.iter().rev().enumerate().map(|(i, card)| (i + 1) * card).sum::<usize>());
}

fn game_rec(init_player_a_deck: &[usize], init_player_b_deck: &[usize]) -> (bool, Vec<usize>) {
    let mut seen: HashSet<(Vec<usize>, Vec<usize>)> = HashSet::new();
    let mut player_a_deck = init_player_a_deck.to_vec();
    let mut player_b_deck = init_player_b_deck.to_vec();
    while !player_a_deck.is_empty() && !player_b_deck.is_empty() {
        // Loop dedection
        let seen_entry = (player_a_deck.clone(), player_b_deck.clone());
        if !seen.insert(seen_entry) {
            return (true, player_a_deck.to_vec());
        }

        // Draw cards
        let a = player_a_deck.remove(0);
        let b = player_b_deck.remove(0);

        // Subgame?
        if player_a_deck.len() >= a && player_b_deck.len() >= b {
            let (a_won, _) = game_rec(&player_a_deck[..a], &player_b_deck[..b]);
            if a_won {
                player_a_deck.push(a);
                player_a_deck.push(b);
            } else {
                player_b_deck.push(b);
                player_b_deck.push(a);
            }
        } else {
            if a > b {
                player_a_deck.push(a);
                player_a_deck.push(b);
            } else {
                player_b_deck.push(b);
                player_b_deck.push(a);
            }
        }
    }

    let result = if player_a_deck.is_empty() {
        (false, player_b_deck)
    } else {
        (true, player_a_deck)
    };
    result
}
