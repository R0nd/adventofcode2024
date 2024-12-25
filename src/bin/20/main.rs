use std::{collections::HashSet, str::Lines};

fn parse(iter: Lines) -> (Vec<Vec<bool>>, (usize, usize), (usize, usize)) {
    let mut start = None;
    let mut end = None;

    let map = iter
        .enumerate()
        .map(|(r, row)| {
            row.chars()
                .enumerate()
                .map(|(c, v)| match v {
                    '#' => false,
                    '.' => true,
                    'S' => {
                        start = Some((r, c));
                        true
                    }
                    'E' => {
                        end = Some((r, c));
                        true
                    }
                    _ => panic!(),
                })
                .collect()
        })
        .collect();

    (map, start.unwrap(), end.unwrap())
}

fn heatmap(
    map: &Vec<Vec<bool>>,
    start: &(usize, usize),
    buf: &mut Option<Vec<Vec<Option<usize>>>>,
) -> Vec<Vec<Option<usize>>> {
    let mut buffer = buf.clone().unwrap_or_else(|| {
        let mut default = vec![vec![None; map[0].len()]; map.len()];
        default[start.0][start.1] = Some(0);
        default
    });

    let neighbors = neighbors(start)
        .iter()
        .cloned()
        .filter(|&(r, c)| *map.get(r).map(|row| row.get(c)).flatten().unwrap_or(&false))
        .collect::<Vec<_>>();

    for n in neighbors {
        if buffer[n.0][n.1].is_none() {
            buffer[n.0][n.1] = Some(buffer[start.0][start.1].unwrap() + 1);
            buffer = heatmap(map, &n, &mut Some(buffer));
        }
    }

    buffer
}

fn zip_map(
    a: &Vec<Vec<Option<usize>>>,
    b: &Vec<Vec<Option<usize>>>,
) -> Vec<Vec<Option<(usize, usize)>>> {
    a.iter()
        .zip(b.iter())
        .map(|(a_row, b_row)| {
            a_row
                .iter()
                .zip(b_row.iter())
                .map(|(a, b)| a.and_then(|a| Some((a, b.unwrap()))))
                .collect()
        })
        .collect()
}

fn neighbors(&(r, c): &(usize, usize)) -> Vec<(usize, usize)> {
    [
        r.checked_sub(1).map(|r| (r, c)),
        c.checked_sub(1).map(|c| (r, c)),
        Some((r + 1, c)),
        Some((r, c + 1)),
    ]
    .iter()
    .filter_map(|n| *n)
    .collect()
}

fn neighbors_deep(&(r, c): &(usize, usize), depth: &usize) -> Vec<(usize, usize)> {
    (0..=*depth)
        .flat_map(|dr| {
            (0..=(*depth - dr)).flat_map(move |dc| {
                if dr == 0 && dc == 0 {
                    HashSet::new()
                } else {
                    [
                        r.checked_sub(dr)
                            .and_then(|r| c.checked_sub(dc).map(|c| (r, c))),
                        r.checked_sub(dr).and_then(|r| Some((r, c + dc))),
                        c.checked_sub(dc).and_then(|c| Some((r + dr, c))),
                        Some((r + dr, c + dc)),
                    ]
                    .iter()
                    .copied()
                    .flatten()
                    .filter(|p| p != &(r, c))
                    .collect::<HashSet<_>>()
                }
            })
        })
        .collect()
}

fn shortcuts(map: &Vec<Vec<Option<(usize, usize)>>>, depth: &usize) -> Vec<usize> {
    map.iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(c, v)| {
                    v.and_then(|(start_distance, end_distance)| {
                        Some(
                            neighbors_deep(&(r, c), depth)
                                .iter()
                                .filter_map(|&(nr, nc)| {
                                    map.get(nr)
                                        .and_then(|row| row.get(nc).cloned())
                                        .flatten()
                                        .map(|d| (r.abs_diff(nr) + c.abs_diff(nc), d))
                                })
                                .filter_map(|(jump, (other_start_distance, other_end_distance))| {
                                    if other_start_distance > start_distance + jump
                                        && other_end_distance < end_distance + jump
                                    {
                                        Some(end_distance - jump - other_end_distance)
                                    } else {
                                        None
                                    }
                                })
                                .collect::<Vec<_>>(),
                        )
                    })
                })
                .flatten()
        })
        .collect()
}

fn find_shortcuts(
    map: &Vec<Vec<bool>>,
    start: &(usize, usize),
    end: &(usize, usize),
    depth: &usize,
) -> Vec<usize> {
    let start_map = heatmap(&map, &start, &mut None);
    let end_map = heatmap(&map, &end, &mut None);
    let zipped_map = zip_map(&start_map, &end_map);
    shortcuts(&zipped_map, depth)
}

fn process_part1(map: &Vec<Vec<bool>>, start: &(usize, usize), end: &(usize, usize)) -> usize {
    find_shortcuts(map, start, end, &2)
        .iter()
        .filter(|d| **d >= 100)
        .count()
}

fn process_part2(map: &Vec<Vec<bool>>, start: &(usize, usize), end: &(usize, usize)) -> usize {
    find_shortcuts(map, start, end, &20)
        .iter()
        .filter(|d| **d >= 100)
        .count()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    static INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        let (map, start, end) = parse(INPUT.lines());
        let shortcuts = find_shortcuts(&map, &start, &end, &2);

        let groups = shortcuts.iter().fold(HashMap::new(), |mut acc, &d| {
            acc.entry(d).and_modify(|a| *a += 1).or_insert(1);
            acc
        });
        assert_eq!(
            groups,
            [
                (2, 14),
                (4, 14),
                (6, 2),
                (8, 4),
                (10, 2),
                (12, 3),
                (20, 1),
                (36, 1),
                (38, 1),
                (40, 1),
                (64, 1)
            ]
            .iter()
            .cloned()
            .collect()
        );
    }

    #[test]
    fn test_part2() {
        let (map, start, end) = parse(INPUT.lines());
        let shortcuts = find_shortcuts(&map, &start, &end, &20);
        let groups = shortcuts
            .iter()
            .fold(HashMap::new(), |mut acc, &d| {
                acc.entry(d).and_modify(|a| *a += 1).or_insert(1);
                acc
            })
            .iter()
            .filter(|(n, _)| **n >= 50)
            .map(|(a, b)| (*a, *b))
            .collect::<HashSet<_>>();
        assert_eq!(
            groups,
            [
                (50, 32),
                (52, 31),
                (54, 29),
                (56, 39),
                (58, 25),
                (60, 23),
                (62, 20),
                (64, 19),
                (66, 12),
                (68, 14),
                (70, 12),
                (72, 22),
                (74, 4),
                (76, 3),
            ]
            .iter()
            .cloned()
            .collect::<HashSet<_>>()
        );
    }
}

fn main() {
    let input = include_str!("input.txt");
    let (map, start, end) = parse(input.lines());

    let result_part1 = process_part1(&map, &start, &end);
    println!("{}", result_part1);

    let result_part2 = process_part2(&map, &start, &end);
    println!("{}", result_part2);
}
