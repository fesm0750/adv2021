//! Day 07
//!
//! From a list of integers representing positions on a line, find the position at which all elements can be grouped
//! together miniising fuel consumption by the following rules:
//!
//! 1. Fuel comsumption is given by the absolute distance between the origin and destination;
//!
//! 2. Fuel comsumption is calculated by the sum of the integers from zero to the distance measured.
use std::cmp::{max, min};

use crate::helpers::read;

pub fn run() {
    let input: String = read::file_to_string("day07").unwrap();
    let crab_fleet: Vec<u16> = read::split_into_vec(input.trim_end(), ",");

    println!("Day 07");
    println!("Fuel spent to get to the median in pt1: {}", min_fuel_pt1(&crab_fleet));
    println!("Fuel spent to get to the mean in pt2: {}", min_fuel_pt2(&crab_fleet));
}

/// Returns the minimum amount of fuel to group all elements into a single position when the fuel comsumption is given
/// by the absolute distance between the origin and destination.
///
/// # Implementation details
/// This problem boils down to the minimisation of the sum of the absolute distances |x_i - m|, which is minimised by
/// the median of the elements x_i.
fn min_fuel_pt1(fleet: &[u16]) -> u32 {
    let mut fleet: Vec<u16> = fleet.iter().copied().collect();
    fleet.sort_unstable();

    let half = fleet.len() / 2;
    let median = fleet[half];

    fleet.iter().map(|&n| (max(n, median) - min(n, median)) as u32).sum()
}

/// Returns the minimum amount of fuel to group all elements into a single position when the fuel comsumption is given
/// by the sum of the integers from zero to the distance measured.
///
/// # Implementation details
/// - The fuel consumption is computed by the sum of integers formula:  f = 1/2[ s * ( s + 1 ) ], where s is the
///   distance |x_i - m|.
///
/// - Using this formula, the consumption is given by a 2nd degree equation of the distance, therefore we have a sum of
///   squared distances which results in the minimisation function being the mean of the elements x_i.
///
/// # Simplified mathematical proof of the minimum
///
/// Given a distance S(m) = |x_i - m|, the fuel consumptions is the sum of all positive integers from zero to S(m). The
/// formula to sum a sequency of positive interger is N = n*(n+1)/2:
///
/// F(m) = Sum_i{ 1/2 * S(m)*[S(m) + 1] }
/// F(m) = 1/2 * Sum_i{ S(m)^2 + S(m) }
/// F(m) = 1/2 * Sum_i{ (x_i - m)^2 + |x_i - m| }
///
/// In order to minimise fuel comsumption, we want to calculate the first derivative in respect to m, for that we use
/// the chain rule and for simplicity we will assume that |x_i - m| = x_i - m:
///
/// dF(m)/dm = 1/2 * Sum_i{ 2(x_i - m) + 1 }
/// dF(m)/dm = 1/2 * Sum_i{ 2x_i - 2m + 1 }
///
/// To get the minimisation funtion, we equals the first derivative to zero:
///
/// 2 * Sum_i{ 2x_i - 2m + 1 } = 0
/// Sum_i{ 2x_i - 2m + 1 } = 0
///
/// Finally, we define n as the upper limit for the summing and then solve for m:
///
/// 2 * Sum_i{ x_i } - 2mn + n = 0
/// 2mn = 2 * Sum_i{ x_i } + n
/// m = 1/n * Sum_i{ x_i } + 1/2
///
/// Therefore the value of m that minimises the fuel consumption function is given by mean(x) plus a decimal constant
/// number. As we are in a integer context, the decimal constant can be discarted and only the mean remains.
///
/// m = mean(x)
///
/// We will also arrive at the same conclusion if we do the same for |x_i - m| = -x_i + m, and then compose booth
/// answers.
fn min_fuel_pt2(fleet: &[u16]) -> u32 {
    let len = fleet.len() as u32;
    let sum: u32 = fleet.iter().map(|&n| n as u32).sum();
    let mean = ((sum + 1) / len) as u16; // +1 for integer division

    fleet
        .iter()
        .map(|&n| {
            let delta = (max(n, mean) - min(n, mean)) as u32;
            delta * (delta + 1) / 2
        })
        .sum()
}

//-----------------
// Type Definitions
//-----------------

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref TEST_INPUT: Vec<u16> = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
    }

    #[test]
    fn test_part01() {
        assert_eq!(min_fuel_pt1(&TEST_INPUT), 37);
    }

    #[test]
    fn test_part02() {
        assert_eq!(min_fuel_pt2(&TEST_INPUT), 168);
    }
}
