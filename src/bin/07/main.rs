use std::{
    iter::once,
    ops::{Add, Mul},
    str::Lines,
};

fn parse(iter: Lines) -> Vec<(i64, Vec<i64>)> {
    iter.map(|s| {
        let [left, right] = s.split(':').collect::<Vec<_>>()[..] else {
            panic!();
        };

        (
            left.parse().unwrap(),
            right
                .split_whitespace()
                .map(str::parse)
                .map(Result::unwrap)
                .collect(),
        )
    })
    .collect()
}

fn is_solvable(test_value: i64, operators: &Vec<fn(i64, i64) -> i64>, values: &Vec<i64>) -> bool {
    match values[..] {
        [v, ..] if v > test_value => false,
        [v] => test_value == v,
        [a, b, ..] => operators.iter().any(|op| {
            is_solvable(
                test_value,
                operators,
                &once(op(a, b))
                    .chain(values.iter().skip(2).cloned())
                    .collect(),
            )
        }),
        _ => panic!(),
    }
}

fn process_part1(ns: &Vec<(i64, Vec<i64>)>) -> i64 {
    ns.iter()
        .filter(|(test_value, values)| is_solvable(*test_value, &vec![Add::add, Mul::mul], values))
        .map(|(test_value, _)| test_value)
        .sum()
}

fn process_part2(ns: &Vec<(i64, Vec<i64>)>) -> i64 {
    ns.iter()
        .filter(|(test_value, values)| {
            is_solvable(
                *test_value,
                &vec![Add::add, Mul::mul, |a, b| {
                    a * (10 as i64).pow(((b as f64).log10() + 1 as f64).floor() as u32) + b
                }],
                values,
            )
        })
        .map(|(test_value, _)| test_value)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let ns = parse(INPUT.lines());
        let result = process_part1(&ns);
        assert_eq!(result, 3749);
    }

    #[test]
    fn test_part2() {
        let ns = parse(INPUT.lines());
        let result = process_part2(&ns);
        assert_eq!(result, 11387);
    }
}

fn main() {
    let input = include_str!("input.txt");
    let ns = parse(input.lines());

    let result_part1 = process_part1(&ns);
    println!("{}", result_part1);

    let result_part2 = process_part2(&ns);
    println!("{}", result_part2);
}
