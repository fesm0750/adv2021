use std::{convert::TryInto, error::Error, str::FromStr};

use crate::helpers::read;

pub fn run() {
    let input = read::file_to_string("day04").unwrap();
    let (draw_numbers, boards) = parse_input(&input);
    let (first_winner, last_winner) = go_bingo(boards, draw_numbers.into_iter());

    println!("Day 04");
    println!("First Winning Board Score: {}", first_winner);
    println!("Last Winning Board Score: {}", last_winner);
    println!();
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

/// returns an iterator over the scores of the winning boards
fn go_bingo(boards: Vec<BingoBoard>, draw_numbers: impl Iterator<Item = u8>) -> (u32, u32) {
    // iterator does not yield all boards. For simplicity, if there is a draw, only the last one is
    // yield
    let mut unstable_winners = draw_numbers
        .scan(boards, |boards, x| {
            if !boards.is_empty() {
                Some((x, boards.drain_filter(|b| b.mark_number(x)).last()))
            } else {
                None
            }
        })
        .filter_map(|(x, board)| board.map(|b| b.calculate_score(x)));

    (
        unstable_winners.next().unwrap(),
        unstable_winners.last().unwrap(),
    )
}

/// complete a row or column to win
struct BingoBoard {
    board_numbers: [u8; 25],   // original board
    board_markers: [bool; 25], // markers to determine which numbers are marked
    board_row_scores: [u8; 5], // keeps track of marked numbers in each row
    board_col_scores: [u8; 5], // keeps track of marked numbers in each column
}

impl BingoBoard {
    pub fn new(board: [u8; 25]) -> Self {
        Self {
            board_numbers: board,
            board_markers: [false; 25],
            board_row_scores: [0; 5],
            board_col_scores: [0; 5],
        }
    }

    /// returns true if this board has won
    pub fn mark_number(&mut self, n: u8) -> bool {
        let on_board = self.board_numbers.iter().enumerate().find(|(_, &x)| x == n);
        if let Some((idx, _)) = on_board {
            self.board_markers[idx] = true;
            let row_idx = idx / 5;
            let col_idx = idx % 5;
            self.board_row_scores[row_idx] += 1;
            self.board_col_scores[col_idx] += 1;
            return self.board_row_scores[row_idx] == 5 || self.board_col_scores[col_idx] == 5;
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

    #[allow(dead_code)]
    pub fn clean(&mut self) {
        self.board_markers = [false; 25];
        self.board_row_scores = [0; 5];
        self.board_col_scores = [0; 5];
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
