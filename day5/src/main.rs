extern crate utils;

use crate::day5::{part1, part2};
use utils::read_lines;
use std::borrow::Borrow;

mod day5;

fn main() {
    let input_text = include_str!("../input/day5_input.txt");
    let lines = read_lines(input_text);
    let part1Result = part1(Vec::from(lines.borrow()));
    println!("Day 5 part 1 {}", part1Result);
    let part2Result = part2(Vec::from(lines.borrow()));
    println!("Day 5 part 2 {}", part2Result);
}
