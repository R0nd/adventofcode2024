use std::{collections::HashSet, str::Lines};

fn parse(iter: Lines) -> Vec<Vec<u32>> {
    iter.map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn step((x, y): (usize, usize)) -> Vec<(usize, usize)> {
    vec![
        if x > 0 { Some((x - 1, y)) } else { None },
        Some((x + 1, y)),
        if y > 0 { Some((x, y - 1)) } else { None },
        Some((x, y + 1)),
    ]
    .iter()
    .filter_map(|v| *v)
    .collect()
}

fn score(start: (usize, usize), ns: &Vec<Vec<u32>>) -> usize {
    let mut edge = vec![start];
    let mut trail: HashSet<(usize, usize)> = HashSet::new();

    while !edge.is_empty() {
        let next_edge = edge
            .iter()
            .cloned()
            .flat_map(|(erow, ecol)| {
                step((erow, ecol))
                    .iter()
                    .cloned()
                    .filter(|&(row, col)| row < ns.len() && col < ns[0].len())
                    .filter(|&(row, col)| (ns[row][col] as i32 - ns[erow][ecol] as i32) == 1)
                    .collect::<Vec<_>>()
            })
            .filter(|&e| trail.insert(e))
            .collect::<Vec<_>>();

        edge = next_edge;
    }

    trail
        .iter()
        .filter(|&&(row, col)| ns[row][col] == 9)
        .count()
}

fn rating(start: (usize, usize), ns: &Vec<Vec<u32>>) -> usize {
    let height = ns[start.0][start.1];
    if height == 9 {
        1
    } else {
        step(start)
            .iter()
            .filter(|&&(row, col)| row < ns.len() && col < ns[0].len())
            .filter(|&&(row, col)| (ns[row][col] as i32 - height as i32) == 1)
            .map(|&next| rating(next, ns))
            .sum()
    }
}

fn starts(ns: &Vec<Vec<u32>>) -> impl Iterator<Item = (usize, usize)> + '_ {
    (0..ns.len()).flat_map(move |row| {
        (0..ns[0].len()).filter_map(move |col| {
            if ns[row][col] == 0 {
                Some((row, col))
            } else {
                None
            }
        })
    })
}

fn process_part1(ns: &Vec<Vec<u32>>) -> usize {
    starts(ns).map(|start| score(start, ns)).sum()
}

fn process_part2(ns: &Vec<Vec<u32>>) -> usize {
    starts(ns).map(|start| rating(start, ns)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let ns = parse(INPUT.lines());
        let result = process_part1(&ns);
        assert_eq!(result, 36);
    }

    #[test]
    fn test_part2() {
        let ns = parse(INPUT.lines());
        let result = process_part2(&ns);
        assert_eq!(result, 81);
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
