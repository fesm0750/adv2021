use adv21::*;
use std::env;

fn main() {
    let input = env::args().nth(1);
    if input.is_none() {
        println!("No input argument.");
        return;
    }

    match input.unwrap().parse().unwrap() {
        1 => day01::run(),
        _ => println!("Invalid input argument."),
    }
}