extern crate utils;

use crate::day6::{part1, part2};
use std::iter::FromIterator;
use std::borrow::Borrow;

mod day6;

fn main() {
    let input_text = include_str!("../input/day6_input.txt");
    let input = Vec::from_iter(input_text.split("\n\n"));
    let part1result = part1(Vec::from(input.borrow()));
    println!("Answer day 6 part 1 {}", part1result);
    let part2result = part2(Vec::from(input.borrow()));
    println!("Answer day 6 part 2 {}", part2result);
}
