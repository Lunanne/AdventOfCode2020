use grid::Grid;
use grid::grid;
use crate::grid::CharGrid;

pub const FLOOR: char = '.';
pub const OCCUPIED_SEAT: char = '#';
pub const EMPTY_SEAT: char = 'L';

pub trait SeatGrid {
    fn amount_occupied_surrounding_seats(&self, x:i32, y:i32) -> usize;
    fn amount_occupied_seats(&self) -> usize;
    fn amount_occupied_seats_in_sight(&self, x:i32, y:i32) -> usize;
    fn check_direction(&self, x:i32, y:i32, step_x: i32, step_y:i32) -> Option<char>;
}

impl SeatGrid for Grid<char> {
    fn amount_occupied_surrounding_seats(&self, x: i32, y: i32) -> usize {
        let surrounding_seats = vec![
            self.get_limited(x-1, y-1),
            self.get_limited(x, y-1),
            self.get_limited(x+1, y-1),
            self.get_limited(x-1, y),
            self.get_limited(x+1, y),
            self.get_limited(x-1, y+1),
            self.get_limited(x, y+1),
            self.get_limited(x+1, y+1),
        ];

        return surrounding_seats.into_iter().filter(|p| p.is_some() && p.unwrap().is_occupied()).count();
    }

    fn amount_occupied_seats(&self) -> usize {
        let seats: &Vec<char> = self.flatten();
        return seats.into_iter().filter(|s| s.is_occupied()).count();
    }

    fn amount_occupied_seats_in_sight(&self, x: i32, y: i32) -> usize {
        let seats = vec![
            self.check_direction(x, y, -1, -1),
            self.check_direction(x, y, 0, -1),
            self.check_direction(x, y, 1, -1),
            self.check_direction(x, y, -1, 0),
            self.check_direction(x, y, 1, 0),
            self.check_direction(x, y, -1, 1),
            self.check_direction(x, y, 0, 1),
            self.check_direction(x, y, 1, 1),
        ];
        return seats.into_iter().filter(|s| s.is_some() && s.unwrap().is_occupied()).count();
    }

    fn check_direction(&self, x:i32, y:i32, step_x: i32, step_y:i32) -> Option<char> {
        let mut mut_x = x + step_x;
        let mut mut_y = y + step_y;
        while self.get_limited(mut_x, mut_y).is_some() {
            if self.get_limited(mut_x, mut_y).unwrap().is_floor() {
                mut_x+=step_x;
                mut_y+=step_y;
            }
            else {
                return self.get_limited(mut_x, mut_y);
            }
        }
        return None;
    }
}


pub trait Seat: Sized {
    fn is_occupied(&self) -> bool;
    fn is_empty(&self) -> bool;
    fn is_floor(&self) -> bool;
}

impl Seat for char {
    fn is_occupied(&self) -> bool {
        return self == &OCCUPIED_SEAT;
    }

    fn is_empty(&self) -> bool {
        return self == &EMPTY_SEAT;
    }

    fn is_floor(&self) -> bool {
        return self == &FLOOR;
    }
}