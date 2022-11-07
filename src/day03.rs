//! Day 03
//!
//! # Problem:
//!
//! From an input file containig a list of binary numbers:
//!
//! 1. Determine which bit is the most common for each position and then calculate the power
//! consumption rate based on this information;
//!
//! 2. Determine the Oxygen Generator Rating and the CO2 Scrubber Ratings. Filter the numbers
//! according to a bit criteria until just one remains. Criterias:
//!    a. For the OGR, find the most common value for a bit position and keep only numbers with
//! that bit in that position, then repeat the process for the next position;
//!    b. For CO2 Scrubber rating, keep only the numbers that have the least common value.
//!
//! # Implementation Details
//!
//! For part 1, the `get_frequencies` function reads the whole list and returns a `frequencies`
//! array with the counting of the occurencies of bit `1` for each position. This info is used later
//! to determine the most common bits.
//!
//! For part 2 uses `most_common_bit_at` a given position due to the need of iteratively filtering
//! the list;
//!
//! Tie rules: for part 1, a tie results in bit 1. However, for part 2, a draw would result in
//! yielding a bit '1' for the Oxigen Generator Rating and a '0' for the CO2 Scrubber Rating.

extern crate test;
use crate::helpers::read;
use std::str;

pub fn run() {
    let diagnostic_report = read::file_to_string("day03").unwrap();

    // Part 01 - Power consumption parameters
    let (size, frequencies) = get_frequencies(&diagnostic_report);
    let pcr = power_consumption_rate(size, &frequencies);

    // Part 02 - Life support parameters
    let diagnostic_report: Vec<&str> = diagnostic_report.lines().collect();
    let ogr = calc_life_support_params(&diagnostic_report, true);
    let co2sr = calc_life_support_params(&diagnostic_report, false);

    println!("Day 03");
    println!("Power Consumption Rate: {}", pcr);
    println!("Life Support Rate: {}", ogr * co2sr);
    println!();
}

/// returns the total of elements in the list and an array of `frequencies` for the occurencies of
/// bit `1` in each position.
///
/// # Assumptions:
/// `input` is a list of 12 bits binary numbers separated by new line characters.
///
/// # Implementation Details
/// Even if a value has less than 12 bits, it is considered to have the least significant bits as
/// zero. For example, 101 would be processed as 101000000000.
///
/// # Panics
/// Panics if a value in the list has more than 12 bits.
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
    size += 1; // +1 because counting started at zero

    (size, frequencies)
}

/// returns the power_consumption_rate
fn power_consumption_rate(total_elements: usize, bit_1_frequencies: &[u32; 12]) -> u32 {
    // gamma is composed by the most common bits on diagnostic report
    let gamma = bit_1_frequencies.map(|f| most_common_bit_as_ascii(total_elements, f));
    // Safety: array has been built above and contains only valid characters
    let gamma = parse_binary(unsafe { str::from_utf8_unchecked(&gamma) });

    // epsilon is the reverse of gamma, the least common digits
    const INVERSION_MASK: u32 = 0b111111111111;
    let epsilon = gamma ^ INVERSION_MASK;

    gamma * epsilon
}

/// returns life support parameters. If the flag `is_ogr` is set, it returns the Oxygen Generator
/// Rating, otherwise returns the CO2 Scrubber Rating.
///
/// # Assumptions
/// Assumes an unique solution will always exist.
///
/// # Implementation Details
/// Copies the input because the contents of the inner `Vec` will be filtered by each iteration.
fn calc_life_support_params(input: &[&str], is_ogr: bool) -> u32 {
    let mut values: Vec<&str> = input.iter().copied().collect(); // copies the input cuz filtered elements are removed from `Vec`
    let mut idx = 0;
    while let Some(most_common) = most_common_bit_at(&values, idx) {
        if values.len() == 1 {
            break;
        }
        values = values
            .iter()
            .filter(|s| {
                // if `is_ogr` is set, the filter is by the most common, otherwise filters by the
                // least common
                !((s.as_bytes()[idx] == most_common) ^ is_ogr)
            })
            .copied()
            .collect();
        idx += 1;
    }

    parse_binary(&values[0])
}

//--------------------------------------------------------------------
// Helpers
//--------------------------------------------------------------------

/// returns the ascii code for one if `bit_1_count` is equals or greather than `total_elements`,
/// otherwise returns the ascii code for zero.
fn most_common_bit_as_ascii(total_elements: usize, bit_1_count: u32) -> u8 {
    if bit_1_count >= ((total_elements + 1) / 2) as u32 {
        b'1'
    } else {
        b'0'
    }
}

