use std::{collections::HashSet, str::Lines};

use itertools::Itertools;

fn parse(iter: Lines) -> Vec<Vec<char>> {
    iter.map(|s| s.chars().collect()).collect()
}

fn flood_fill(
    ns: &mut Vec<Vec<(char, bool)>>,
    row: usize,
    col: usize,
) -> (usize, usize, HashSet<(usize, usize)>) {
    let (n, _) = ns[row][col];
    ns[row][col].1 = true;
    let neighbors = [
        if row > 0 { Some((row - 1, col)) } else { None },
        if col > 0 { Some((row, col - 1)) } else { None },
        if col < ns[0].len() - 1 {
            Some((row, col + 1))
        } else {
            None
        },
        if row < ns.len() - 1 {
            Some((row + 1, col))
        } else {
            None
        },
    ]
    .iter()
    .filter_map(|x| *x)
    .filter(|&(r, c)| ns[r][c].0 == n)
    .collect::<Vec<_>>();

    neighbors
        .iter()
        .filter_map(|&(r, c)| {
            if ns[r][c].1 {
                None
            } else {
                Some(flood_fill(ns, r, c))
            }
        })
        .fold(
            (
                1,
                4 - neighbors.len(),
                [(row, col)].iter().copied().collect::<HashSet<_>>(),
            ),
            |(acc_a, acc_b, acc_set), (a, b, set)| {
                (
                    acc_a + a,
                    acc_b + b,
                    acc_set.iter().chain(set.iter()).copied().collect(),
                )
            },
        )
}

fn process_internal(ns: &Vec<Vec<char>>) -> Vec<(usize, usize, HashSet<(usize, usize)>)> {
    let mut state = ns
        .iter()
        .map(|row| row.iter().map(|v| (*v, false)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut acc = vec![];

    while let Some((r, c, _)) = state
        .iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .map(move |(c, (_, visited))| (r, c, visited))
        })
        .find(|(_, _, visited)| !**visited)
    {
        acc.push(flood_fill(&mut state, r, c));
    }

    acc
}

fn sides(set: &HashSet<(usize, usize)>) -> usize {
    let (r0, rn) = set.iter().map(|(r, _)| *r).minmax().into_option().unwrap();
    let (c0, cn) = set.iter().map(|(_, c)| *c).minmax().into_option().unwrap();

    (r0..=(rn + 1))
        .map(|r| {
            (c0..=cn)
                .map(move |c| (r > 0 && set.contains(&(r - 1, c)), set.contains(&(r, c))))
                .collect::<Vec<_>>()
                .chunk_by(|a, b| a == b)
                .filter(|chunk| matches!(chunk, [(a, b), ..] if a != b))
                .count()
        })
        .sum::<usize>()
        + (c0..=(cn + 1))
            .map(|c| {
                (r0..=rn)
                    .map(move |r| (c > 0 && set.contains(&(r, c - 1)), set.contains(&(r, c))))
                    .collect::<Vec<_>>()
                    .chunk_by(|a, b| a == b)
                    .filter(|chunk| matches!(chunk, [(a, b), ..] if a != b))
                    .count()
            })
            .sum::<usize>()
}

fn process_part1(ns: &Vec<Vec<char>>) -> usize {
    process_internal(ns).iter().map(|(a, b, _)| a * b).sum()
}

fn process_part2(ns: &Vec<Vec<char>>) -> usize {
    process_internal(ns)
        .iter()
        .map(|(_, _, set)| set.len() * sides(set))
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
        assert_eq!(result, 1930);
    }

    #[test]
    fn test_part2() {
        let ns = parse(INPUT.lines());
        let result = process_part2(&ns);
        assert_eq!(result, 1206);
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
