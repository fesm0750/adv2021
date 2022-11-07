//! Day 01
//!
//! # Problem:
//!
//! From an input file containig unsigned integer values representing measurements:
//!
//! 1. Count the number of times a measurement increases in relation to the previous one;
//!
//! 2. Using a sliding moving window containing three measurements, count how many times the total
//! value of the window increases in relation to the previous one.

extern crate test;

use crate::helpers::read;
use itertools::{izip, Itertools};

pub fn run() {
    let depths = read::file_lines_to_vec("day01").unwrap();

    println!("Day 01");
    println!("Number of depth increases: {}", count_depth_increases(&depths));
    println!(
        "Number of depth increases for a 3 value window: {}",
        count_depth_increases(&depth_slice_window(&depths))
    );
    println!();
}

/// Returns the count for how many times a measurement increased in relation to the previous one.
fn count_depth_increases(depths: &[u32]) -> usize {
    let iter1 = depths.iter();
    let iter2 = depths[1..].iter();

    iter1.zip(iter2).map(|(&a, &b)| b > a).filter(|&x| x).count()
}

/// Applies a three measuments sliding window to the input and returns a `Vec` with the sums for
/// each window.
/// Uses `windows` iterator from the slice.
fn depth_slice_window(depths: &[u32]) -> Vec<u32> {
    const WIN_SIZE: usize = 3;
    depths.windows(WIN_SIZE).map(|a| a.iter().sum()).collect()
}

//--------------------------------------------------------------------
// Additional Solutions for part 2
//--------------------------------------------------------------------

/// Applies a three measuments sliding window to the input and returns a `Vec` with the sums for
/// each window.
/// Uses a loop internally for the sliding window.
#[allow(dead_code)]
fn depth_window_loop(depths: &[u32]) -> Vec<u32> {
    const WINDOW_SIZE: usize = 3;
    let len = depths.len();
    let mut ret = Vec::with_capacity(len - WINDOW_SIZE + 1);

    let mut i = WINDOW_SIZE - 1;
    while i < len {
        ret.push(depths[i - 2] + depths[i - 1] + depths[i]);
        i += 1;
    }

    ret
}

/// Applies a three measuments sliding window to the input and returns a `Vec` with the sums for
/// each window.
/// Uses `itertools::izip` internally.
#[allow(dead_code)]
fn depth_window_izip(depths: &[u32]) -> Vec<u32> {
    izip!(depths, depths[1..].iter(), depths[2..].iter())
        .map(|(a, b, c)| a + b + c)
        .collect()
}

/// Applies a three measuments sliding window to the input and returns a `Vec` with the sums for
/// each window.
/// Uses `itertools::tuple_windows` internally.
#[allow(dead_code)]
fn depth_window_tuple(depths: &[u32]) -> Vec<u32> {
    depths.iter().tuple_windows().map(|(a, b, c)| a + b + c).collect()
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
        static ref TEST_INPUT: Vec<u32> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        static ref INPUT: Vec<u32> = read::file_lines_to_vec("day01").unwrap();
    }

    //-----------------
    // Tests
    //-----------------

    #[test]
    fn test_count_depth_increases() {
        assert_eq!(count_depth_increases(&TEST_INPUT), 7);
    }

    #[test]
    fn test_count_depth_increases_window() {
        assert_eq!(count_depth_increases(&depth_window_izip(&TEST_INPUT)), 5);
    }

    #[test]
    fn test_count_depth_window() {
        assert_eq!(count_depth_increases(&depth_window_loop(&TEST_INPUT)), 5);
    }

    #[test]
    fn test_count_depth_window_tuple() {
        assert_eq!(count_depth_increases(&depth_window_tuple(&TEST_INPUT)), 5);
    }

    #[test]
    fn test_count_depth_slice_window() {
        assert_eq!(count_depth_increases(&depth_slice_window(&TEST_INPUT)), 5);
    }

    //-----------------
    // Benches
    //-----------------

    #[bench]
    fn bench_depth_window_loop(b: &mut Bencher) {
        b.iter(|| depth_window_loop(&INPUT));
    }

    #[bench]
    fn bench_depth_window_izip(b: &mut Bencher) {
        b.iter(|| depth_window_izip(&INPUT));
    }

    #[bench]
    fn bench_depth_window_tuple(b: &mut Bencher) {
        b.iter(|| depth_window_tuple(&INPUT));
    }

    #[bench]
    fn bench_slice_window(b: &mut Bencher) {
        b.iter(|| depth_slice_window(&INPUT));
    }
}
