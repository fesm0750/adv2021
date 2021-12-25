//! Day 03
//!
//! # Problem:
//!
//! From an input file containig a list of binary values:
//!
//! 1. Determine which bit is the most common bit for each position and then create the numbers
//! gamma and epsilon based on those values;
//!
//! 2.
//!
//! # Implementation Details
//!
//! For part 1, there is no rule for draws. However, for part 2 a draw would result in yielding a
//! bit '1' for the Oxigen Generator Rating and a '0' for the CO2 Scrubber Rating.
extern crate test;
use crate::helpers::read;
use std::str;

pub fn run() {
    let diagnostic_report = read::to_str("day03").unwrap();

    // Part 01 - Power consumption parameters
    let (size, frequencies) = get_frequencies(&diagnostic_report);
    let gamma = calculate_gamma(&frequencies, size); // most common bits on diagnostic_report
    let epsilon = calculate_epsilon(gamma); // least common bits

    // Part 02 - Life support parameters
    let diagnostic_report: Vec<&str> = diagnostic_report.lines().collect();
    let ogr = calc_life_support_params(&diagnostic_report, true);
    let co2sr = calc_life_support_params(&diagnostic_report, false);

    println!("Day 03");
    println!("Power Consumption Rate: {}", gamma * epsilon);
    println!();
    println!("Life Support Rate: {}", ogr * co2sr);
}

/// returns the total of elements processed and an array of frequencies for bit `1` in each
/// position.
fn get_frequencies(input: &str) -> (usize, [u32; 12]) {
    let mut frequencies: [u32; 12] = [0; 12];
    let mut size = 0;

    for (line_count, s) in input.lines().enumerate() {
        for (i, ch) in s.chars().enumerate() {
            if ch == '1' {
                frequencies[i] += 1;
            }
        }
        size = line_count;
    }
    size += 1; // correcting because counting started at zero

    (size, frequencies)
}

fn calculate_gamma(frequencies: &[u32; 12], size: usize) -> u32 {
    let half_size = ((size + 1) / 2) as u32;
    let gamma = frequencies.map(|f| if f > half_size { b'1' } else { b'0' });
    let gamma = unsafe { str::from_utf8_unchecked(&gamma) };
    parse_binary(gamma)
}

fn calculate_epsilon(gamma: u32) -> u32 {
    const INVERSION_MASK: u32 = 0b111111111111;
    gamma ^ INVERSION_MASK
}

/// returns life support parameters. If the flag `is_ogr` is set, it return the Oxygen Generator
/// Rating, otherwise it returns the CO2 Scrubber Rating.
fn calc_life_support_params(input: &[&str], is_ogr: bool) -> u32 {
    let mut bytes: Vec<&str> = input.iter().copied().collect();
    let mut idx = 0;
    while let Some(most_common) = most_common_byte_at(&bytes, idx) {
        if bytes.len() == 1 {
            break;
        }
        bytes = bytes
            .iter()
            .filter(|s| {
                let is_most_common = s.as_bytes()[idx] == most_common;
                !(is_most_common ^ is_ogr) // if `is_ogr` is set, the filter is by the most common,
                                           // otherwise filters by the least common
            })
            .copied()
            .collect();
        idx += 1;
    }

    parse_binary(&bytes[0])
}

//--------------------------------------------------------------------
// Helpers
//--------------------------------------------------------------------

///
fn most_common_byte_at(input: &[&str], at: usize) -> Option<u8> {
    let mut freq = 0;
    let mut size = 0;

    // validation added because of a previous bug where `input` came empty. Although that may not
    // happen anymore, it is still a nice validation to keep around.
    if input.is_empty() {
        return None;
    }

    for (line_count, s) in input.iter().enumerate() {
        let byte = s.as_bytes().get(at);
        let byte = if let Some(&x) = byte {
            x
        } else {
            return None;
        };

        if byte == b'1' {
            freq += 1;
        }
        size = line_count;
    }

    size += 2; // correcting because counter started at zero and because integer division rounds down

    let most_common = if freq >= size / 2 { b'1' } else { b'0' };
    Some(most_common)
}

