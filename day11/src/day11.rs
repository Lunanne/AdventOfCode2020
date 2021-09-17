use std::borrow::Borrow;

use grid::Grid;
use regex::Regex;

use crate::utils::grid::CharGrid;
use utils::seatgrid::{Seat, SeatGrid, OCCUPIED_SEAT, EMPTY_SEAT};

pub(crate) fn part1(grid: Grid<char>) -> usize {
    let stable_grid = simulate_seat_picking(grid, 4, SeatGrid::amount_occupied_surrounding_seats);
    return stable_grid.amount_occupied_seats();
}

pub(crate) fn part2(grid: Grid<char>) -> usize {
    let stable_grid = simulate_seat_picking(grid, 5, SeatGrid::amount_occupied_seats_in_sight);
    return stable_grid.amount_occupied_seats();}

fn simulate_seat_picking(grid: Grid<char>, max_occupied:usize, occupied_function: fn(&Grid<char>, i32, i32) -> usize) -> Grid<char>{
    let mut changed = true;
    let mut old_grid = grid.clone();
    let mut new_grid= grid.clone();
    while changed {
        changed = false;

        for y in 0..old_grid.size().0 {
            for x in 0..old_grid.size().1 {
                let seat = old_grid.get_limited(x as i32, y as i32);
                if seat.is_some() && !seat.unwrap().is_floor() {
                    let unwrapped_seat = seat.unwrap();
                    let amount_surrounding_seats = occupied_function(&old_grid, x as i32, y as i32);
                    if unwrapped_seat.is_empty() && amount_surrounding_seats == 0 {
                        changed = true;
                        new_grid.set(x, y, OCCUPIED_SEAT);
                    } else if unwrapped_seat.is_occupied() && amount_surrounding_seats >= max_occupied {
                        changed = true;
                        new_grid.set(x, y, EMPTY_SEAT);
                    }
                }
            }
        }

        // let tmp = new_grid.amount_occupied_seats();
        // println!("amount of seats {}", tmp);
        old_grid = new_grid.clone();
    }
    return new_grid;
}


#[cfg(test)]
mod tests {
    use std::iter::FromIterator;

    use grid::Grid;

    use utils::read_lines;

    use super::*;

    #[test]
    fn test_part1() {
        let input_text = include_str!("../input/day11_test_input.txt");
        let lines = read_lines(input_text);
        let grid_input: Vec<Vec<char>> = Vec::from_iter(lines.iter().map(|&s| Vec::from_iter(s.chars())));
        let grid = Grid::from_char_vec(grid_input);
        let part1result = part1(grid.clone());
        assert_eq!(part1result, 37);
    }

    #[test]
    fn test_part2() {
        let input_text = include_str!("../input/day11_test_input.txt");
        let lines = read_lines(input_text);
        let grid_input: Vec<Vec<char>> = Vec::from_iter(lines.iter().map(|&s| Vec::from_iter(s.chars())));
        let grid = Grid::from_char_vec(grid_input);
        let part1result = part2(grid.clone());
        assert_eq!(part1result, 26);
    }
}
