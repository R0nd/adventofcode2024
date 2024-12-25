use std::{
    collections::HashSet,
    iter::{once, Chain, Once},
    ops::Sub,
    str::Lines,
};

use itertools::Itertools;

type Symbol = [char; 2];

#[derive(Clone, Copy, Eq, Hash)]
struct Connection(Symbol, Symbol);

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 || self.0 == other.1 && self.1 == other.0
    }
}

impl Sub<Symbol> for Connection {
    type Output = Symbol;

    fn sub(self, rhs: Symbol) -> Self::Output {
        match self {
            Self(a, b) if a == rhs => b,
            Self(a, b) if b == rhs => a,
            _ => panic!(),
        }
    }
}

impl PartialEq<Symbol> for Connection {
    fn eq(&self, other: &Symbol) -> bool {
        self.0 == *other || self.1 == *other
    }
}

impl IntoIterator for Connection {
    type Item = Symbol;

    type IntoIter = Chain<Once<Self::Item>, Once<Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        once(self.0).chain(once(self.1))
    }
}

fn parse(iter: Lines) -> Vec<Connection> {
    iter.map(|s| {
        if let Some((a, b)) = s.split('-').tuples().next() {
            let mut a_chars = a.chars();
            let mut b_chars = b.chars();
            Connection(
                [a_chars.next().unwrap(), a_chars.next().unwrap()],
                [b_chars.next().unwrap(), b_chars.next().unwrap()],
            )
        } else {
            panic!()
        }
    })
    .collect()
}

fn process_part1(ns: &Vec<Connection>) -> usize {
    let symbols = ns
        .iter()
        .flat_map(|n| n.into_iter())
        .collect::<HashSet<_>>();

    symbols
        .iter()
        .filter(|[a, _]| *a == 't')
        .flat_map(|sym| {
            ns.iter()
                .filter(move |c| **c == *sym)
                .combinations(2)
                .filter(|s| matches!(s[..], [a,b] if ns.contains(&Connection(*a-*sym,*b-*sym))))
                .map(|conns| match conns[..] {
                    [&a, &b] => a
                        .into_iter()
                        .chain(b.into_iter())
                        .collect::<HashSet<_>>()
                        .iter()
                        .sorted()
                        .cloned()
                        .collect::<Vec<_>>(),
                    _ => panic!(),
                })
        })
        .collect::<HashSet<_>>()
        .len()
}

fn process_part2(ns: &Vec<Connection>) -> String {
    let symbols = ns
        .iter()
        .flat_map(|n| once(n.0).chain(once(n.1)))
        .collect::<HashSet<_>>();

    (2..=13)
        .rev()
        .find_map(|n| {
            symbols.iter().find_map(|s| {
                ns.iter()
                    .filter(|n| **n == *s)
                    .map(|n| *n - *s)
                    .chain(once(*s))
                    .collect::<HashSet<_>>()
                    .iter()
                    .cloned()
                    .combinations(n)
                    .find(|g| {
                        g.iter().combinations(2).all(
                            |pair| matches!(pair[..], [a,b] if ns.contains(&Connection(*a,*b))),
                        )
                    })
            })
        })
        .unwrap()
        .iter()
        .map(|n| n.iter().collect::<String>())
        .sorted()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let ns = parse(INPUT.lines());
        let result = process_part1(&ns);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_part2() {
        let ns = parse(INPUT.lines());
        let result = process_part2(&ns);
        assert_eq!(result, "co,de,ka,ta");
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
