extern crate utils;

use std::borrow::Borrow;

use utils::read_lines;
use crate::day8::{part1, part2};

mod day8;
mod operations;

fn main() {
    let input_text = include_str!("../input/day8_input.txt");
    let lines: Vec<String> = read_lines(input_text).into_iter().map(|s| s.to_string()).collect();
    let part1result = part1(Vec::from(lines.borrow()));
    println!("Day 8 part 1 {}", part1result);
    let part2result = part2(Vec::from(lines.borrow()));
    println!("Day 8 part 2 {}", part2result);
}
