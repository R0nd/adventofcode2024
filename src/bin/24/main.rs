use std::{cmp::Reverse, collections::HashMap, str::Lines};

use itertools::Itertools;

#[derive(Clone, Copy)]
enum Operator {
    AND,
    OR,
    XOR,
}

fn parse(
    iter: Lines,
) -> (
    HashMap<String, bool>,
    HashMap<String, (String, String, Operator)>,
) {
    let values = iter
        .clone()
        .take_while(|s| !s.is_empty())
        .map(|s| {
            let (a, b) = s.split(": ").tuples().next().unwrap();
            (a.to_owned(), b == "1")
        })
        .collect();

    let rules = iter
        .skip_while(|s| !s.is_empty())
        .skip(1)
        .map(|s| {
            let (left, right) = s.split(" -> ").tuples().next().unwrap();
            let (a, op, b) = left.split_whitespace().tuples().next().unwrap();

            let operator = match op {
                "OR" => Operator::OR,
                "AND" => Operator::AND,
                "XOR" => Operator::XOR,
                _ => panic!(),
            };

            (right.to_owned(), (a.to_owned(), b.to_owned(), operator))
        })
        .collect();

    (values, rules)
}

fn resolve(
    key: &str,
    values: &HashMap<String, bool>,
    rules: &HashMap<String, (String, String, Operator)>,
) -> bool {
    if let Some(v) = values.get(key) {
        *v
    } else {
        let (a, b, op) = rules.get(key).unwrap();

        let a_value = resolve(a, values, rules);
        let b_value = resolve(b, values, rules);
        match op {
            Operator::AND => a_value && b_value,
            Operator::OR => a_value || b_value,
            Operator::XOR => a_value != b_value,
        }
    }
}

fn squash(bits: &Vec<bool>) -> usize {
    bits.iter().fold(0, |acc, v| (acc << 1) ^ *v as usize)
}

fn unsquash(value: usize, length: usize) -> Vec<bool> {
    (0..length).map(|i| ((1 << i) & value) > 0).collect()
}

fn process_part1(
    values: &HashMap<String, bool>,
    rules: &HashMap<String, (String, String, Operator)>,
) -> usize {
    squash(
        &rules
            .keys()
            .filter(|k| k.starts_with('z'))
            .sorted()
            .rev()
            .map(|k| resolve(k, values, rules))
            .collect(),
    )
}

fn find_values(values: &HashMap<String, bool>, prefix: char) -> Vec<bool> {
    values
        .iter()
        .filter(|(k, _)| k.starts_with(prefix))
        .sorted_by_key(|(k, _)| Reverse(*k))
        .map(|(_, v)| *v)
        .collect()
}

fn swap(k1: &str, k2: &str, rules: &mut HashMap<String, (String, String, Operator)>) {
    let a = rules.remove(k1).unwrap();
    let b = rules.remove(k2).unwrap();
    rules.insert(k1.to_string(), b);
    rules.insert(k2.to_string(), a);
}

fn process_part2(
    values: &HashMap<String, bool>,
    rules: &HashMap<String, (String, String, Operator)>,
) {
    let mut swapped_rules = rules.clone();

    swap("fhg", "z17", &mut swapped_rules);
    swap("tnc", "z39", &mut swapped_rules);
    swap("vcf", "z10", &mut swapped_rules);
    swap("fsq", "dvb", &mut swapped_rules);

    let z = process_part1(values, &swapped_rules);
    let x = squash(&find_values(values, 'x'));
    let y = squash(&find_values(values, 'y'));

    let mask = (x + y) ^ z;
    let wrong_bits = unsquash(mask, values.keys().filter(|k| k.starts_with('x')).count())
        .iter()
        .enumerate()
        .filter(|(_, v)| **v)
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    println!("{:?}", wrong_bits);
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let (values, rules) = parse(INPUT.lines());
        let result = process_part1(&values, &rules);
        assert_eq!(result, 2024);
    }
}

fn main() {
    let input = include_str!("input.txt");
    let (values, rules) = parse(input.lines());

    let result_part1 = process_part1(&values, &rules);
    println!("{}", result_part1);

    process_part2(&values, &rules);
}
