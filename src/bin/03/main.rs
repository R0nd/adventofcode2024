use regex::Regex;
use std::str::Lines;

fn parse(iter: Lines) -> String {
    iter.collect::<Vec<&str>>().join("")
}

fn process_part1(instructions: &str) -> i32 {
    let instruction_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    instruction_regex
        .captures_iter(instructions)
        .map(|caps| caps.extract())
        .map(|(_, vals)| vals.map(|v| v.parse::<i32>().unwrap()))
        .map(|[a, b]| a * b)
        .sum()
}

fn process_part2(instructions: &str) -> i32 {
    let dont_regex = Regex::new(r"don't\(\).*?(do\(\))|$").unwrap();

    let s = dont_regex.replace_all(instructions, "").into_owned();

    process_part1(&s)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("test_input.txt");
    static INPUT2: &str = include_str!("test_input2.txt");

    #[test]
    fn test_part1() {
        let instructions = parse(INPUT.lines());
        let result = process_part1(&instructions);
        assert_eq!(result, 161);
    }

    #[test]
    fn test_part2() {
        let instructions = parse(INPUT2.lines());
        let result = process_part2(&instructions);
        assert_eq!(result, 48);
    }
}

fn main() {
    let input = include_str!("input.txt");
    let instructions = parse(input.lines());

    let result_part1 = process_part1(&instructions);
    println!("{}", result_part1);

    let result_part2 = process_part2(&instructions);
    println!("{}", result_part2);
}
