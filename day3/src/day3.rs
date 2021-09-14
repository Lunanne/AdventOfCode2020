use regex::Regex;
use grid::Grid;
use crate::utils::grid::CharGrid;
use crate::utils::grid::Square;

pub(crate) fn part1(grid: Grid<char>) -> i32 {
    let step_x = 3;
    let step_y = 1;
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
        x +=3;
        y +=1;
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
        let part1result = part1(grid);
        assert_eq!(part1result, 7);
    }
}
