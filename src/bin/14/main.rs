use std::{
    collections::BTreeMap,
    ops::{Add, ControlFlow, Rem},
    str::Lines,
};

use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point(i32, i32);

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Rem for Point {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self(self.0 % rhs.0, self.1 % rhs.1)
    }
}

fn parse(iter: Lines) -> Vec<(Point, Point)> {
    let regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    iter.flat_map(|s| {
        regex
            .captures(s)
            .unwrap()
            .iter()
            .skip(1)
            .map(|c| c.unwrap().as_str().parse().unwrap())
            .tuples()
            .collect::<Vec<_>>()
    })
    .map(|(px, py, vx, vy)| (Point(px, py), Point(vx, vy)))
    .collect()
}

fn process_part1(ns: &Vec<(Point, Point)>, size: Point) -> usize {
    (0..100)
        .fold(ns.iter().copied().collect::<Vec<_>>(), |acc, _| {
            acc.iter()
                .map(|&(p, v)| ((p + v + size) % size, v))
                .collect()
        })
        .iter()
        .fold(
            BTreeMap::<(bool, bool), usize>::new(),
            |mut acc, &(Point(x, y), _)| {
                if x == size.0 / 2 || y == size.1 / 2 {
                    acc
                } else {
                    acc.entry((x > size.0 / 2, y > size.1 / 2))
                        .and_modify(|n| *n += 1)
                        .or_insert(1);
                    acc
                }
            },
        )
        .iter()
        .map(|(_, n)| n)
        .product()
}

fn process_part2(ns: &Vec<(Point, Point)>, size: Point) -> Option<i32> {
    let mut result = None;

    (1..).try_fold(ns.iter().copied().collect::<Vec<_>>(), |acc, i| {
        let r: Vec<_> = acc
            .iter()
            .map(|&(p, v)| ((p + v + size) % size, v))
            .collect();

        if (0..size.1).any(|row| {
            (0..size.0).collect::<Vec<_>>().windows(30).any(|w| {
                w.iter()
                    .all(|col| r.iter().any(|(p, _)| p == &Point(*col, row)))
            })
        }) {
            (0..size.1)
                .inspect(|row| {
                    println!(
                        "{}",
                        (0..size.0)
                            .map(|col| {
                                if r.iter().any(|(p, _)| *p == Point(col, *row)) {
                                    '#'
                                } else {
                                    '.'
                                }
                            })
                            .collect::<String>()
                    );
                })
                .for_each(drop);

            result = Some(i);
            return ControlFlow::Break(r);
        }

        ControlFlow::Continue(r)
    });

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let ns = parse(INPUT.lines());
        let result = process_part1(&ns, Point(11, 7));
        assert_eq!(result, 12);
    }
}

fn main() {
    let input = include_str!("input.txt");
    let ns = parse(input.lines());

    let result_part1 = process_part1(&ns, Point(101, 103));
    println!("{}", result_part1);

    let result_part2 = process_part2(&ns, Point(101, 103));
    println!("{}", result_part2.unwrap());
}
