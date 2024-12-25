use std::{
    cmp::Ordering,
    collections::HashMap,
    iter::once,
    ops::{Add, Sub},
    str::Lines,
};

use itertools::Itertools;

const NUMERIC_PAD: [&str; 4] = ["789", "456", "123", " 0A"];

const DIRECTIONAL_PAD: [&str; 2] = [" ^A", "<v>"];

fn find_position(key: char, pad: &[&str]) -> Point {
    let row = pad
        .iter()
        .enumerate()
        .find(|(_, row)| row.contains(key))
        .unwrap()
        .0;
    let col = pad[row].find(key).unwrap();

    Point(row, col)
}

#[derive(Clone, Copy)]
struct Point(usize, usize);

#[derive(Clone, Copy)]
struct Delta(isize, isize);

fn decompose(delta: Delta) -> Vec<Delta> {
    [
        vec![Delta(delta.0.signum(), 0); delta.0.abs() as usize],
        vec![Delta(0, delta.1.signum()); delta.1.abs() as usize],
    ]
    .concat()
}

fn serialize(delta: &Delta) -> char {
    match delta {
        Delta(-1, 0) => '^',
        Delta(1, 0) => 'v',
        Delta(0, -1) => '<',
        Delta(0, 1) => '>',
        _ => panic!(),
    }
}

impl Sub for Point {
    type Output = Delta;

    fn sub(self, rhs: Self) -> Self::Output {
        Delta(
            self.0 as isize - rhs.0 as isize,
            self.1 as isize - rhs.1 as isize,
        )
    }
}

impl Add<Delta> for Point {
    type Output = Self;

    fn add(self, rhs: Delta) -> Self::Output {
        Self(
            (self.0 as isize + rhs.0) as usize,
            (self.1 as isize + rhs.1) as usize,
        )
    }
}

type Memo = Vec<HashMap<(char, char), usize>>;

fn validate(from: &Point, deltas: &Vec<&Delta>, pad: &[&str]) -> bool {
    let mut current = *from;

    for d in deltas {
        current = current + **d;

        if pad[current.0].chars().nth(current.1) == Some(' ') {
            return false;
        }
    }

    true
}

fn steps(from: char, to: char, pad: &[&str]) -> String {
    let from_position = find_position(from, pad);
    let to_position = find_position(to, pad);
    let delta = to_position - from_position;
    let segments = decompose(delta);
    let mut steps = segments
        .iter()
        .permutations(segments.len())
        .filter(|ds| validate(&from_position, ds, pad))
        .map(|ds| {
            ds.iter()
                .map(|d| serialize(*d))
                .chain(once('A'))
                .collect::<String>()
        })
        .collect::<Vec<_>>();
    steps.sort_by(compare_steps);
    steps.iter().find(is_batched).unwrap().clone()
}

fn compare_steps(a: &String, b: &String) -> Ordering {
    if a.len() < b.len() {
        Ordering::Less
    } else if a.len() > b.len() {
        Ordering::Greater
    } else if a == b {
        Ordering::Equal
    } else {
        let (a_char, b_char) = a.chars().zip(b.chars()).find(|(a, b)| a != b).unwrap();
        match a_char {
            '<' => Ordering::Less,
            '^' if b_char == '<' => Ordering::Greater,
            '^' => Ordering::Less,
            '>' => Ordering::Greater,
            'v' => Ordering::Greater,
            _ => panic!(),
        }
    }
}

fn is_batched(segment: &&String) -> bool {
    for i in 2..segment.len() {
        let last = segment.chars().nth(i).unwrap();
        if segment.chars().nth(i - 1).unwrap() != last && segment[0..i].contains(last) {
            return false;
        }
    }

    true
}

fn encode_inner(input: &str, pad: &[&str]) -> String {
    (0..input.len() - 1)
        .map(|i| {
            (
                input.chars().nth(i).unwrap(),
                input.chars().nth(i + 1).unwrap(),
            )
        })
        .map(|(a, b)| steps(a, b, pad))
        .collect()
}

fn encode_inner_memoized(input: &str, &depth: &usize, memo: &mut Memo) -> usize {
    let prefixed_chunk = prefix(input);
    if depth == 1 {
        encode_inner(&prefixed_chunk, &DIRECTIONAL_PAD).len()
    } else {
        (0..prefixed_chunk.len() - 1)
            .map(|i| {
                (
                    prefixed_chunk.chars().nth(i).unwrap(),
                    prefixed_chunk.chars().nth(i + 1).unwrap(),
                )
            })
            .map(|(a, b)| {
                if let Some(cached) = memo[depth].get(&(a, b)) {
                    *cached
                } else {
                    let result = steps(a, b, &DIRECTIONAL_PAD)
                        .split_inclusive('A')
                        .map(|chunk| encode_inner_memoized(chunk, &(depth - 1), memo))
                        .sum();

                    memo[depth].insert((a, b), result);
                    result
                }
            })
            .sum()
    }
}

fn prefix(s: &str) -> String {
    "A".to_owned() + s
}

fn parse(iter: Lines) -> Vec<String> {
    iter.map(|s| s.to_owned()).collect()
}

fn process(ns: &Vec<String>, depth: &usize) -> usize {
    let mut memo: Memo = vec![HashMap::new(); depth + 1];

    ns.iter()
        .map(|n| {
            let numeric_part = n.strip_suffix('A').unwrap().parse::<usize>().unwrap();

            let length = (0..n.len())
                .map(|i| prefix(n)[i..=i + 1].to_string())
                .map(|w| {
                    encode_inner(&w, &NUMERIC_PAD)
                        .split_inclusive('A')
                        .map(|chunk| encode_inner_memoized(chunk, depth, &mut memo))
                        .sum::<usize>()
                })
                .sum::<usize>();

            numeric_part * length
        })
        .sum()
}

fn process_part1(ns: &Vec<String>) -> usize {
    process(ns, &2)
}

fn process_part2(ns: &Vec<String>) -> usize {
    process(ns, &25)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let ns = parse(INPUT.lines());
        let result = process_part1(&ns);
        assert_eq!(result, 126384);
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
