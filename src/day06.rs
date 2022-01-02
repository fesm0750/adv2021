use std::{error::Error, str::FromStr};

use crate::helpers::read;

pub fn run() {
    let mut fishes = parse_input();
    let ans_pt1 = part01(&mut fishes, 80);
    let mut fish_school = parse_input_pt2();
    let ans_pt2 = part02(&mut fish_school, 256);

    println!("Day 06");
    println!("Fish schools size after 80 days: {}", ans_pt1);
    println!("Fish schools size after 256 days: {}", ans_pt2);
}

fn parse_input() -> Vec<Lanternfish> {
    let input = read::file_to_string("day06").unwrap();
    let input = input.lines().next().unwrap();
    read::split_into_vec(input, ",")
}

fn parse_input_pt2() -> SchoolOfLanternfish {
    let input = read::file_to_string("day06").unwrap();
    let input = input.lines().next().unwrap();
    input.parse().unwrap()
}

fn part01(fish_school: &mut Vec<Lanternfish>, days: usize) -> usize {
    for _ in 1..days + 1 {
        let mut younglings: Vec<Lanternfish> = Vec::new();
        for f in fish_school.iter_mut() {
            let young = f.try_reproduce();
            if let Some(lf) = young {
                younglings.push(lf);
            }
        }
        fish_school.extend(younglings);
    }
    fish_school.len()
}

fn part02(fish_school: &mut SchoolOfLanternfish, days: usize) -> u64 {
    // println!("{:?}", fish_school);
    for _ in 1..days + 1 {
        fish_school.try_reproduce();
        // println!("{:?}", fish_school);
    }
    fish_school.school.iter().sum()
}

//------------------------------
// Lanternfish
//------------------------------

#[derive(Copy, Clone, Debug)]
struct Lanternfish(u8);

impl Lanternfish {
    fn try_reproduce(&mut self) -> Option<Lanternfish> {
        if self.0 == 0 {
            self.0 = 6;
            return Some(Lanternfish(8));
        }
        self.0 -= 1;
        None
    }
}

impl FromStr for Lanternfish {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Lanternfish(s.parse()?))
    }
}

//------------------------------
// Fish School
//------------------------------

#[derive(Copy, Clone, Debug)]
struct SchoolOfLanternfish {
    school: [u64; 9], // an array to use as a circular buffer and hold day 0 to day 8
    idx0: usize,      // the index for day 0 in the buffer
}

impl SchoolOfLanternfish {
    const BUFFER_LEN: usize = 9;
    const RESET_DAY: usize = 6;

    /// uses a circular buffer, `self::idx0` indicates where in the buffer is located the amount of
    /// fishes at day 0. When `idx0` moves to the next position, the previous one becomes day 8.
    /// Therefore, moving `idx0` already implements the counter for the newborn whereas the current
    /// fishes needs to be moved to day 6.
    fn try_reproduce(&mut self) {
        let curr = self.school[self.idx0];
        self.idx0 = (self.idx0 + 1) % Self::BUFFER_LEN; // updates current day 0 idx
        let reset_idx = (self.idx0 + Self::RESET_DAY) % Self::BUFFER_LEN; // define reset idx
        self.school[reset_idx] += curr; // adds the reseted fishes to reset day
    }
}

impl FromStr for SchoolOfLanternfish {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut school = [0u64; Self::BUFFER_LEN];
        let fish_timers = s.split(',').flat_map(str::parse::<usize>);
        for t in fish_timers {
            school[t] += 1;
        }
        Ok(Self { school, idx0: 0 })
    }
}

//--------------------------------------------------------------------
// Tests and Benches
//--------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref TEST_INPUT: Vec<Lanternfish> = vec![
            Lanternfish(3),
            Lanternfish(4),
            Lanternfish(3),
            Lanternfish(1),
            Lanternfish(2)
        ];
        static ref TEST_INPUT2: SchoolOfLanternfish = "3,4,3,1,2".parse().unwrap();
    }

    #[test]
    fn test_part01() {
        let mut fishes = TEST_INPUT.clone();
        assert_eq!(part01(&mut fishes, 18), 26);
        assert_eq!(part01(&mut fishes, 62), 5934); // total = 80 days
    }

    #[test]
    fn test_part02() {
        let mut fish_school = *TEST_INPUT2;
        assert_eq!(part02(&mut fish_school, 18), 26);
        assert_eq!(part02(&mut fish_school, 62), 5934); // total = 80 days
        assert_eq!(part02(&mut fish_school, 176), 26984457539); // total = 256 days
    }
}
