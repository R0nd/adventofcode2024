use std::{collections::HashSet, str::Lines};

use itertools::Itertools;

fn parse(iter: Lines) -> Vec<(usize, usize)> {
    iter.map(|s| {
        s.split(",")
            .map(|v| v.parse().unwrap())
            .tuples()
            .next()
            .unwrap()
    })
    .collect()
}

fn neighbors(start: (usize, usize), grid_size: usize) -> Vec<(usize, usize)> {
    let mut result = vec![];

    if start.0 > 0 {
        result.push((start.0 - 1, start.1));
    }
    if start.1 > 0 {
        result.push((start.0, start.1 - 1));
    }
    if start.0 < grid_size - 1 {
        result.push((start.0 + 1, start.1));
    }
    if start.1 < grid_size - 1 {
        result.push((start.0, start.1 + 1));
    }

    result
}

fn process_part1(blocked: &Vec<(usize, usize)>, grid_size: usize) -> Option<usize> {
    let mut map = vec![vec![None; grid_size]; grid_size];
    map[0][0] = Some(0);

    let mut edge = HashSet::new();
    edge.insert((0, 0));
    let mut score = 0;
    while map[grid_size - 1][grid_size - 1].is_none() && !edge.is_empty() {
        score += 1;
        edge = edge
            .iter()
            .flat_map(|&e| neighbors(e, grid_size))
            .filter(|e| !blocked.contains(e))
            .filter(|&(x, y)| map[x][y].is_none())
            .collect();
        edge.iter().for_each(|&(x, y)| map[x][y] = Some(score));
    }

    map[grid_size - 1][grid_size - 1]
}

fn process_part2(ns: &Vec<(usize, usize)>, grid_size: usize) -> (usize, usize) {
    let i = (1..ns.len())
        .rev()
        .find(|&count| process_part1(&ns[0..=count].to_vec(), grid_size).is_some())
        .unwrap()
        + 1;

    ns[i]
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let ns = parse(INPUT.lines());
        let result = process_part1(&ns[0..12].to_vec(), 7);
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part2() {
        let ns = parse(INPUT.lines());
        let result = process_part2(&ns, 7);
        assert_eq!(result, (6, 1));
    }
}

fn main() {
    let input = include_str!("input.txt");
    let ns = parse(input.lines());

    let result_part1 = process_part1(&ns[0..1024].to_vec(), 71);
    println!("{}", result_part1.unwrap());

    let result_part2 = process_part2(&ns, 71);
    println!("{},{}", result_part2.0, result_part2.1);
}
