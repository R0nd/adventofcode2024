use std::{cmp::min, collections::HashMap, hash::Hash, ops::Add, str::Lines};

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq)]
enum Cell {
    Space,
    Wall,
    End,
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Point(usize, usize);

impl Hash for Point {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
        self.1.hash(state);
    }
}

impl Add<Direction> for Point {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::Up => Self(self.0 - 1, self.1),
            Direction::Down => Self(self.0 + 1, self.1),
            Direction::Left => Self(self.0, self.1 - 1),
            Direction::Right => Self(self.0, self.1 + 1),
        }
    }
}

fn parse(iter: Lines) -> (Vec<Vec<Cell>>, Point) {
    let mut start = None;
    let map = iter
        .enumerate()
        .map(|(r, s)| {
            s.chars()
                .enumerate()
                .map(|(c, v)| match v {
                    '.' => Cell::Space,
                    '#' => Cell::Wall,
                    'E' => Cell::End,
                    'S' => {
                        start = Some(Point(r, c));
                        Cell::Space
                    }
                    _ => panic!(),
                })
                .collect()
        })
        .collect();

    (map, start.unwrap())
}

fn cw(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::Right => Direction::Down,
    }
}

fn ccw(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Left,
        Direction::Down => Direction::Right,
        Direction::Left => Direction::Down,
        Direction::Right => Direction::Up,
    }
}

fn process(
    map: &Vec<Vec<Cell>>,
    start: &Point,
    state: &mut HashMap<Point, usize>,
    best: &mut HashMap<Point, usize>,
    direction: &Direction,
) -> Option<usize> {
    let current = state[start];
    if map[start.0][start.1] == Cell::End {
        best.entry(*start)
            .and_modify(|b| *b = min(*b, current))
            .or_insert(current);
        Some(current)
    } else {
        [
            (*direction, current + 1),
            (cw(direction), current + 1001),
            (ccw(direction), current + 1001),
        ]
        .iter()
        .filter_map(|(direction, nextscore)| {
            let nextpoint = *start + *direction;
            if map[nextpoint.0][nextpoint.1] == Cell::Wall {
                None
            } else if matches!(state.get(&nextpoint), Some(n) if *n < *nextscore-1000) {
                None
            } else {
                state.insert(nextpoint, *nextscore);
                process(map, &nextpoint, state, best, direction).inspect(|r| {
                    best.entry(*start)
                        .and_modify(|b| *b = min(*b, *r))
                        .or_insert(*r);
                })
            }
        })
        .min()
    }
}

fn process_part1(map: &Vec<Vec<Cell>>, start: &Point) -> usize {
    process(
        map,
        start,
        &mut HashMap::from([(*start, 0)]),
        &mut HashMap::new(),
        &Direction::Right,
    )
    .unwrap()
}

fn process_part2(map: &Vec<Vec<Cell>>, start: &Point) -> usize {
    let mut best = HashMap::new();
    let result = process(
        map,
        start,
        &mut HashMap::from([(*start, 0)]),
        &mut best,
        &Direction::Right,
    )
    .unwrap();
    best.iter().filter(|(_, v)| **v == result).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let (map, start) = parse(INPUT.lines());
        let result = process_part1(&map, &start);
        assert_eq!(result, 7036);
    }

    #[test]
    fn test_part2() {
        let (map, start) = parse(INPUT.lines());
        let result = process_part2(&map, &start);
        assert_eq!(result, 45);
    }
}

fn main() {
    let input = include_str!("input.txt");
    let (map, start) = parse(input.lines());

    let result_part1 = process_part1(&map, &start);
    println!("{}", result_part1);

    let result_part2 = process_part2(&map, &start);
    println!("{}", result_part2);
}
