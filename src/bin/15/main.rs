use std::{ops::Add, str::Lines};

use itertools::Itertools;

#[derive(PartialEq, Clone, Copy, Debug)]
enum Cell {
    None,
    Box,
    Wall,
}

#[derive(PartialEq, Clone, Copy)]
enum Cell2 {
    None,
    BoxLeft,
    BoxRight,
    Wall,
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
struct Point(usize, usize);

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

fn parse(iter: Lines) -> (Vec<Vec<Cell>>, Point, Vec<Direction>) {
    let mut map = vec![];
    let mut position = None;

    let mut enum_iter = iter.enumerate();
    while let Some((row, s)) = enum_iter.next() {
        if s.is_empty() {
            break;
        }

        if let Some(col) = s.find('@') {
            position = Some(Point(row, col));
        }

        map.push(
            s.chars()
                .map(|c| match c {
                    '#' => Cell::Wall,
                    'O' => Cell::Box,
                    '.' | '@' => Cell::None,
                    _ => panic!(),
                })
                .collect::<Vec<_>>(),
        );
    }

    let moves = enum_iter
        .flat_map(|(_, s)| s.chars())
        .map(|c| match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!(),
        })
        .collect();

    (map, position.unwrap(), moves)
}

fn map2(map: &Vec<Vec<Cell>>) -> Vec<Vec<Cell2>> {
    map.iter()
        .map(|r| {
            r.iter()
                .flat_map(|c| match c {
                    Cell::None => [Cell2::None; 2],
                    Cell::Box => [Cell2::BoxLeft, Cell2::BoxRight],
                    Cell::Wall => [Cell2::Wall; 2],
                })
                .collect()
        })
        .collect()
}

fn push(map: &mut Vec<Vec<Cell2>>, from: &Point, direction: &Direction, tail: bool) -> bool {
    let nextpoint = *from + *direction;
    let nextcell = map[nextpoint.0][nextpoint.1];
    let result = match nextcell {
        Cell2::None => true,
        Cell2::Wall => false,
        Cell2::BoxLeft if *direction == Direction::Up || *direction == Direction::Down => {
            push(map, &nextpoint, direction, false)
                && (tail || push(map, &(*from + Direction::Right), direction, true))
        }
        Cell2::BoxRight if *direction == Direction::Up || *direction == Direction::Down => {
            push(map, &nextpoint, direction, false)
                && (tail || push(map, &(*from + Direction::Left), direction, true))
        }
        _ => push(map, &nextpoint, direction, false),
    };

    if result && (nextcell == Cell2::BoxLeft || nextcell == Cell2::BoxRight) {
        map[nextpoint.0][nextpoint.1] = Cell2::None;
        let pushed_into = nextpoint + *direction;
        map[pushed_into.0][pushed_into.1] = nextcell;
    }

    result
}

fn process_part1(map: &Vec<Vec<Cell>>, position: &Point, moves: &Vec<Direction>) -> usize {
    moves
        .iter()
        .fold((map.clone(), *position), |(prevmap, prevpos), direction| {
            let Point(r, c) = prevpos;
            let slice = match direction {
                Direction::Up => (0..r).rev().map(|rr| prevmap[rr][c]).collect::<Vec<_>>(),
                Direction::Down => (r + 1..prevmap.len())
                    .map(|rr| prevmap[rr][c])
                    .collect::<Vec<_>>(),
                Direction::Left => (0..c).rev().map(|cc| prevmap[r][cc]).collect::<Vec<_>>(),
                Direction::Right => (c + 1..prevmap[0].len())
                    .map(|cc| prevmap[r][cc])
                    .collect::<Vec<_>>(),
            };
            let chunks = slice.iter().chunk_by(|v| **v);
            let mut chunk_iter = chunks.into_iter();

            let chunk_a = chunk_iter.next().unwrap();
            let chunk_b = chunk_iter.next();

            match chunk_a {
                (Cell::None, _) => (prevmap, prevpos + *direction),
                (Cell::Wall, _) => (prevmap, prevpos),
                (Cell::Box, boxes) => match chunk_b {
                    None | Some((Cell::Wall, _)) => (prevmap, prevpos),
                    Some((Cell::None, _)) => {
                        let mut nextmap = prevmap.clone();
                        let nextpos = prevpos + *direction;
                        let nextbox = boxes.fold(nextpos, |p, _| p + *direction);
                        nextmap[nextpos.0][nextpos.1] = Cell::None;
                        nextmap[nextbox.0][nextbox.1] = Cell::Box;
                        (nextmap, nextpos)
                    }
                    _ => panic!(),
                },
            }
        })
        .0
        .iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .map(move |(c, cell)| if *cell == Cell::Box { r * 100 + c } else { 0 })
        })
        .sum()
}

fn process_part2(map: &Vec<Vec<Cell>>, &Point(row, col): &Point, moves: &Vec<Direction>) -> usize {
    moves
        .iter()
        .fold(
            (map2(map), Point(row, col * 2)),
            |(prevmap, prevpos), direction| {
                let mut nextmap = prevmap.clone();
                if push(&mut nextmap, &prevpos, direction, false) {
                    (nextmap, prevpos + *direction)
                } else {
                    (prevmap, prevpos)
                }
            },
        )
        .0
        .iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter().enumerate().map(move |(c, cell)| {
                if *cell == Cell2::BoxLeft {
                    r * 100 + c
                } else {
                    0
                }
            })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let (map, position, moves) = parse(INPUT.lines());
        let result = process_part1(&map, &position, &moves);
        assert_eq!(result, 10092);
    }

    #[test]
    fn test_part2() {
        let (map, position, moves) = parse(INPUT.lines());
        let result = process_part2(&map, &position, &moves);
        assert_eq!(result, 9021);
    }
}

fn main() {
    let input = include_str!("input.txt");
    let (map, position, moves) = parse(input.lines());

    let result_part1 = process_part1(&map, &position, &moves);
    println!("{}", result_part1);

    let result_part2 = process_part2(&map, &position, &moves);
    println!("{}", result_part2);
}
