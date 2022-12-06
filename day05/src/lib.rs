use std::{borrow::BorrowMut, collections::VecDeque};

#[derive(Debug)]
pub struct Move {
    from: usize,
    to: usize,
    amount: usize,
}

#[derive(Debug)]
pub struct ParsedInput {
    stacks: Vec<VecDeque<char>>,
    moves: Vec<Move>,
}

pub fn parse_input(input: &str) -> ParsedInput {
    let values: Vec<&str> = input.split("\n\n").collect();
    let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::new()];

    values.get(0).unwrap().lines().for_each(|line| {
        let characters: Vec<char> = line.chars().collect();

        let mut i = 0;
        let mut stack_index = 0;
        while i < characters.len() {
            if stacks.get(stack_index).is_none() {
                stacks.push(VecDeque::new());
            }

            if *characters.get(i).unwrap() == '[' {
                stacks[stack_index].push_front(*characters.get(i + 1).unwrap());
            }

            stack_index += 1;
            i += 4;
        }
    });

    let moves: Vec<Move> = values
        .get(1)
        .unwrap()
        .split('\n')
        .map(|line| {
            let values: Vec<usize> = line
                .replace("move ", "")
                .replace(" from ", ",")
                .replace(" to ", ",")
                .split(",")
                .map(|n| n.parse::<usize>().unwrap())
                .collect();

            Move {
                amount: *values.get(0).unwrap(),
                from: *values.get(1).unwrap(),
                to: *values.get(2).unwrap(),
            }
        })
        .collect();

    ParsedInput { stacks, moves }
}

pub fn part_a(input: &mut ParsedInput) -> String {
    for m in &input.moves {
        for _ in 0..m.amount {
            let from_value = input.stacks[m.from - 1].pop_back().unwrap();
            let to: &mut VecDeque<char> = input.stacks[m.to - 1].borrow_mut();
            to.push_back(from_value);
        }
    }

    input
        .stacks
        .iter_mut()
        .map(|q| q.pop_back().unwrap())
        .collect::<String>()
}

pub fn part_b(input: &mut ParsedInput) -> String {
    for m in &input.moves {
        let mut temp: Vec<char> = Vec::new();
        for _ in 0..m.amount {
            temp.push(input.stacks[m.from - 1].pop_back().unwrap());
        }

        temp.reverse();

        for item in temp {
            input.stacks[m.to - 1].push_back(item);
        }
    }

    input
        .stacks
        .iter_mut()
        .map(|q| q.pop_back().unwrap())
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "    [D]
[N] [C]
[Z] [M] [P]
1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_sample_part_a() {
        let result: String = part_a(&mut parse_input(SAMPLE_INPUT));
        println!("part A sample input: {:#?}", result);
        assert_eq!("CMZ", result);
    }

    #[test]
    fn test_part_a() {
        let result: String = part_a(&mut parse_input(include_str!("input.txt")));
        println!("part A input: {:#?}", result);
        assert_eq!(result, "TLFGBZHCN");
    }

    #[test]
    fn test_sample_part_b() {
        let result: String = part_b(&mut parse_input(SAMPLE_INPUT));
        println!("part A sample input: {:#?}", result);
        assert_eq!("MCD", result);
    }

    #[test]
    fn test_part_b() {
        let result: String = part_b(&mut parse_input(include_str!("input.txt")));
        println!("part A sample input: {:#?}", result);
        assert_eq!("QRQFHFWCL", result);
    }
}
