use itertools::Itertools;
use num_integer::{lcm, Integer, Roots};
use regex::Regex;

type Game = ((usize, usize), (usize, usize), (usize, usize));

fn parse(input: &str) -> Vec<Game> {
    let regex = Regex::new(r"(\d+).+?(\d+)").unwrap();

    regex
        .captures_iter(input)
        .map(|cap| cap.extract())
        .map(|(_, vals)| vals.map(|v| v.parse().unwrap()))
        .map(|[a, b]| (a, b))
        .tuples()
        .collect()
}

fn hypotenuse(x: &usize, y: &usize) -> usize {
    (x.pow(2) + y.pow(2)).sqrt()
}

fn solve_for(&((ax, ay), (bx, by), (x, y)): &Game) -> Option<(usize, usize)> {
    let alcm = lcm(ax, ay);

    let (nb, remb) = ((x * alcm / ax) as i64 - (y * alcm / ay) as i64)
        .div_rem(&((bx * alcm / ax) as i64 - (by * alcm / ay) as i64));
    if remb == 0 {
        let (na, rema) = (x - nb as usize * bx).div_rem(&ax);
        if rema == 0 {
            Some((na, nb as usize))
        } else {
            None
        }
    } else {
        None
    }
}

fn process(ns: &Vec<Game>) -> usize {
    ns.iter()
        .map(|((ax, ay), (bx, by), (x, y))| {
            let a_coef = hypotenuse(ax, ay) / 3;
            let b_coef = hypotenuse(bx, by);

            if a_coef >= b_coef {
                solve_for(&((*ax, *ay), (*bx, *by), (*x, *y)))
                    .map(|(a, b)| a * 3 + b)
                    .unwrap_or_default()
            } else {
                solve_for(&((*bx, *by), (*ax, *ay), (*x, *y)))
                    .map(|(b, a)| a * 3 + b)
                    .unwrap_or_default()
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let ns = parse(INPUT);
        let result = process(&ns);
        assert_eq!(result, 480);
    }
}

fn main() {
    let input = include_str!("input.txt");
    let ns = parse(input);

    let result_part1 = process(&ns);
    println!("{}", result_part1);

    let result_part2 = process(
        &ns.iter()
            .map(|(a, b, (x, y))| (*a, *b, (x + 10000000000000, y + 10000000000000)))
            .collect(),
    );
    println!("{}", result_part2);
}
