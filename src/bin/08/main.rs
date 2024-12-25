use std::{
    collections::HashSet,
    hash::Hash,
    ops::{Add, Mul, Sub},
};

use itertools::Itertools;
use num_integer::gcd;

#[derive(Clone, Copy, Debug)]
struct Point(i32, i32);

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul<i32> for Point {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Eq for Point {}

impl Hash for Point {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
        self.1.hash(state);
    }
}

fn parse(input: &str) -> (i32, Vec<Vec<Point>>) {
    let size = input.lines().count() as i32;

    let antennae = input
        .lines()
        .enumerate()
        .flat_map(|(row, s)| {
            s.chars().enumerate().filter_map(move |(col, c)| match c {
                'A'..='Z' | 'a'..='z' | '0'..='9' => Some((c, Point(row as i32, col as i32))),
                _ => None,
            })
        })
        .into_group_map()
        .values()
        .cloned()
        .collect::<Vec<_>>();

    (size, antennae)
}

fn step(delta: &Point) -> Point {
    let gcd = gcd(delta.0, delta.1);
    Point(delta.0 / gcd, delta.1 / gcd)
}

fn in_bounds(size: i32) -> impl Fn(&Point) -> bool {
    move |p| p.0 >= 0 && p.0 < size && p.1 >= 0 && p.1 < size
}

fn process_part1(size: i32, antennae: &Vec<Vec<Point>>) -> usize {
    antennae
        .iter()
        .flat_map(|ants| ants.iter().combinations(2))
        .flat_map(|ants| {
            let [&a, &b] = ants[..] else { panic!() };

            vec![a - (b - a), b + (b - a)]
        })
        .filter(in_bounds(size))
        .collect::<HashSet<_>>()
        .len()
}

fn process_part2(size: i32, antennae: &Vec<Vec<Point>>) -> usize {
    antennae
        .iter()
        .flat_map(|ants| ants.iter().combinations(2))
        .flat_map(|ants| {
            let [&a, &b] = ants[..] else { panic!() };

            let step_forward = step(&(b - a));
            let forward_points = (1..)
                .map(move |n| a + (step_forward * n))
                .take_while(in_bounds(size));

            let step_backward = step(&(a - b));
            let backward_points = (1..)
                .map(move |n| b + (step_backward * n))
                .take_while(in_bounds(size));

            forward_points.chain(backward_points)
        })
        .collect::<HashSet<_>>()
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let (size, antennae) = parse(INPUT);
        let result = process_part1(size, &antennae);
        assert_eq!(result, 14);
    }

    #[test]
    fn test_part2() {
        let (size, antennae) = parse(INPUT);
        let result = process_part2(size, &antennae);
        assert_eq!(result, 34);
    }
}

fn main() {
    let input = include_str!("input.txt");
    let (size, antennae) = parse(input);

    let result_part1 = process_part1(size, &antennae);
    println!("{}", result_part1);

    let result_part2 = process_part2(size, &antennae);
    println!("{}", result_part2);
}
