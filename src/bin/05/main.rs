use std::cmp::Ordering;

use regex::Regex;

fn parse(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let sections = input.split("\n\n").collect::<Vec<&str>>();
    let [rules_input, pages_input] = sections.as_slice() else {
        panic!()
    };

    let rule_regex = Regex::new(r"(\d+)\|(\d+)").unwrap();
    let rules = rule_regex
        .captures_iter(rules_input)
        .map(|cap| cap.extract())
        .map(|(_, vals)| vals.map(|v| v.parse().unwrap()))
        .map(|[a, b]| (a, b))
        .collect();

    let updates = pages_input
        .lines()
        .map(|s| s.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();

    (rules, updates)
}

fn rule_sort(rules: &Vec<(i32, i32)>) -> impl FnMut(&i32, &i32) -> Ordering + '_ {
    |a, b| {
        if rules.contains(&(*a, *b)) {
            Ordering::Less
        } else if rules.contains(&(*b, *a)) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

fn middle<T>(ns: &Vec<T>) -> T
where
    T: Copy,
{
    ns.get(ns.len() / 2).unwrap().to_owned()
}

fn process(rules: &Vec<(i32, i32)>, updates: &Vec<Vec<i32>>) -> (i32, i32) {
    let (correct, incorrect): (Vec<Option<i32>>, Vec<Option<i32>>) = updates
        .iter()
        .map(|u: &Vec<i32>| {
            let mut update = u.to_owned();

            update.sort_by(rule_sort(rules));

            if u == &update {
                (Some(middle(u)), None)
            } else {
                (None, Some(middle(&update)))
            }
        })
        .unzip();

    (
        correct.iter().flatten().sum(),
        incorrect.iter().flatten().sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let (rules, updates) = parse(INPUT);
        let (result, _) = process(&rules, &updates);
        assert_eq!(result, 143);
    }

    #[test]
    fn test_part2() {
        let (rules, updates) = parse(INPUT);
        let (_, result) = process(&rules, &updates);
        assert_eq!(result, 123);
    }
}

fn main() {
    let input = include_str!("input.txt");
    let (rules, updates) = parse(input);

    let (result_part1, result_part2) = process(&rules, &updates);
    println!("{}", result_part1);
    println!("{}", result_part2);
}
