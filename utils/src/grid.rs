use grid::Grid;
use grid::grid;

const TREE: char = '#';
const EMPTY: char = '.';

pub trait CharGrid {
    fn get_infinite_right(&self, x:usize, y:usize) -> Option<char>;
    fn from_char_vec(chars: Vec<Vec<char>>) -> Grid<char>;
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

    fn from_char_vec(chars: Vec<Vec<char>>) -> Grid<char>{
        let mut grid = grid![];
        for row in chars {
            grid.push_row(row);
        }
        return grid;
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


