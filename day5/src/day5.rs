use regex::Regex;
use std::borrow::Borrow;
use std::str::Chars;

pub(crate) fn part1(seats: Vec<&str>) -> i32 {
    return seats.into_iter().map(|seat| parse_seat(seat)).max().unwrap();
}

pub(crate) fn part2(seats: Vec<&str>) -> i32 {
    let mut seat_map: Vec<i32> = seats.into_iter().map(|seat| parse_seat(seat)).collect();
    seat_map.sort();
    let mut last_seat_id = 0;
    for seat_id in seat_map {
        if seat_id-last_seat_id != 2 {
            last_seat_id = seat_id;
        }
        else {
            return last_seat_id + 1;
        }
    }
    return 0;
}

fn parse_seat(seat:&str) -> i32 {
    let regex = Regex::new(r"([F|B]*)([L|R]*)").unwrap();
    let captures = regex.captures(seat).unwrap();
    let row = calculate_seat_column_or_row(captures[1].chars(), 0, 127);
    let column = calculate_seat_column_or_row(captures[2].chars(), 0, 7);
    return row * 8 + column
}

fn calculate_seat_column_or_row(mut seat:Chars<'_>, lower:i32, higher:i32) -> i32{
    if lower == higher {
        return lower;
    }
    let half = (higher - lower)/2;
    let char = seat.next().unwrap();
    if char == 'F' || char == 'L' {
        return calculate_seat_column_or_row(seat, lower, lower + half);
    }
    else {
        return calculate_seat_column_or_row(seat, lower + half + 1, higher);
    }
}

#[cfg(test)]
mod tests {
    use crate::day5::parse_seat;

    #[test]
    fn test_parse_seat() {
        assert_eq!(parse_seat("BFFFBBFRRR"), 567);
        assert_eq!(parse_seat("FFFBBBFRRR"), 119);
        assert_eq!(parse_seat("BBFFBBFRLL"), 820);
    }
}
