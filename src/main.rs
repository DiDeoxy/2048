extern crate termion;

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

static ROWS: [[usize; 4]; 4] = [[0, 1, 2, 3], [4, 5, 6, 7], [8, 9, 10, 11], [12, 13, 14, 15]];
static COLS: [[usize; 4]; 4] = [[0, 4, 8, 12], [1, 5, 9, 13], [2, 6, 10, 14], [3, 7, 11, 15]];

fn make_move(board: &mut [u8; 16], indices: &[[usize; 4]; 4], f: i8) -> [u8; 16] {
    let mut new_board = [0u8; 16];

    for index in indices.iter() {
        let mut old_values = [0u8; 4];
        if f > 0 {
            for (i, pos) in index.iter().enumerate() {
                old_values[i] = board[*pos]
            }
        } else {
            for (i, pos) in index.iter().rev().enumerate() {
                old_values[i] = board[*pos]
            }
        }

        let mut at = 4;
        for (i, value) in old_values.clone().iter().enumerate() {
            if (at == 4 || *value != old_values[at]) && *value > 0 {
                at = i;
            } else if at != 4 && *value == old_values[at] {
                old_values[at] = value * 2;
                old_values[i] = 0;
                at = 4;
            }
        }

        'outer: loop {
            let mut to = 4;
            for (i, old_value) in old_values.clone().iter().enumerate() {
                if i == 3 && *old_value == 0 {
                    break 'outer;
                } else if *old_value == 0 && to == 4 {
                    to = i;
                } else if *old_value > 0 && to != 4 {
                    old_values[to] = *old_value;
                    old_values[i] = 0;
                    break;
                }
            }
        }

        if f > 0 {
            for (i, pos) in index.iter().enumerate() {
                new_board[*pos] = old_values[i]
            }
        } else {
            for (i, pos) in index.iter().rev().enumerate() {
                new_board[*pos] = old_values[i]
            }
        }
    }
    new_board
}

fn match_move(direction: &str, mut board: [u8; 16]) -> [u8; 16] {
    match direction {
        "l" => make_move(&mut board, &ROWS, 1),
        "u" => make_move(&mut board, &COLS, 1),
        "r" => make_move(&mut board, &ROWS, -1),
        "d" => make_move(&mut board, &COLS, -1),
        _ => {
            println!("Invalid move, try again!");
            board
        }
    }
}

fn print_board(board: &[u8; 16]) {
    println!(
        "The current board is:\n{:?}\n{:?}\n{:?}\n{:?}.\n",
        &board[0..4],
        &board[4..8],
        &board[8..12],
        &board[12..16]
    );
}

fn is_power_of_two(x: i32) -> bool {
    (x != 0) && ((x & (x - 1)) == 0)
}

fn main() {
    let mut win_value_string = String::new();
    let win_value = loop {
        println!("What value of a block do you want for the win value? (must be a power of 2).");
        io::stdin()
            .read_line(&mut win_value_string)
            .ok()
            .expect("Failed to read line");
        match win_value_string.trim().parse::<i32>() {
            Ok(num) => {
                if is_power_of_two(num) {
                    break num;
                } else {
                    println!("Enter a power of two!");
                    win_value_string = String::new();
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                win_value_string = String::new();
            }
        }
    };

    let mut board = [0u8; 16];
    board[0] = 2;
    board[1] = 2;
    board.shuffle(&mut thread_rng());

    print_board(&board);

    println!("Valid moves are: l (left), r (right), u (up), d (down).");

    loop {
        let mut direction = String::new();
        io::stdin()
            .read_line(&mut direction)
            .ok()
            .expect("Failed to read line");

        let new_board = match_move(direction.trim(), board);

        if new_board.iter().any(|&x| x as i32 == win_value) {
            print_board(&new_board);
            println!("You win! Congratulations!");
            break;
        } else if !new_board.iter().any(|&x| x == 0) {
            print_board(&board);
            println!("No, new value can be added, you suck, loser!");
            break;
        } else if new_board != board {
            board = new_board;
            let mut index: Vec<usize> = Vec::new();
            for (i, value) in board.iter().enumerate() {
                if *value == 0 {
                    index.push(i)
                }
            }
            let new_value = [(2, 9), (4, 1)]
                .choose_weighted(&mut thread_rng(), |item| item.1)
                .unwrap()
                .0;
            board[*index.choose(&mut thread_rng()).unwrap()] = new_value;
        } else {
            println!("No change in board.")
        }

        println!(
            "The current board is:\n{:?}\n{:?}\n{:?}\n{:?}.\n",
            &board[0..4],
            &board[4..8],
            &board[8..12],
            &board[12..16]
        );
    }
}
