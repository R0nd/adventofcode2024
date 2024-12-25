use std::str::Lines;

use itertools::Itertools;

type Pattern = [usize; 5];

fn parse(iter: Lines) -> (Vec<Pattern>, Vec<Pattern>) {
    iter.chunks(8)
        .into_iter()
        .fold((vec![], vec![]), |(mut locks, mut keys), mut next| {
            let header = next.next().unwrap();
            let lines = next.take(5).collect::<Vec<_>>();
            let pattern = (0..5)
                .map(|i| {
                    lines
                        .iter()
                        .filter(|s| s.chars().nth(i) == Some('#'))
                        .count()
                })
                .collect::<Vec<_>>();
            if header.starts_with('#') {
                &mut locks
            } else {
                &mut keys
            }
            .push(pattern.try_into().unwrap());
            (locks, keys)
        })
}

fn fit(lock: &Pattern) -> impl Fn(&&Pattern) -> bool + '_ {
    |key| lock.iter().zip(key.iter()).all(|(a, b)| a + b <= 5)
}

fn process_part1(locks: &Vec<Pattern>, keys: &Vec<Pattern>) -> usize {
    locks
        .iter()
        .map(|lock| keys.iter().filter(fit(lock)).count())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let (locks, keys) = parse(INPUT.lines());
        let result = process_part1(&locks, &keys);
        assert_eq!(result, 3);
    }
}

fn main() {
    let input = include_str!("input.txt");
    let (locks, keys) = parse(input.lines());

    let result_part1 = process_part1(&locks, &keys);
    println!("{}", result_part1);
}
