use std::{
    collections::HashSet,
    hash::Hash,
    iter::{once, Cycle},
    slice::Iter,
};

use itertools::Itertools;

const DIRECTIONS: [Point; 4] = [Point(-1, 0), Point(0, 1), Point(1, 0), Point(0, -1)];

#[derive(Clone)]
struct Guard<'a> {
    position: Point,
    direction: Cycle<Iter<'a, Point>>,
}

impl PartialEq for Guard<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
            && self.direction.clone().peekable().peek() == other.direction.clone().peekable().peek()
    }
}

impl Eq for Guard<'_> {}

impl Hash for Guard<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.position.hash(state);
        self.direction.clone().peekable().peek().hash(state);
    }
}

#[derive(Copy, Clone)]
struct Point(i32, i32);

impl std::ops::Add for Point {
    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }

    type Output = Point;
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

fn parse(input: &str) -> (Guard, Vec<Point>, usize) {
    let obstacles = input
        .lines()
        .enumerate()
        .flat_map(|(row, s)| {
            s.match_indices('#')
                .map(move |(col, _)| Point(row as i32, col as i32))
        })
        .collect();

    let position = input
        .lines()
        .enumerate()
        .find_map(|(row, s)| s.find('^').map(|col| Point(row as i32, col as i32)))
        .unwrap();
    let direction = DIRECTIONS.iter().cycle();
    let guard = Guard {
        position,
        direction,
    };

    (guard, obstacles, input.lines().count())
}

fn path<'a>(
    mut guard: Guard<'a>,
    obstacles: &'a Vec<Point>,
    size: usize,
) -> impl Iterator<Item = Guard<'a>> + 'a {
    (0..)
        .map(move |_| {
            let next_position =
                guard.position + **guard.direction.clone().peekable().peek().unwrap();
            match next_position {
                Point(x, _) if x < 0 || x >= size as i32 => None,
                Point(_, y) if y < 0 || y >= size as i32 => None,
                p if obstacles.contains(&p) => {
                    guard.direction.next();
                    Some(guard.clone())
                }
                _ => {
                    guard.position = next_position;
                    Some(guard.clone())
                }
            }
        })
        .take_while(Option::is_some)
        .flatten()
}

fn process_part1(guard: Guard, obstacles: &Vec<Point>, size: usize) -> usize {
    let start_position = guard.position;
    path(guard, obstacles, size)
        .map(|g| g.position)
        .chain(once(start_position))
        .collect::<HashSet<_>>()
        .len()
}

fn process_part2(guard: Guard, obstacles: &Vec<Point>, size: usize) -> usize {
    let original_path = path(guard.clone(), obstacles, size)
        .map(|g| g.position)
        .collect::<HashSet<_>>();

    original_path
        .iter()
        .filter(|path_position| {
            let new_obstacles = obstacles
                .iter()
                .cloned()
                .chain(once(**path_position))
                .collect::<Vec<_>>();

            let is_loop = !path(guard.clone(), &new_obstacles, size).all_unique();

            is_loop
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let (guard, obstacles, size) = parse(INPUT);
        let result = process_part1(guard, &obstacles, size);
        assert_eq!(result, 41);
    }

    #[test]
    fn test_part2() {
        let (guard, obstacles, size) = parse(INPUT);
        let result = process_part2(guard, &obstacles, size);
        assert_eq!(result, 6);
    }
}

fn main() {
    let input = include_str!("input.txt");
    let (guard, obstacles, size) = parse(input);

    let result_part1 = process_part1(guard.clone(), &obstacles, size);
    println!("{}", result_part1);

    let result_part2 = process_part2(guard, &obstacles, size);
    println!("{}", result_part2);
}