/// Parses strings representing binary numbers up to 32 bits.
/// Also returns zero if input is empty.
fn parse_binary(s: &str) -> u32 {
    const MAX_BITS: usize = 32;
    const PARSE_MASK: u32 = 0b1 << (MAX_BITS - 1); // exemple for 8 bits -> 0b10000000
    let number_of_bits = s.len();

    s.as_bytes()
        .iter()
        .enumerate()
        .filter(|(_, &bit)| bit == b'1')
        .fold(0, |acc, (i, _)| {
            acc + (PARSE_MASK >> (MAX_BITS - number_of_bits + i))
        })
}

//--------------------------------------------------------------------
// Tests and Benches
//--------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    use test::Bencher;

    lazy_static! {
        static ref TEST_INPUT: Vec<&'static str> = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010"
        ];
        static ref ONES: String = "111111111111".to_string();
        static ref FILE: Vec<String> = read::to_lines("day03").unwrap().flatten().collect();
    }

    #[test]
    fn test_most_common_byte_at() {
        assert_eq!(most_common_byte_at(&TEST_INPUT, 0).unwrap(), b'1');
        assert_eq!(most_common_byte_at(&TEST_INPUT, 1).unwrap(), b'0');
        assert_eq!(most_common_byte_at(&TEST_INPUT, 2).unwrap(), b'1');
        assert_eq!(most_common_byte_at(&TEST_INPUT, 3).unwrap(), b'1');
        assert_eq!(most_common_byte_at(&TEST_INPUT, 4).unwrap(), b'0');
    }

    #[test]
    fn test_calculate_ogr() {
        assert_eq!(calc_life_support_params(&TEST_INPUT, true), 23);
        assert_eq!(calc_life_support_params(&TEST_INPUT, false), 10);
    }

    #[bench]
    fn bench_calc_life_support_rating1a(b: &mut Bencher) {
        b.iter(|| calc_life_support_params(&TEST_INPUT, true));
    }

    #[bench]
    fn bench_calc_life_support_rating2a(b: &mut Bencher) {
        b.iter(|| calc_life_support_params(&TEST_INPUT, false));
    }

    #[bench]
    fn bench_calc_life_support_rating1(b: &mut Bencher) {
        let file: Vec<&str> = FILE.iter().map(|s| s.as_str()).collect();
        b.iter(|| calc_life_support_params(&file, true));
    }

    #[bench]
    fn bench_calc_life_support_rating2(b: &mut Bencher) {
        let file: Vec<&str> = FILE.iter().map(|s| s.as_str()).collect();
        b.iter(|| calc_life_support_params(&file, false));
    }
}

//--------------------------------------------------------------------
// Trying a purely iteration solution
//--------------------------------------------------------------------

// fn calculate_ogr<'a>(input: &'a str) {
//     let mut iter: Box<dyn Iterator<Item = &'a str>> = Box::new(input.lines());

//     let process = |iter: Box<dyn Iterator<Item = &'a str>>| {};

//     for i in 0..12 {
//         iter = Box::new(iter.filter(|&s| s.as_bytes().iter().nth(i.clone()).unwrap() == &b'1'));
//     }
// }

//--------------------------------------------------------------------
// Iterator for getting the most common byte at each position in order
//--------------------------------------------------------------------

// struct CommonByteIter<'a> {
//     str: &'a str,
//     idx: usize,
// }

// impl<'a> CommonByteIter<'a> {
//     fn new(str: &'a str) -> Self {
//         Self { str, idx: 0 }
//     }
// }

// impl Iterator for CommonByteIter<'_> {
//     type Item = bool;

//     fn next(&mut self) -> Option<Self::Item> {
//         let mut freq = 0;
//         let mut size = 0;

//         for (count, s) in self.str.lines().enumerate() {
//             let byte = s.as_bytes().get(self.idx);
//             let byte = if let Some(&x) = byte {
//                 x
//             } else {
//                 return None;
//             };

//             if byte == b'1' {
//                 freq += 1;
//             }
//             size = count;
//         }

//         self.idx += 1;
//         Some(freq >= size / 2)
//     }
// }

//--------------------------------------------------------------------
// Helpers
//--------------------------------------------------------------------

// fn is_most_common_at_iter<'a, I>(input: I, pos: usize) -> bool
// where
//     I: IntoIterator<Item = &'a str>,
// {
//     let mut freq = 0;
//     let mut size = 0;

//     for (line_count, s) in input.into_iter().enumerate() {
//         let s = s.as_bytes();
//         if s[pos] == b'1' {
//             freq += 1;
//         }
//         size = line_count;
//     }

//     freq >= size / 2
// }
