use regex::Regex;
use grid::Grid;
use crate::utils::grid::CharGrid;
use crate::utils::grid::Square;
use std::borrow::Borrow;

pub(crate) fn part1(grid: &Grid<char>) -> i32 {
   return calculate_trees(grid.borrow(), 3, 1);
}

pub(crate) fn part2(grid: &Grid<char>) -> i64 {
    let results = vec![
        calculate_trees(grid.borrow(), 1, 1),
        calculate_trees(grid.borrow(), 3, 1),
        calculate_trees(grid.borrow(), 5, 1),
        calculate_trees(grid.borrow(), 7, 1),
        calculate_trees(grid.borrow(), 1, 2),
    ];
    let mut outcome:i64 =1;
    for r in results {
        outcome *= r as i64;
    }
    return outcome;
}

fn calculate_trees(grid: &Grid<char>, step_x:usize, step_y:usize) -> i32 {
    let mut x: usize = step_x;
    let mut y: usize = step_y;
    let mut tree_count = 0;
    // for y in 0..grid.rows() {
    //     for x in 0..2*grid.cols(){
    //         print!("{}",grid.get_infinite_right(y,x).unwrap());
    //     }
    //     print!("\n");
    // }
    while y < grid.size().0{
        let square_opt = grid.get_infinite_right(x,y);
        if let Some(square)  = square_opt {
            if square.is_tree(){
                tree_count +=1;
            }
        }
        x +=step_x;
        y +=step_y;
    }
    return tree_count;
}


#[cfg(test)]
mod tests {
    use super::*;
    use grid::Grid;
    use utils::read_lines;
    use std::iter::FromIterator;

    #[test]
    fn test_part1() {
        let input_text = include_str!("../input/day3_test_input.txt");
        let lines = read_lines(input_text);
        let grid_input: Vec<Vec<char>> = Vec::from_iter(lines.iter().map( |&s| Vec::from_iter(s.chars()) ));
        let grid = Grid::from_char_vec(grid_input);
        let part1result = part1(grid.borrow());
        assert_eq!(part1result, 7);
    }

    #[test]
    fn test_part2() {
        let input_text = include_str!("../input/day3_test_input.txt");
        let lines = read_lines(input_text);
        let grid_input: Vec<Vec<char>> = Vec::from_iter(lines.iter().map( |&s| Vec::from_iter(s.chars()) ));
        let grid = Grid::from_char_vec(grid_input);
        let part1result = part2(grid.borrow());
        assert_eq!(part1result, 336);
    }
}
