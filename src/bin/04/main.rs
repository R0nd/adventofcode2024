use std::str::Lines;

fn parse(iter: Lines) -> Vec<Vec<char>> {
    iter.map(|line| line.chars().collect()).collect()
}

fn windows2<const SIZE: usize>(vec2: &Vec<Vec<char>>) -> Vec<[[char; SIZE]; SIZE]> {
    vec2.windows(SIZE)
        .flat_map(|row_window| {
            (0..=row_window[0].len() - SIZE).map(|i| {
                row_window
                    .into_iter()
                    .map(|row| {
                        row.iter()
                            .skip(i)
                            .take(SIZE)
                            .copied()
                            .collect::<Vec<char>>()
                            .try_into()
                            .unwrap()
                    })
                    .collect::<Vec<[char; SIZE]>>()
                    .try_into()
                    .unwrap()
            })
        })
        .collect()
}

fn process_part1(nss: &Vec<Vec<char>>) -> i32 {
    let horizontal_lines = nss.iter().map(|row| row.iter().collect::<String>());
    let vertical_lines =
        (0..nss[0].len()).map(|i| nss.iter().map(|row| row[i]).collect::<String>());

    let lines = horizontal_lines.chain(vertical_lines);
    let across_count = lines
        .map(|s| s.matches("XMAS").chain(s.matches("SAMX")).count() as i32)
        .sum::<i32>();

    let ws = windows2::<4>(nss);
    let mut diagonal_count = 0;
    for w in ws {
        diagonal_count += match w {
            [['X', _, _, _], [_, 'M', _, _], [_, _, 'A', _], [_, _, _, 'S']]
            | [['S', _, _, _], [_, 'A', _, _], [_, _, 'M', _], [_, _, _, 'X']] => 1,
            _ => 0,
        };
        diagonal_count += match w {
            [[_, _, _, 'X'], [_, _, 'M', _], [_, 'A', _, _], ['S', _, _, _]]
            | [[_, _, _, 'S'], [_, _, 'A', _], [_, 'M', _, _], ['X', _, _, _]] => 1,
            _ => 0,
        };
    }

    across_count + diagonal_count
}

fn process_part2(nss: &Vec<Vec<char>>) -> i32 {
    let ws = windows2::<3>(nss);
    let mut count = 0;
    for w in ws {
        count += match w {
            [['M', _, 'M'], [_, 'A', _], ['S', _, 'S']]
            | [['S', _, 'M'], [_, 'A', _], ['S', _, 'M']]
            | [['S', _, 'S'], [_, 'A', _], ['M', _, 'M']]
            | [['M', _, 'S'], [_, 'A', _], ['M', _, 'S']] => 1,
            _ => 0,
        };
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let ns = parse(INPUT.lines());
        let result = process_part1(&ns);
        assert_eq!(result, 18);
    }

    #[test]
    fn test_part2() {
        let ns = parse(INPUT.lines());
        let result = process_part2(&ns);
        assert_eq!(result, 9);
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
