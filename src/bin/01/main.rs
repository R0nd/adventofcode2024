use std::{iter::zip, str::Lines};

fn parse(iter: Lines) -> (Vec<i32>, Vec<i32>) {
    let mut xs = vec![];
    let mut ys = vec![];

    for line in iter {
        let mut values = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        xs.push(values.next().unwrap());
        ys.push(values.next().unwrap());
    }

    (xs, ys)
}

fn clone_sort<T>(xs: &Vec<T>) -> Vec<T>
where
    T: Clone,
    T: Ord,
{
    let mut clone = xs.clone();
    clone.sort();
    clone
}

fn process_part1(xs: &Vec<i32>, ys: &Vec<i32>) -> i32 {
    zip(clone_sort(xs), clone_sort(ys)).fold(0, |acc, (x, y)| acc + (x - y).abs())
}

fn process_part2(xs: &Vec<i32>, ys: &Vec<i32>) -> i32 {
    xs.iter().fold(0, |acc, x| {
        acc + x * ys.iter().filter(|y| *y == x).count() as i32
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let (xs, ys) = parse(INPUT.lines());
        let result = process_part1(&xs, &ys);
        assert_eq!(result, 11);
    }

    #[test]
    fn test_part2() {
        let (xs, ys) = parse(INPUT.lines());
        let result = process_part2(&xs, &ys);
        assert_eq!(result, 31);
    }
}

fn main() {
    let input = include_str!("input.txt");
    let (xs, ys) = parse(input.lines());

    let result_part1 = process_part1(&xs, &ys);
    println!("{}", result_part1);

    let result_part2 = process_part2(&xs, &ys);
    println!("{}", result_part2);
}
