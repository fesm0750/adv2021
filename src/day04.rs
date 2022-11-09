//! Day 04
//!
//! # Problem:
//!
//! From an input file containig in the first line the drawn numbers and then the bingo boards:
//!
//! 1. Determine the first winner and return its score;
//!
//! 2. Determine the last winner and return its score;
//!
//! # Implementation Details
//!
//! - The scores are calculated summing all non marked numbers and multiplying by the number drawn when the board had
//!   won.
//! - There are two implementations for the simulation functions: `go_bingo()` and `go_bingo_new()`. The former uses the
//!   nightly function `drains_filter` from `Vec` and consumes the input boards whereas the latter mutates the input but
//!   does not consume.

extern crate test;
use std::{convert::TryInto, str::FromStr};

use crate::helpers::read;

pub fn run() {
    let input = read::file_to_string("day04").unwrap();
    let (draw_numbers, mut boards) = parse_input(&input);
    let (first_winner, last_winner) = go_bingo_new(&mut boards, draw_numbers.into_iter());

    println!("Day 04");
    println!("First Winning Board Score: {}", first_winner);
    println!("Last Winning Board Score: {}", last_winner);
    println!();
}

/// parses the input string and returns a tuple containing the drawn numbers and the boards.
fn parse_input(input: &str) -> (Vec<u8>, Vec<BingoBoard>) {
    let mut input = input.split("\n\n");
    let drawn_numbers: Vec<u8> = input.next().unwrap().split(',').flat_map(str::parse::<u8>).collect();
    let boards: Vec<BingoBoard> = input.flat_map(str::parse).collect();

    (drawn_numbers, boards)
}

/// runs the bingo simulation and returns a tuple containing the scores of the first and last winner.
///
/// # Assumptions
/// All boards win at some time;
/// If there is a tie, uses the order of appearence on the `boards` slice input.
fn go_bingo_new(boards: &mut [BingoBoard], draw_numbers: impl Iterator<Item = u8>) -> (u32, u32) {
    let total_boards = boards.len();
    let mut count_winners = 0;
    let mut winning_boards_indexes = Vec::with_capacity(total_boards);

    draw_numbers
        .take_while(move |_| count_winners < total_boards)
        .for_each(|n| {
            boards
                .iter_mut()
                .enumerate()
                .filter(|(_, b)| b.is_playing())
                .for_each(|(idx, b)| {
                    if b.mark_number(n) {
                        count_winners += 1;
                        winning_boards_indexes.push(idx);
                    }
                })
        });

    let first_win_idx = *winning_boards_indexes.first().unwrap();
    let last_win_idx = *winning_boards_indexes.last().unwrap();

    (
        boards[first_win_idx].calculate_score(),
        boards[last_win_idx].calculate_score(),
    )
}

/// runs the bingo simulation and returns a tuple containing the scores of the first and last winner.
///
/// # Assumptions
/// - As and Advent of Code solution, it assumes the first and last winners are defined.
/// - Draws for the first winner will yield the score for the later boars, instead of the very first
/// one.
///
/// # Warning
/// - Uses the `drain_filter` nightly function from `Vec`, so it consumes the `boards` input vector.
#[allow(dead_code)]
fn go_bingo(boards: Vec<BingoBoard>, draw_numbers: impl Iterator<Item = u8>) -> (u32, u32) {
    // iterator does not yield all boards. For simplicity, if there is a draw, only the last one is
    // yielded
    let mut unstable_winners = draw_numbers
        .scan(boards, |boards, x| {
            if !boards.is_empty() {
                Some(boards.drain_filter(|b| b.mark_number(x)).last())
            } else {
                None
            }
        })
        .filter_map(|board| board.map(|b| b.calculate_score()));

    (unstable_winners.next().unwrap(), unstable_winners.last().unwrap())
}

/// A struct representing a Bingo Board
/// complete a row or column to win
#[derive(Copy, Clone)]
struct BingoBoard {
    board_numbers: [u8; 25],    // original board
    board_markers: [bool; 25],  // markers for the board, mirrors the board_numbers
    board_row_scores: [u8; 5],  // keeps track of marked numbers in each row
    board_col_scores: [u8; 5],  // keeps track of marked numbers in each column
    winning_number: Option<u8>, // saves the drawn number at the winning moment
}

impl BingoBoard {
    pub fn new(board: [u8; 25]) -> Self {
        Self {
            board_numbers: board,
            board_markers: [false; 25],
            board_row_scores: [0; 5],
            board_col_scores: [0; 5],
            winning_number: None,
        }
    }

    /// marks the input on the board, returns true if this board has won
    pub fn mark_number(&mut self, n: u8) -> bool {
        let on_board = self.board_numbers.iter().enumerate().find(|(_, &x)| x == n);
        if let Some((idx, _)) = on_board {
            self.board_markers[idx] = true;
            let row_idx = idx / 5;
            let col_idx = idx % 5;
            self.board_row_scores[row_idx] += 1;
            self.board_col_scores[col_idx] += 1;
            let has_won = self.board_row_scores[row_idx] == 5 || self.board_col_scores[col_idx] == 5;
            if has_won {
                self.winning_number = Some(n);
            }
            return has_won;
        }
        false
    }

    /// score: the sum of all unmarked numbers times the number drawn at the winning moment.
    /// Also, returns zero if the board has not yet won.
    pub fn calculate_score(&self) -> u32 {
        let sum: u32 = self
            .board_numbers
            .iter()
            .zip(self.board_markers.iter())
            .filter(|(_, &is_marked)| !is_marked)
            .map(|(&n, _)| n as u32)
            .sum();

        sum * self.winning_number.unwrap_or(0) as u32
    }

    /// returns true if a board has not yet won
    pub fn is_playing(&self) -> bool {
        self.winning_number.is_none()
    }

    /// helper to clean the state of the board and start anew
    #[allow(dead_code)]
    pub fn clean(&mut self) {
        self.board_markers = [false; 25];
        self.board_row_scores = [0; 5];
        self.board_col_scores = [0; 5];
        self.winning_number = None;
    }
}

impl FromStr for BingoBoard {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let board: Vec<u8> = s.split_ascii_whitespace().flat_map(str::parse).collect();
        let board: [u8; 25] = board.try_into().map_err(|_| {
            "Error converting Str into BingoBoard. It needs 25 numbers of the u8 type separated by whitespaces."
                .to_string()
        })?;
        Ok(Self::new(board))
    }
}

//--------------------------------------------------------------------
// Testes
//--------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    use test::Bencher;

    lazy_static! {
        static ref BINGO: (Vec<u8>, Vec<BingoBoard>) = parse_input(&read::file_to_string("day04").unwrap());
    }

    //-----------------
    // Benches
    //-----------------

    #[bench]
    fn bench_bingo(b: &mut Bencher) {
        b.iter(|| go_bingo(BINGO.1.clone(), BINGO.0.iter().copied()));
    }

    #[bench]
    fn bench_bingo2(b: &mut Bencher) {
        let mut boards = BINGO.1.clone();
        b.iter(|| {
            boards.iter_mut().for_each(|b| b.clean());
            go_bingo_new(&mut boards, BINGO.0.iter().copied())
        });
    }
}
