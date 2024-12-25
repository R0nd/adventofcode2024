use std::{str::Lines, vec};

use itertools::Itertools;

#[derive(Clone, Debug)]
struct State {
    a: usize,
    b: usize,
    c: usize,
    instruction_ptr: usize,
    output: Vec<usize>,
}

type Instruction = fn(State, usize) -> State;

fn combo(state: &State, operand: usize) -> usize {
    match operand {
        0..=3 => operand,
        4 => state.a,
        5 => state.b,
        6 => state.c,
        _ => panic!(),
    }
}

const INSTRUCTIONS: [Instruction; 8] = [
    |state, operand| {
        let a = state.a >> combo(&state, operand);
        State { a, ..state }
    },
    |state, operand| {
        let b = state.b ^ operand;
        State { b, ..state }
    },
    |state, operand| {
        let b = combo(&state, operand) % 8;
        State { b, ..state }
    },
    |state, operand| {
        if state.a != 0 {
            let instruction_ptr = operand;
            State {
                instruction_ptr,
                ..state
            }
        } else {
            state
        }
    },
    |state, _| {
        let b = state.b ^ state.c;
        State { b, ..state }
    },
    |state, operand| {
        let out = combo(&state, operand) % 8;
        let mut output = state.output;
        output.push(out);
        State { output, ..state }
    },
    |state, operand| {
        let b = state.a >> combo(&state, operand);
        State { b, ..state }
    },
    |state, operand| {
        let c = state.a >> combo(&state, operand);
        State { c, ..state }
    },
];

fn parse(iter: Lines) -> (State, Vec<usize>) {
    let (a, b, c) = iter
        .clone()
        .take(3)
        .map(|s| s.split(": ").last().unwrap().parse::<usize>().unwrap())
        .tuples()
        .next()
        .unwrap();

    let program = iter
        .skip(4)
        .next()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    (
        State {
            a,
            b,
            c,
            instruction_ptr: 0,
            output: vec![],
        },
        program,
    )
}

fn process_part1(state: State, program: &Vec<usize>) -> Vec<usize> {
    let mut current = state;
    while let Some((&instruction, &operand)) =
        program.iter().skip(current.instruction_ptr).tuples().next()
    {
        let prev_ptr = current.instruction_ptr;
        current = INSTRUCTIONS[instruction](current, operand);
        if current.instruction_ptr == prev_ptr {
            current.instruction_ptr += 2;
        }
    }

    current.output
}

fn process_part2(state: State, program: &Vec<usize>, prefix: usize) -> Vec<usize> {
    if prefix >= 1 << (3 * (program.len() - 1)) {
        vec![prefix]
    } else {
        (0b000..=0b111)
            .map(|t| t | (prefix << 3))
            .filter(|&a| a > 0)
            .filter(|&a| {
                program.ends_with(&process_part1(
                    State {
                        a,
                        output: vec![],
                        ..state
                    },
                    program,
                ))
            })
            .flat_map(|next_pref| process_part2(state.clone(), program, next_pref))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("test_input.txt");
    static INPUT2: &str = include_str!("test_input2.txt");

    #[test]
    fn test_part1() {
        let (state, program) = parse(INPUT.lines());
        let result = process_part1(state, &program);
        assert_eq!(result, [4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);
    }

    #[test]
    fn test_part2() {
        let (state, program) = parse(INPUT2.lines());
        let result = process_part2(state, &program, 0)[0];
        assert_eq!(result, 117440);
    }
}

fn main() {
    let input = include_str!("input.txt");
    let (state, program) = parse(input.lines());

    let result_part1 = process_part1(state.clone(), &program);
    println!("{}", result_part1.iter().map(|o| o.to_string()).join(","));

    let result_part2 = process_part2(state, &program, 0)[0];
    println!("{}", result_part2);
}
