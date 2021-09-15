use std::collections::{HashSet, HashMap};
use std::borrow::Borrow;

pub(crate) fn part1(custom_questions: Vec<&str>) -> usize {
    let mut groups = Vec::<HashSet<char>>::new();
    for grp_str in custom_questions {
        let mut set = HashSet::new();
        for char in grp_str.chars().into_iter().filter(|c| c != &'\n'){
            set.insert(char);
        }
        groups.push(set);
    }
    let mut count = 0;
    for group in groups {
        count += group.len();
    }
    return count;
}

pub(crate) fn part2(custom_questions: Vec<&str>) -> usize {
    let mut groups = Vec::<HashMap<char, usize>>::new();
    for grp_str in custom_questions {
        let mut map = HashMap::new();
        for char in grp_str.chars().into_iter(){
            map.insert(char, grp_str.matches(&char.to_string()).count());
        }
        groups.push(map);
    }
    let mut count = 0;
    for group in groups {
        let required = group.get(&'\n').unwrap_or(&(0 as usize)) + 1;
        let c = group.borrow().into_iter().filter(|(k, v)| k!=&&'\n').filter(|(k, v)| v >= &&required).count();
        count += c;
    }
    return count;
}

#[cfg(test)]
mod tests {
    use std::iter::FromIterator;

    use crate::day6::{part1, part2};

    #[test]
    fn test_part1() {
        let input_text = include_str!("../input/day6_test_input.txt");
        let input = Vec::from_iter(input_text.split("\n\n"));
        let part1result = part1(input);
        assert_eq!(part1result, 11);
    }

    #[test]
    fn test_part2() {
        let input_text = include_str!("../input/day6_test_input.txt");
        let input = Vec::from_iter(input_text.split("\n\n"));
        let part2result = part2(input);
        assert_eq!(part2result, 6);
    }

}
