use std::borrow::Borrow;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::str::Chars;

use regex::Regex;

use crate::bag::Bag;
use crate::bag::HasBag;

pub(crate) fn part1<'a>(bags_txt: Vec<&str>) -> usize {
    let bags = parse_bags(bags_txt);
    return collect_bags("shiny gold", bags.borrow()).len();
}

pub(crate) fn part2<'a>(bags_txt: Vec<&str>) -> i32 {
    let bags = parse_bags(bags_txt);
    return count_required_bags("shiny gold", bags.borrow());
}

fn count_required_bags(colour:&str, bags: &Vec<Bag>) -> i32{
    let mut required: i32 = 0;
    let bag = bags.into_iter().find(|b| b.colour == colour).unwrap();
    for (bag, count) in &bag.contents{
        // println!("colour {} count {} required {}", bag.colour, count, required);
        required += count + count * count_required_bags(&*bag.colour, bags.borrow());
    }
    return required;
}


fn collect_bags<'a>(colour: &str, bags: &'a Vec<Bag>) -> HashSet<&'a str> {
    let mut bag_set= HashSet::<&'a str>::new();
    for bag in bags {
        if bag.has_bag(colour) {
            // println!("bag {} can contain {}", bag.colour, colour);
            bag_set.insert(&bag.colour);
            bag_set.extend(collect_bags(bag.colour.borrow(), bags.borrow()));
        }
    }
    return bag_set;
}

fn parse_bags(bags_txt: Vec<&str>) -> Vec<Bag> {
    let mut bags = Vec::new();
    let regex = Regex::new(r"(\w* \w*) bags contain ([\d|\W|\w]*).").unwrap();
    let inner_regex = Regex::new(r"(\d?) (\w* \w*) bag[s]?").unwrap();
    for bag in bags_txt {
        let caps = regex.captures(bag).unwrap();
        let mut inner_bags = BTreeMap::<Bag, i32>::new();
        let colour = caps[1].borrow();
        let inner_bags_txt = caps[2].borrow();
        if inner_bags_txt != "no other bags" {
            for inner_bag_txt in inner_bags_txt.split(",").into_iter() {
                let inner_caps = inner_regex.captures(inner_bag_txt).unwrap();
                let inner_bag = Bag {
                    colour: inner_caps[2].borrow().parse().unwrap(),
                    contents: BTreeMap::new()
                };
                inner_bags.insert(inner_bag, inner_caps[1].parse().unwrap());
            }
        }
        let outer_bag = Bag {
            colour: colour.parse().unwrap(),
            contents: inner_bags
        };
        bags.push(outer_bag);
    }
    return bags;
}

#[cfg(test)]
mod tests {
    use utils::read_lines;

    use crate::day7::{part1, part2};

    #[test]
    fn test_part1() {
        let input_text = include_str!("../input/day7_test_input.txt");
        let input = read_lines(input_text);
        let part1result = part1(input);
        assert_eq!(part1result, 4);
    }

    #[test]
    fn test_part2_test1() {
        let input_text = include_str!("../input/day7_test_input.txt");
        let input = read_lines(input_text);
        let part2result = part2(input);
        assert_eq!(part2result, 32);
    }

    #[test]
    fn test_part2_test2() {
        let input_text = include_str!("../input/day7_test_part2_input.txt");
        let input = read_lines(input_text);
        let part2result = part2(input);
        assert_eq!(part2result, 126);
    }
}
