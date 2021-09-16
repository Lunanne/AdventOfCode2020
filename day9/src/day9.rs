use std::borrow::Borrow;
use std::collections::{HashSet, VecDeque, HashMap, BTreeSet, BTreeMap};
use std::iter::FromIterator;
use std::cmp::min;

pub(crate) fn part1(numbers: Vec<i64>) -> i64 {
    return find_wrong_number(numbers, 25)
}

pub(crate) fn part2(numbers: Vec<i64>) -> i64 {
    return find_weakest_point(numbers, 25)
}

fn find_weakest_point(numbers: Vec<i64>, preamble:usize) -> i64 {
    let wrong_number = find_wrong_number(Vec::from(numbers.borrow()), preamble);
    let mut map_sets = BTreeMap::<i64, VecDeque<i64>>::new();
    for i in 2..numbers.len() {
        map_sets.append(&mut find_right_set(Vec::from(numbers.borrow()), i, wrong_number));
    }
    let min_key = map_sets.keys().min_by_key(|x| {x.clone()}).unwrap();
    return min_key.clone();
}

fn find_right_set(numbers: Vec<i64>, preamble: usize, target:i64) -> BTreeMap<i64, VecDeque<i64>> {
    let mut earlier_numbers = VecDeque::<i64>::new();
    let mut map_sets = BTreeMap::<i64, VecDeque<i64>>::new();
    for (idx, number) in numbers.into_iter().enumerate() {
        if idx < preamble {
            earlier_numbers.push_back(number);
        }
        else {
            let sum = calculate_collective_sum(earlier_numbers.borrow());
            if sum == target {
                let mut results = earlier_numbers.borrow().into_iter().collect::<Vec<_>>();
                results.sort();
                let first: i64 = **results.first().unwrap();
                let last: i64 = **results.last().unwrap();
                map_sets.insert(first+last, earlier_numbers.clone());
            }
            earlier_numbers.pop_front();
            earlier_numbers.push_back(number);
        }
    }
    return map_sets;
}

fn find_wrong_number(numbers: Vec<i64>, preamble: usize) -> i64 {
    let mut earlier_numbers = VecDeque::<i64>::new();
    for (idx, number) in numbers.into_iter().enumerate() {
        if idx < preamble {
            earlier_numbers.push_back(number);
        }
        else {
            let sums: BTreeSet<i64> = calculate_sums(earlier_numbers.borrow());

            if !sums.contains(&number) {
                return number;
            }
            earlier_numbers.pop_front();
            earlier_numbers.push_back(number);
        }
    }

    return 0;
}

fn calculate_collective_sum(numbers: &VecDeque<i64>) -> i64 {
    return numbers.into_iter().sum();
}

fn calculate_sums(numbers: &VecDeque<i64>) -> BTreeSet<i64> {
    let mut set = BTreeSet::new();
    let mut queue = VecDeque::from_iter(numbers.iter());
    while !queue.is_empty() {
        let x = queue.pop_front().unwrap();
        for number in numbers {
            set.insert(x + number);
        }
    }
    return set;
}


#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    use utils::read_lines;
    use crate::day9::{part1, find_wrong_number, find_weakest_point};

    #[test]
    fn test_part1() {
        let input_text = include_str!("../input/day9_test_input.txt");
        let input = read_lines(input_text);
        let part1result = find_wrong_number(input.into_iter().map(|s| s.parse().unwrap()).collect(), 5);
        assert_eq!(part1result, 127);
    }

    #[test]
    fn test_part2() {
        let input_text = include_str!("../input/day9_test_input.txt");
        let input = read_lines(input_text);
        let part2result = find_weakest_point(input.into_iter().map(|s| s.parse().unwrap()).collect(), 5);
        assert_eq!(part2result, 62);
    }
}
