use std::str::Lines;

fn parse(iter: Lines) -> Vec<Vec<i32>> {
    iter.map(|line| {
        line.split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect()
    })
    .collect()
}

fn skip_index(ns: &Vec<i32>, index: usize) -> Vec<i32> {
    ns[..index]
        .iter()
        .chain(ns[index + 1..].iter())
        .cloned()
        .collect()
}

fn is_safe(report: &Vec<i32>, dampen: bool) -> bool {
    let mut last_delta: i32 = 0;
    for (i, (a, b)) in report.iter().zip(report.iter().skip(1)).enumerate() {
        let delta = a - b;

        let is_good_delta = (last_delta == 0 || delta.signum() == last_delta.signum())
            && (delta.abs() >= 1 && delta.abs() <= 3);

        if !is_good_delta {
            return if dampen {
                (0..i + 2).any(|i| is_safe(&skip_index(report, i), false))
            } else {
                false
            };
        }

        last_delta = delta;
    }
    true
}

fn process_part1(reports: &Vec<Vec<i32>>) -> i32 {
    reports.iter().filter(|r| is_safe(r, false)).count() as i32
}

fn process_part2(reports: &Vec<Vec<i32>>) -> i32 {
    reports.iter().filter(|r| is_safe(r, true)).count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let ns = parse(INPUT.lines());
        let result = process_part1(&ns);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part2() {
        let ns = parse(INPUT.lines());
        let result = process_part2(&ns);
        assert_eq!(result, 4);
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
