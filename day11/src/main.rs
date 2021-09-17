extern crate utils;

use std::iter::FromIterator;

use grid::Grid;

use crate::day11::{part1, part2};
use crate::utils::read_lines;
use crate::utils::grid::CharGrid;
use std::borrow::Borrow;


mod day11;

fn main() {
    let input_text = include_str!("../input/day11_input.txt");
    let lines = read_lines(input_text);
    let grid_input: Vec<Vec<char>> = Vec::from_iter(lines.iter().map(|&s| Vec::from_iter(s.chars())));
    let grid = Grid::from_char_vec(grid_input);
    let part1result = part1(grid.clone());
    println!("Answer to day 11 part 1 is {}", part1result);
    let part2result = part2(grid.clone());
    println!("Answer to day 11 part 2 is {}", part2result);
}
