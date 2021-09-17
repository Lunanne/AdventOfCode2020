use std::borrow::{Borrow, BorrowMut};
use std::cmp::min;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::iter::FromIterator;

pub(crate) fn part1(numbers: Vec<i32>) -> i32 {
    let mut adapters = numbers.clone();
    adapters.push(0);
    adapters.sort();
    adapters.push(adapters.last().unwrap() + 3);
    let mut count_differences = HashMap::<i32, i32>::new();
    for x in 1..adapters.len() {
        let step = adapters[x] - adapters[x - 1];
        let current_value = count_differences.get(&step).unwrap_or_else(|| &{ 0 });
        count_differences.insert(step, current_value + 1);
    }
    return count_differences[&1] * count_differences[&3]
}

pub(crate) fn part2(numbers: Vec<i32>) -> i64 {
    let mut adapters = numbers.clone();
    adapters.push(0);
    adapters.sort();
    adapters.push(adapters.last().unwrap() + 3);
    let mut found = Vec::new();
    found.resize((adapters.clone().into_iter().max().unwrap()  as usize) + 1, None);
    return get_combinations(adapters, 0, &mut found);
}

pub(crate) fn part2_attempt2(numbers: Vec<i32>) -> i64 {
    let mut adapters = numbers.clone();
    adapters.push(0);
    adapters.sort();
    adapters.push(adapters.last().unwrap() + 3);
    let mut result = 1;
    for (idx, head) in (&adapters).into_iter().enumerate() {
        if idx +1 >= adapters.len() {
            return result;
        }

        let tail = adapters.clone().split_off(idx + 1);
        let chunk_opt = tail.chunks(3).next();
        if chunk_opt.is_some() {
            let chunk = chunk_opt.unwrap();

            let options = chunk.into_iter().filter(|c| (*c - head) <= 3).count();
            if options > 1 {
                println!("found {} options for {}", options, head);
                result += options as i64;
            }
        }
    }
    return result as i64;
}

pub(crate) fn part2_ruud(numbers: Vec<i32>) -> i64 {
    let translated:Vec<u32> = numbers.into_iter().map(|c| c as u32).collect();
   return count_paths_from_outlet_to_device(&*translated, 3) as i64;
}


fn get_combinations(mut rest: Vec<i32>, mut count: i64, found: & mut Vec<Option<i64>>) -> i64 {
    let current = rest.pop();//pops last item
    if current.is_none() {
        return count + 1;
    }

    if found.get(current.unwrap() as usize).unwrap().is_some(){
        return found.get(current.unwrap()as usize).unwrap().unwrap();
    }
    // println!("current {} still to go {}", current.unwrap(), rest.len());
    let mut safe_rest = rest.clone();
    safe_rest.pop();
    let chunk_opt = safe_rest.rchunks(2).next();
    if chunk_opt.is_some() && chunk_opt.unwrap().len() > 1 {
        let mut chunk: Vec<i32> = Vec::from(chunk_opt.unwrap());
        chunk.reverse();
        for i in 0..chunk.len() {
            if current.unwrap() - chunk[i] <= 3 {
                let result = get_combinations(safe_rest.clone(), 0, found);
                found.insert(chunk[i] as usize, Some(result));
                count += result;
            }
        }
    }

    let result2 = get_combinations(rest.clone(), count.clone(), found);
    found.insert(current.unwrap() as usize, Some(result2));
    return result2;
}


//// Copied from Ruud van der Weide
/// Counts the number of paths from start to end, where each individual step does not exceed a
/// specified threshold.
///
/// We iterate through the list from end to start, and keep track per element how many paths there
/// are from the _current_ element to the end. We can find the next count, by simply checking which
/// elements can be reached from our _current_ element, and then sum the number of paths for those
/// elements. For example, let's say we have the input list [1, 3, 4, 5] and a max jolt
/// difference of 3. We start at 0, and end at the highest element + 3, so let's add those elements
/// to the list to get [0, 1, 3, 4, 5, 8]. We loop through this list in reverse order, so let's
/// start by considering how many paths exist from 5 to 8: Only one single path. Next is 4. The only
/// element that can be reached from 4 is 5, so the number of paths from 4 to 8 is equal to the
/// number of paths from 5 to 8: Only one single path. Next is 3. There are two elements that can be
/// reached from 3: 4 and 5. There is one path from 4 to 8 and one path from 5 to 8, so the number
/// of paths from 3 to 8 is two. We can keep this up all the way to 0, to find that the total number
/// of paths is two.
///
/// One final optimisation: Since the elements in the list are unique, and only have integer values,
/// we only need to keep track of `max_jolt_difference` elements, since each step adds at least one.
fn count_paths_from_outlet_to_device(joltage_ratings: &[u32], max_jolt_difference: u32) -> u64 {
    let mut joltage_ratings = joltage_ratings.to_vec();
    joltage_ratings.push(0); // we start at 0
    joltage_ratings.sort_unstable();
    joltage_ratings.push(joltage_ratings[joltage_ratings.len() - 1] + max_jolt_difference); // and end at the target
    let joltage_ratings = joltage_ratings;

    let result_size = max_jolt_difference as usize;
    let mut result: Vec<u64> = vec![0; result_size];
    result[(joltage_ratings.len() - 1) % result_size] = 1;

    for (idx, joltage_rating) in joltage_ratings.iter().enumerate().rev().skip(1) {
        result[idx % result_size] = joltage_ratings
            .iter()
            .skip(idx + 1)
            .take_while(|&&next_joltage_rating| {
                next_joltage_rating - joltage_rating <= max_jolt_difference
            })
            .enumerate()
            .map(|(pos, _)| result[(pos + idx + 1) % result_size])
            .sum();
    }

    result[0]
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    use utils::read_lines;

    use crate::day10::{part1, part2, part2_attempt2, part2_ruud};

    #[test]
    fn test_part1() {
        let input_text = include_str!("../input/day10_test_input.txt");
        let input = read_lines(input_text);
        let part1result = part1(input.into_iter().map(|s| s.parse().unwrap()).collect());
        assert_eq!(part1result, 35);
    }

    #[test]
    fn test_part1_2() {
        let input_text = include_str!("../input/day10_test_input2.txt");
        let input = read_lines(input_text);
        let part1result = part1(input.into_iter().map(|s| s.parse().unwrap()).collect());
        assert_eq!(part1result, 220);
    }

    #[test]
    fn test_part2() {
        let input_text = include_str!("../input/day10_test_input.txt");
        let input = read_lines(input_text);
        let part2result = part2_ruud(input.into_iter().map(|s| s.parse().unwrap()).collect());
        assert_eq!(part2result, 8);
    }

    #[test]
    fn test_part2_2() {
        let input_text = include_str!("../input/day10_test_input2.txt");
        let input = read_lines(input_text);
        let part2result = part2_ruud(input.into_iter().map(|s| s.parse().unwrap()).collect());
        assert_eq!(part2result, 19208);
    }
}
