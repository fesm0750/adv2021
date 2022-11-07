//! Day 02
//!
//! # Problem:
//!
//! From an input file containig a list of commands for a submarine:
//!
//! 1. Determine depth and distance travelled by the submarine by the first set of rules;
//!
//! 2. Determine depth and distance by the second set of rules;
//!
//! # Implementation Details
//!
//! Defines a structure `SubCommand` to store a command consisting of a `Direction` enum and an i32
//! value `val`.

use std::error::Error;
use std::str::FromStr;

use crate::helpers::read;

pub fn run() {
    let commands = read::file_lines_to_vec("day02").unwrap();
    let (x, y) = navigate_part1(&commands);

    println!("Day 02");
    println!("Part 01 - Depth times Distance: {}", x * y);

    let (x1, y1) = navigate_part2(&commands);
    println!("Part 02 - Depth times Distance: {}", x1 * y1);
    println!();
}

/// Returns a tuple `(x, y)` as the final position of the submarine by the first set of rules.
fn navigate_part1(commands: &[SubCommand]) -> (i32, i32) {
    let (mut x, mut y) = (0, 0);
    for sc in commands {
        match sc.dir {
            Direction::Upward => y -= sc.val,
            Direction::Downward => y += sc.val,
            Direction::Forward => x += sc.val,
        };
    }
    (x, y)
}

/// Returns a tuple `(x, y)` as the final position of the submarine by the second set of rules.
fn navigate_part2(commands: &[SubCommand]) -> (i32, i32) {
    let (mut x, mut y, mut aim) = (0, 0, 0);
    for sc in commands {
        match sc.dir {
            Direction::Upward => aim -= sc.val,
            Direction::Downward => aim += sc.val,
            Direction::Forward => {
                x += sc.val;
                y += sc.val * aim;
            }
        };
    }
    (x, y)
}

//-------------------------------

#[derive(Copy, Clone)]
enum Direction {
    Forward,
    Downward,
    Upward,
}

#[derive(Copy, Clone)]
struct SubCommand {
    dir: Direction,
    val: i32,
}

impl FromStr for Direction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[0..1] {
            "f" => Ok(Direction::Forward),
            "d" => Ok(Direction::Downward),
            "u" => Ok(Direction::Upward),
            _ => Err("Failed to parse `&str` into a Direction".into()),
        }
    }
}

impl FromStr for SubCommand {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(' ');
        let dir = iter
            .next()
            .ok_or("struct SubCommand, unable to parse direction.")?
            .parse()?;
        let val = iter
            .next()
            .ok_or("struct SubCommand, unable to parse value.")?
            .parse()?;
        Ok(Self { dir, val })
    }
}

//-------------------------------

// fn navigate_fold(commands: &[SubmarineCommands]) -> (u32, u32) {
//     let iter = commands.iter().fold(0, |acc, &sc| {
//         let a = match sc {
//             SubmarineCommands::Upward(v) => -v,
//             SubmarineCommands::Downward(v) => v,
//             SubmarineCommands::Forward(v) => v
//         }
//     });
//     // 0 | x,sc | -> u32 {
//     //     match sc {
//     //         SubmarineCommands::Upward(v) => x + v,
//     //         SubmarineCommands::Downward(v) => x - v,
//     //         SubmarineCommands::Forward(v) => x + v,
//     //     }
//     // },
//     // );
//     (0, 0)
// }

//---------------------------------
// Legacy code
//---------------------------------

#[derive(Copy, Clone)]
enum SubmarineCommands {
    Forward(i32),
    Downward(i32),
    Upward(i32),
}

impl FromStr for SubmarineCommands {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_value = |s: &str| -> i32 { s.split(' ').nth(1).unwrap().parse::<i32>().unwrap() };

        match &s[0..1] {
            "f" => Ok(SubmarineCommands::Forward(parse_value(s))),
            "d" => Ok(SubmarineCommands::Downward(parse_value(s))),
            "u" => Ok(SubmarineCommands::Upward(parse_value(s))),
            _ => Err("Failed to parse `&str` into a SubmarineCommand".into()),
        }
    }
}

#[allow(dead_code)]
fn navigate_part1z(commands: &[SubmarineCommands]) -> (i32, i32) {
    let (mut x, mut y) = (0, 0);
    for &sc in commands {
        match sc {
            SubmarineCommands::Upward(v) => y -= v,
            SubmarineCommands::Downward(v) => y += v,
            SubmarineCommands::Forward(v) => x += v,
        };
    }
    (x, y)
}

#[allow(dead_code)]
fn navigate_part2z(commands: &[SubmarineCommands]) -> (i32, i32) {
    let (mut x, mut y, mut aim) = (0, 0, 0);
    for &sc in commands {
        match sc {
            SubmarineCommands::Upward(v) => aim -= v,
            SubmarineCommands::Downward(v) => aim += v,
            SubmarineCommands::Forward(v) => {
                x += v;
                y += v * aim;
            }
        };
    }
    (x, y)
}

//-----------------------------------
