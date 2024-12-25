fn parse(input: &str) -> Vec<Option<usize>> {
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .enumerate()
        .flat_map(|(i, c)| {
            if i % 2 == 0 {
                vec![Some(i / 2); c]
            } else {
                vec![None; c]
            }
        })
        .collect()
}

fn process_part1(blocks: &Vec<Option<usize>>) -> usize {
    let mut revblocks = blocks.iter().rev().filter_map(|b| *b);

    (0..blocks.iter().filter(|b| b.is_some()).count())
        .map(|i| blocks[i].or_else(|| revblocks.next()).unwrap() * i)
        .sum()
}

fn process_part2(blocks: &Vec<Option<usize>>) -> usize {
    let chunks = blocks.chunk_by(|a, b| a == b);
    let mut spans = chunks.map(|c| (c[0], c.len())).collect::<Vec<_>>();
    let revspans = spans
        .iter()
        .cloned()
        .rev()
        .filter_map(|(key, size)| key.and_then(|v| Some((v, size))))
        .collect::<Vec<_>>();
    for (key, size) in revspans {
        let index = spans
            .iter()
            .position(|(s, _)| s.is_some_and(|v| v == key))
            .unwrap();
        spans
            .iter()
            .cloned()
            .enumerate()
            .find(|(i, (s, n))| *i < index && s.is_none() && *n >= size)
            .inspect(|(i, (_, n))| {
                spans.remove(index);
                spans.insert(index, (None, size));
                spans.remove(*i);
                spans.insert(*i, (Some(key), size));
                if *n > size {
                    spans.insert(*i + 1, (None, n - size));
                }
            });
    }
    spans
        .iter()
        .flat_map(|(key, size)| vec![key; *size])
        .enumerate()
        .map(|(i, k)| k.unwrap_or_default() * i)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let ns = parse(INPUT);
        let result = process_part1(&ns);
        assert_eq!(result, 1928);
    }

    #[test]
    fn test_part2() {
        let ns = parse(INPUT);
        let result = process_part2(&ns);
        assert_eq!(result, 2858);
    }
}

fn main() {
    let input = include_str!("input.txt");
    let ns = parse(input);

    let result_part1 = process_part1(&ns);
    println!("{}", result_part1);

    let result_part2 = process_part2(&ns);
    println!("{}", result_part2);
}
