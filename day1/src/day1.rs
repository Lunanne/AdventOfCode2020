use std::collections::VecDeque;
use std::iter::FromIterator;

pub(crate) fn part1(numbers: Vec<i32>) -> (i32, i32) {
    let mut queue = VecDeque::from_iter(numbers.iter());
    while !queue.is_empty() {
        let x: i32 = *queue.pop_front().unwrap();
        for &y in queue.iter() {
            if x + y == 2020 {
                return (x, *y);
            }
        }
    }
    return (0, 0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(part1(input), (1721, 299));
    }
}
