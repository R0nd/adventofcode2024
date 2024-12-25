use std::{collections::HashMap, str::Lines};

fn parse(mut iter: Lines) -> (Vec<&str>, Vec<&str>) {
    let towels = iter.next().unwrap().split(", ").collect();
    let patterns = iter.skip(1).collect();

    (towels, patterns)
}

fn is_possible(towels: &Vec<&str>, pattern: &str) -> bool {
    pattern.is_empty()
        || towels.iter().any(|t| match pattern.strip_prefix(t) {
            Some(p) => is_possible(towels, p),
            None => false,
        })
}

fn process_part1(towels: &Vec<&str>, patterns: &Vec<&str>) -> usize {
    patterns.iter().filter(|p| is_possible(towels, p)).count()
}

fn possible_combinations<'a>(
    towels: &Vec<&str>,
    pattern: &'a str,
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if pattern.is_empty() {
        1
    } else {
        match cache.get(pattern) {
            Some(result) => *result,
            _ => {
                let result = towels
                    .iter()
                    .map(|t| match pattern.strip_prefix(t) {
                        Some(p) => possible_combinations(towels, p, cache),
                        None => 0,
                    })
                    .sum();
                cache.insert(pattern, result);
                result
            }
        }
    }
}

fn process_part2(towels: &Vec<&str>, patterns: &Vec<&str>) -> usize {
    patterns
        .iter()
        .map(|p| possible_combinations(towels, p, &mut HashMap::new()))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let (towels, patterns) = parse(INPUT.lines());
        let result = process_part1(&towels, &patterns);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_part2() {
        let (towels, patterns) = parse(INPUT.lines());
        let result = process_part2(&towels, &patterns);
        assert_eq!(result, 16);
    }
}

fn main() {
    let input = include_str!("input.txt");
    let (towels, patterns) = parse(input.lines());

    let result_part1 = process_part1(&towels, &patterns);
    println!("{}", result_part1);

    let result_part2 = process_part2(&towels, &patterns);
    println!("{}", result_part2);
}
