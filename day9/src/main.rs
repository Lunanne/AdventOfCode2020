extern crate utils;

use std::borrow::Borrow;

use utils::read_lines;
use crate::day9::{part1, part2};

mod day9;

fn main() {
    let input_text = include_str!("../input/day9_input.txt");
    let input = read_lines(input_text);
    let numbers:Vec<i64> = input.into_iter().map(|s| s.parse().unwrap()).collect();
    let part1result = part1(Vec::from(numbers.borrow()));
    println!("Day 9 part 1 {}", part1result);
    let part2result = part2(Vec::from(numbers.borrow()));
    println!("Day 9 part 2 {}", part2result);
}
