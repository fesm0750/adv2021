use std::{convert::TryInto, error::Error, str::FromStr};

use crate::helpers::read;

pub fn run() {
    let input = read::file_to_string("day04").unwrap();
    let (drawn_numbers, mut boards) = parse_input(&input);

    let mut score = 0;

    for x in drawn_numbers {
        let mut has_won = false;
        for board in &mut boards {
            has_won = board.mark_number(x);
            if has_won {
                score = board.calculate_score(x);
                break;
            }
        }
        if has_won {
            break;
        }
    }

    println!("Day 04");
    println!("First Winning Board Score: {}", score);
    println!();

    // let total_boards = boards.len();
    // while
}

fn parse_input(input: &str) -> (Vec<u8>, Vec<BingoBoard>) {
    let mut input = input.split("\n\n");
    let drawn_numbers: Vec<u8> = input
        .next()
        .unwrap()
        .split(',')
        .flat_map(str::parse::<u8>)
        .collect();
    let boards: Vec<BingoBoard> = input.flat_map(str::parse).collect();

    (drawn_numbers, boards)
}

/// complete a row or column to win

struct BingoBoard {
    board_numbers: [u8; 25],   // original board
    board_markers: [bool; 25], // markers to determine which numbers where drawn
    board_scores: [u8; 10],    // keep track of currente scores of lines and columns
}

impl BingoBoard {
    pub fn new(board: [u8; 25]) -> Self {
        Self {
            board_numbers: board,
            board_markers: [false; 25],
            board_scores: [0; 10],
        }
    }

    // returns true if this board has won
    pub fn mark_number(&mut self, n: u8) -> bool {
        let on_board = self.board_numbers.iter().enumerate().find(|(_, &x)| x == n);
        if let Some((idx, _)) = on_board {
            self.board_markers[idx] = true;
            let row = BingoBoard::row_idx(idx);
            let col = BingoBoard::col_idx(idx);
            self.board_scores[row] += 1;
            self.board_scores[col] += 1;
            return self.board_scores[row] == 5 || self.board_scores[col] == 5;
        }
        false
    }

    /// score: the sum of all unmarked numbers times the last drawn number
    pub fn calculate_score(&self, last_draw: u8) -> u32 {
        let sum: u32 = self
            .board_numbers
            .iter()
            .zip(self.board_markers.iter())
            .filter(|(_, &is_marked)| !is_marked)
            .map(|(&n, _)| n as u32)
            .sum();

        sum * last_draw as u32
    }

    //------------------------------
    // Private Helpers
    //------------------------------

    /// returns the `board_scores` index for the row where `idx` is located on the flat board
    /// position.
    fn row_idx(idx: usize) -> usize {
        idx / 5
    }

    /// returns the `board_scores` index for the column where `idx` is located on the flat board
    /// position.
    fn col_idx(idx: usize) -> usize {
        5 + (idx % 5)
    }
}

impl FromStr for BingoBoard {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let board: Vec<u8> = s.split_ascii_whitespace().flat_map(str::parse).collect();
        let board: [u8; 25] = board.try_into().unwrap(); // how to use ? here?
        Ok(Self::new(board))
    }
}
