use regex::Regex;


pub(crate) fn part1(lines: Vec<&str>) -> i32 {
    let regex: &str = r"(\d*)-(\d*) (\w): (\w*)";
    let mut count: i32 = 0;
    let re = Regex::new(regex).unwrap();
    for line in lines {
        let (low, high, character, password) = parse(&re, line);
        let ccount: i32 = password.matches(&character.to_string()).count() as i32;
        if ccount >= low && ccount <= high {
            count += 1;
        }
    }
    return count;
}

pub(crate) fn part2(lines: Vec<&str>) -> i32 {
    let regex: &str = r"(\d*)-(\d*) (\w): (\w*)";
    let mut count: i32 = 0;
    let re = Regex::new(regex).unwrap();
    for line in lines {
        let (low, high, character, password) = parse(&re, line);
        let mut ccount  = 0;
        if password.chars().nth((low - 1) as usize).unwrap().eq(&character) {
            ccount +=1;
        }
        if password.chars().nth((high - 1) as usize).unwrap().eq(&character) {
            ccount +=1;
        }
        if(ccount ==1){
            count +=1;
        }
    }
    return count;
}

fn parse(regex: &Regex, line: &str) -> (i32, i32, char, String){
    let cap = regex.captures(line).unwrap();
    let low: i32 = (&cap[1]).parse().unwrap();
    let high: i32 = (&cap[2]).parse().unwrap();
    let character = &cap[3];
    let password = &cap[4];
    return (low, high, character.parse().unwrap(), password.parse().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];
        assert_eq!(part1(input), 2);
    }

    #[test]
    fn test_part2() {
        let input = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];
        assert_eq!(part2(input), 1);
    }
}
