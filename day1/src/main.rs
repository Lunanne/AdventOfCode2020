extern crate utils;

use crate::day1::{part1, part2};
use crate::utils::read_lines;

mod day1;

fn main() {
    let input_text = include_str!("../input/day1_input.txt");
    let lines = read_lines(input_text);
    let input: Vec<i32>= lines.iter().map(|s| s.parse().unwrap()).collect();
    let (x, y) = part1(input.clone());
    println!("answer day 1 is {}", x * y);
    let (x, y, z) = part2(input.clone());
    println!("answer day 2 is {}", x * y * z);
    println!("Hello, world!");
}
