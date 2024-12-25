use std::{collections::HashMap, str::Lines};

use itertools::Itertools;

fn parse(iter: Lines) -> Vec<usize> {
    iter.map(|s| s.parse().unwrap()).collect()
}

fn next(secret: usize) -> usize {
    const B23: usize = (1 << 24) - 1;
    let a = ((secret << 6) ^ secret) & B23;
    let b = ((a >> 5) ^ a) & B23;
    ((b << 11) ^ b) & B23
}

fn process_part1(ns: &Vec<usize>) -> usize {
    ns.iter()
        .map(|n| (0..2000).fold(*n, |acc, _| next(acc)))
        .sum()
}

fn process_part2(ns: &Vec<usize>) -> usize {
    ns.iter()
        .map(|n| {
            let mut seqs = HashMap::new();
            (0..2000)
                .scan(*n, |acc, _| {
                    let result = Some(*acc % 10);
                    *acc = next(*acc);
                    result
                })
                .scan(0, |prev, next| {
                    let result = Some((next, next as isize - *prev as isize));
                    *prev = next;
                    result
                })
                .skip(1)
                .tuple_windows()
                .for_each(|((_, a), (_, b), (_, c), (n, d))| {
                    seqs.entry([a, b, c, d]).or_insert(n);
                });
            seqs
        })
        .reduce(|mut a, b| {
            b.iter().for_each(|(k, v)| {
                a.entry(*k).and_modify(|n| *n += *v).or_insert(*v);
            });
            a
        })
        .unwrap()
        .values()
        .max()
        .unwrap()
        .clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("test_input.txt");
    static INPUT2: &str = include_str!("test_input2.txt");

    #[test]
    fn test_part1() {
        let ns = parse(INPUT.lines());
        let result = process_part1(&ns);
        assert_eq!(result, 37327623);
    }

    #[test]
    fn test_part2() {
        let ns = parse(INPUT2.lines());
        let result = process_part2(&ns);
        assert_eq!(result, 23);
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
