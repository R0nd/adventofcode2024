use std::collections::HashMap;

use num_integer::Integer;

fn change(n: u64, depth: usize, cache: &mut HashMap<(u64, usize), usize>) -> usize {
    if depth == 0 {
        1
    } else {
        if depth >= 25 {
            let cached = cache.get(&(n, depth));
            if cached.is_some() {
                return *cached.unwrap();
            }
        }
        let r = if n == 0 {
            change(1, depth - 1, cache)
        } else {
            let digits = ((n as f64).log10() + 1 as f64).floor() as u32;
            if digits % 2 == 0 {
                let (a, b) = n.div_rem(&10u64.pow(digits / 2));
                change(a, depth - 1, cache) + change(b, depth - 1, cache)
            } else {
                change(n * 2024, depth - 1, cache)
            }
        };
        if depth >= 25 {
            cache.insert((n, depth), r);
        }
        r
    }
}

fn parse(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn process(ns: &Vec<u64>, blinks: usize) -> usize {
    let mut cache = HashMap::new();
    ns.iter().map(|n| change(*n, blinks, &mut cache)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let ns = parse(INPUT);
        let result = process(&ns, 25);
        assert_eq!(result, 55312);
    }
}

fn main() {
    let input = include_str!("input.txt");
    let ns = parse(input);

    let result_part1 = process(&ns, 25);
    println!("{}", result_part1);

    let result_part2 = process(&ns, 75);
    println!("{}", result_part2);
}
