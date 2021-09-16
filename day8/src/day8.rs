use std::borrow::Borrow;

use regex::Regex;

use crate::operations::{Execute, OperationResult, Operations};

pub(crate) fn part1(program_txt: Vec<String>) -> i32 {
    return run_program(Vec::from(program_txt.borrow())).0.accumelator;
}

pub(crate) fn part2(program_txt: Vec<String>) -> i32 {
    let mut line_number = 0;
    let mut accumelator = 0;
    let loop_lines = run_program(Vec::from(program_txt.borrow())).1;
    for line in loop_lines.clone() {
        let mut new_program_txt: Vec<String> = program_txt.clone();
        let mut modified = false;
        if program_txt.get::<usize>(line).unwrap().starts_with("jmp")
            || program_txt.get::<usize>(line).unwrap().starts_with("nop") {
            let mut instruction: String = new_program_txt.get(line).unwrap().to_string();
            if instruction.starts_with("jmp") {
                instruction = instruction.replace("jmp", "nop");
            } else {
                instruction = instruction.replace("nop", "jmp");
            }

            std::mem::replace(&mut new_program_txt[line], instruction);
            modified = true;
        }
        if modified {
            let result = run_program(Vec::from(new_program_txt.borrow())).0;
            line_number = result.line_number as usize;
            accumelator = result.accumelator;
            if &line_number >= &program_txt.len() {
                break;
            }
        }
    }
    return accumelator;
}

fn run_program(program_txt: Vec<String>) -> (OperationResult, Vec<usize>) {
    let operations = Operations::new();
    let regex = Regex::new(r"(\w{3}) ([+|-]?\d*)").unwrap();
    let mut used_line_numbers = Vec::<usize>::new();
    let mut line_number: i32 = 0;
    let mut accumelator = 0;
    while !used_line_numbers.contains(&(line_number as usize)) && line_number < program_txt.len() as i32 {
        let op_txt_caps = regex.captures(&program_txt[line_number as usize]).unwrap();
        let operation = op_txt_caps[1].borrow();
        let argument: i32 = op_txt_caps[2].borrow().parse().unwrap();
        used_line_numbers.push(line_number as usize);
        let result = operations.execute(operation, argument, line_number, accumelator);
        line_number = result.line_number;
        accumelator = result.accumelator;
    }
    return (OperationResult { line_number, accumelator }, used_line_numbers);
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    use utils::read_lines;

    use crate::day8::{part1, part2};

    #[test]
    fn test_part1() {
        let input_text = include_str!("../input/day8_test_input.txt");
        let input = read_lines(input_text);
        let part1result = part1(input.into_iter().map(|s| s.to_string()).collect());
        assert_eq!(part1result, 5);
    }

    #[test]
    fn test_part2() {
        let input_text = include_str!("../input/day8_test_input.txt");
        let input = read_lines(input_text);
        let part2result = part2(&input.into_iter().map(|s| s.to_string()).collect());
        assert_eq!(part2result, 8);
    }
}