/// From a list of string values representing binary numbers, find the most common bit `at` a the
/// given position and return it as Ascii code.
fn most_common_bit_at(input: &[&str], at: usize) -> Option<u8> {
    let mut freq = 0;
    let mut size = 0;

    // validation added because of a previous bug where `input` came empty. Although that may not
    // happen anymore, it is still a nice validation to keep around.
    if input.is_empty() {
        return None;
    }

    // counting the occurencies of bit `1`
    for (line_count, s) in input.iter().enumerate() {
        let bit = s.as_bytes().get(at)?;
        if *bit == b'1' {
            freq += 1;
        }
        size = line_count;
    }

    size += 1; // correcting because counter started at zero
    let most_common = most_common_bit_as_ascii(size, freq);
    Some(most_common)
}

/// Parses strings representing binary numbers up to 32 bits.
/// Also returns zero if input is empty.
fn parse_binary(s: &str) -> u32 {
    const MAX_BITS: usize = 32;
    const PARSE_MASK: u32 = 0b1 << (MAX_BITS - 1); // example for 8 bits -> 0b10000000
    let number_of_bits = s.len();

    s.as_bytes()
        .iter()
        .enumerate()
        .filter(|(_, &bit)| bit == b'1')
        .fold(0, |acc, (i, _)| acc + (PARSE_MASK >> (MAX_BITS - number_of_bits + i)))
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
        static ref TEST_INPUT_STRING: String =
            "001\n1101\n10001\n100001\n1000001\n01000001\n000000011\n1100000111\n10\n11".to_string();
        static ref TEST_INPUT_VEC: Vec<&'static str> = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001", "00010", "01010"
        ];
        static ref FILE: Vec<String> = read::file_lines_to_vec("day03").unwrap();
    }

    #[test]
    fn test_get_frequencies() {
        assert_eq!(
            get_frequencies(&TEST_INPUT_STRING),
            (10, [7, 4, 1, 1, 1, 1, 1, 3, 2, 1, 0, 0])
        );
    }

    #[test]
    fn test_power_consumption_rate() {
        assert_eq!(
            power_consumption_rate(10, &[7, 4, 1, 1, 1, 1, 1, 3, 2, 1, 0, 0]),
            4192256
        );
    }

    #[test]
    fn test_parse_binary() {
        assert_eq!(parse_binary(&""), 0);
        assert_eq!(parse_binary(&"10110"), 22);
        assert_eq!(parse_binary(&"01001"), 9);
        assert_eq!(parse_binary(&"11111111111111111111111111111111"), 4294967295); // 32 bits
    }

    #[test]
    fn test_most_common_bit_as_ascii() {
        assert_eq!(most_common_bit_as_ascii(3, 1), b'0');
        assert_eq!(most_common_bit_as_ascii(3, 2), b'1');
        assert_eq!(most_common_bit_as_ascii(4, 1), b'0');
        assert_eq!(most_common_bit_as_ascii(4, 2), b'1');
        assert_eq!(most_common_bit_as_ascii(4, 3), b'1');
    }

    #[test]
    fn test_most_common_byte_at() {
        assert_eq!(most_common_bit_at(&TEST_INPUT_VEC, 0).unwrap(), b'1');
        assert_eq!(most_common_bit_at(&TEST_INPUT_VEC, 1).unwrap(), b'0');
        assert_eq!(most_common_bit_at(&TEST_INPUT_VEC, 2).unwrap(), b'1');
        assert_eq!(most_common_bit_at(&TEST_INPUT_VEC, 3).unwrap(), b'1');
        assert_eq!(most_common_bit_at(&TEST_INPUT_VEC, 4).unwrap(), b'0');
    }

    #[test]
    fn test_calc_life_support_params() {
        assert_eq!(calc_life_support_params(&TEST_INPUT_VEC, true), 23);
        assert_eq!(calc_life_support_params(&TEST_INPUT_VEC, false), 10);
    }

    #[bench]
    fn bench_calc_life_support_rating1a(b: &mut Bencher) {
        b.iter(|| calc_life_support_params(&TEST_INPUT_VEC, true));
    }

    #[bench]
    fn bench_calc_life_support_rating2a(b: &mut Bencher) {
        b.iter(|| calc_life_support_params(&TEST_INPUT_VEC, false));
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
