use grid::Grid;
use grid::grid;
use std::fmt::{Display, Formatter};
use std::fmt;

const TREE: char = '#';

const FLOOR: char = '.';
const OCCUPIED_SEAT: char = '#';
const EMPTY_SEAT: char = 'L';

pub trait CharGrid {
    fn get_infinite_right(&self, x:usize, y:usize) -> Option<char>;
    fn get_limited(&self, x:i32, y:i32) -> Option<char>;
    fn from_char_vec(chars: Vec<Vec<char>>) -> Grid<char>;
    fn set(&mut self, x:usize, y:usize, value: char);
    fn print(&self);
}

impl CharGrid for Grid<char> {
    fn get_infinite_right(&self, x: usize,y: usize) -> Option<char> {
        if x < self.size().1 && y< self.size().0  {
            return Option::from(self[y][x]);
        }
        else if x >= self.size().1 && y< self.size().0 {
            return Option::from(self[y][x % self.size().1]);
        }
        return None;
    }

    fn get_limited(&self, x: i32,y: i32) -> Option<char> {
        if x < 0 || y < 0 {
            return None;
        }
        if x < self.size().1 as i32 && y< self.size().0 as i32 {
            return Option::from(self[y as usize][x as usize]);
        }
        return None;
    }

    fn from_char_vec(chars: Vec<Vec<char>>) -> Grid<char>{
        let mut grid = grid![];
        for row in chars {
            grid.push_row(row);
        }
        return grid;
    }

    fn set(&mut self, x: usize, y: usize, value: char) {
        self[y][x] =value;
    }

    fn print(&self) {
        for y in 0..self.size().0 {
            for x in 0..self.size().1 {
                print!("{}", self[y][x]);
            }
            println!();
        }
    }
}

pub trait Square: Sized {
    fn is_tree(&self) -> bool;
}

impl Square for char {
    fn is_tree(&self) -> bool {
        return self == &TREE;
    }
}


