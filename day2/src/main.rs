extern crate utils;

use crate::day2::{part1, part2};
use crate::utils::read_lines;

mod day2;

fn main() {
    let input_text = include_str!("../input/day2_input.txt");
    let lines = read_lines(input_text);
    let count = part1(lines.clone());
    println!("answer part 1 is {}", count);
    let count2 = part2(lines.clone());
    println!("answer part 2 is {}", count2);
}
