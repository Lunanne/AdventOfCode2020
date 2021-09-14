extern crate utils;

use std::iter::FromIterator;

use grid::Grid;

use crate::day3::{part1, part2};
use crate::utils::read_lines;
use crate::utils::grid::CharGrid;
use std::borrow::Borrow;


mod day3;

fn main() {
    let input_text = include_str!("../input/day3_input.txt");
    let lines = read_lines(input_text);
    let grid_input: Vec<Vec<char>> = Vec::from_iter(lines.iter().map(|&s| Vec::from_iter(s.chars())));
    let grid = Grid::from_char_vec(grid_input);
    let part1Result = part1(grid.borrow());
    println!("Answer to day3 part 1 is {}", part1Result);
    let part2Result = part2(grid.borrow());
    println!("Answer to day3 part 2 is {}", part2Result);
}
