use std::{error::Error, str::FromStr};

use crate::helpers::read;

pub fn run() {
    let mut fish_school = parse_input();
    println!("Day 06");
    println!(
        "Fish schools size after 80 days: {}",
        part01(&mut fish_school, 80)
    );
    println!(
        "Fish schools size after 256 days: {}",
        part01(&mut fish_school, 256 - 80)
    );
}

fn parse_input() -> Vec<Lanternfish> {
    let input = read::file_to_string("day06").unwrap();
    let input = input.lines().next().unwrap();
    read::split_into_vec(input, ",")
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
    }

    #[test]
    fn test_part01() {
        let mut fish_school = TEST_INPUT.clone();
        assert_eq!(part01(&mut fish_school, 18), 26);
        assert_eq!(part01(&mut fish_school, 62), 5934); // total = 80 days
        assert_eq!(part01(&mut fish_school, 176), 26984457539); // total = 256 days
    }
}
