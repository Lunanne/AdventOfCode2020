extern crate utils;

use std::borrow::Borrow;

use utils::read_lines;
use crate::day7::{part1, part2};

mod day7;
mod bag;

fn main() {
    let input_text = include_str!("../input/day7_input.txt");
    let lines = read_lines(input_text);
    let part1result = part1(Vec::from(lines.borrow()));
    println!("Day 7 part 1 {}", part1result);

    let part2result = part2(Vec::from(lines.borrow()));
    println!("Day 7 part 2 {}", part2result);
}
