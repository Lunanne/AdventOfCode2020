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

pub(crate) fn part2(numbers: Vec<i32>) -> (i32, i32, i32) {
    let mut queue = VecDeque::from_iter(numbers.iter());
    let secondQueue = queue.clone();
    while !queue.is_empty() {
        let x: i32 = *queue.pop_front().unwrap();
        for &y in queue.iter() {
            for &z in secondQueue.iter() {
                if x + y + z == 2020 {
                    return (x, *y, *z);
                }
            }
        }
    }
    return (0, 0, 0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(part1(input), (1721, 299));
    }

    #[test]
    fn test_part2() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(part2(input), (979, 366, 675));
    }
}
