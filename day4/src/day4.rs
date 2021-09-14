use regex::Regex;
use std::borrow::Borrow;

const BYR: &str = "byr";
const IYR: &str = "iyr";
const EYR: &str = "eyr";
const HGT: &str = "hgt";
const HCL: &str = "hcl";
const ECL: &str = "ecl";
const PID: &str = "pid";

pub(crate) fn part1(passports: Vec<&str>) -> i32 {
    let mut count = 0;
    for passport in passports {
        if passport.contains(BYR) &&
            passport.contains(IYR) &&
            passport.contains(EYR) &&
            passport.contains(HGT) &&
            passport.contains(HCL) &&
            passport.contains(ECL) &&
            passport.contains(PID) {
            count += 1;
        }
    }
    return count;
}

pub(crate) fn part2(passports: Vec<&str>) -> i32 {
    let mut count = 0;
    let byr_regex = Regex::new(r"byr:(\w*)").unwrap();
    let iyr_regex = Regex::new(r"iyr:(\w*)").unwrap();
    let eyr_regex = Regex::new(r"eyr:(\w*)").unwrap();
    let hgt_regex = Regex::new(r"hgt:(\w*)").unwrap();
    let hcl_regex = Regex::new(r"hcl:(#?\w*)").unwrap();
    let ecl_regex = Regex::new(r"ecl:(\w*)").unwrap();
    let pid_regex = Regex::new(r"pid:(\w*)").unwrap();

    for passport in passports {
        if passport.contains(BYR) &&
            passport.contains(IYR) &&
            passport.contains(EYR) &&
            passport.contains(HGT) &&
            passport.contains(HCL) &&
            passport.contains(ECL) &&
            passport.contains(PID) {
            let byr_caps = byr_regex.captures(passport).unwrap();
            let iyr_caps = iyr_regex.captures(passport).unwrap();
            let eyr_caps = eyr_regex.captures(passport).unwrap();
            let hgt_caps = hgt_regex.captures(passport).unwrap();
            let hcl_caps = hcl_regex.captures(passport).unwrap();
            let ecl_caps = ecl_regex.captures(passport).unwrap();
            let pid_caps = pid_regex.captures(passport).unwrap();
            if validate_birth_year(str::parse(&byr_caps[1]).unwrap()) &&
                validate_issue_year(str::parse(&iyr_caps[1]).unwrap()) &&
                validate_experation_year(str::parse(&eyr_caps[1]).unwrap()) &&
                validate_height(&hgt_caps[1]) &&
                validate_hair_colour(&hcl_caps[1]) &&
                validate_eye_colour(&ecl_caps[1]) &&
                validate_passport_id(&pid_caps[1]){
                count +=1;
            }
        }
    }
    return count;
}

fn validate_birth_year(year: i32) -> bool{
    return year >= 1920 && year <= 2002;
}

fn validate_issue_year(year: i32) -> bool{
    return year >= 2010 && year <= 2020;
}

fn validate_experation_year(year: i32) -> bool{
    return year >= 2020 && year <= 2030;
}

fn validate_height(height: &str) -> bool{
    let regex = Regex::new(r"(\d*)(\w*)").unwrap();
    let caps = regex.captures(height).unwrap();
    let number:i32 = str::parse(caps[1].borrow()).unwrap();
    let format: &str = caps[2].borrow();
    return if format == "in" {
        number >= 59 && number <= 76
    } else if format == "cm" {
        number >= 150 && number <= 193
    } else {
        false
    }
}

fn validate_hair_colour(colour: &str) -> bool{
    let regex = Regex::new(r"#[a-f|0-9]{6}").unwrap();
    return regex.is_match(colour);
}

fn validate_eye_colour(colour: &str) -> bool{
    let valid:Vec<&str> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    return valid.contains(&colour);
}

fn validate_passport_id(id: &str) -> bool{
    let regex = Regex::new(r"\b[0-9]{9}\b").unwrap();
    return regex.is_match(id);
}

#[cfg(test)]
mod tests {
    use std::iter::FromIterator;
    use crate::day4::{part1, validate_birth_year, validate_height, validate_hair_colour, validate_eye_colour, validate_passport_id, validate_experation_year, part2};

    #[test]
    fn test_part1() {
        let input_text = include_str!("../input/day4_test_input.txt");
        let input = Vec::from_iter(input_text.split("\n\n"));
        let part1result = part1(input);
        assert_eq!(part1result, 2);
    }

    #[test]
    fn test_validate_birth_year() {
        assert_eq!(validate_birth_year(2002), true);
        assert_eq!(validate_birth_year(2003), false);
    }

    #[test]
    fn test_validate_experation_year() {
        assert_eq!(validate_experation_year(2025), true);
        assert_eq!(validate_experation_year(1972), false);
    }

    #[test]
    fn test_validate_height() {
        assert_eq!(validate_height("60in"), true);
        assert_eq!(validate_height("190cm"), true);
        assert_eq!(validate_height("190in"), false);
        assert_eq!(validate_height("190"), false);
    }

    #[test]
    fn test_validate_hcl() {
        assert_eq!(validate_hair_colour("#123abc"), true);
        assert_eq!(validate_hair_colour("#123abz"), false);
        assert_eq!(validate_hair_colour("123abc"), false);
    }

    #[test]
    fn test_validate_ecl() {
        assert_eq!(validate_eye_colour("brn"), true);
        assert_eq!(validate_eye_colour("wat"), false);
    }

    #[test]
    fn test_validate_pid() {
        assert_eq!(validate_passport_id("000000001"), true);
        assert_eq!(validate_passport_id("0123456789"), false);
    }

    #[test]
    fn test_part2() {
        let input_text = include_str!("../input/day4_test_part2_input.txt");
        let input = Vec::from_iter(input_text.split("\n\n"));
        let part2result = part2(input);
        assert_eq!(part2result, 4);
    }
}
