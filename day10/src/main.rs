extern crate utils;

use std::borrow::Borrow;

use utils::read_lines;
use crate::day10::{part1, part2, part2_attempt2, part2_ruud};

mod day10;

fn main() {
    let input_text = include_str!("../input/day10_input.txt");
    let input = read_lines(input_text);
    let numbers:Vec<i32> = input.into_iter().map(|s| s.parse().unwrap()).collect();
    let part1result = part1(Vec::from(numbers.borrow()));
    println!("Day 10 part 1 {}", part1result);
    let part2result = part2_ruud(Vec::from(numbers.borrow()));
    println!("Day 10 part 2 {}", part2result);
}
