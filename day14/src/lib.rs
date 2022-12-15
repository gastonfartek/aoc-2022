use std::{cmp, collections::HashSet};

pub type Data = HashSet<(i32, i32)>;
pub struct Input {
    data: Data,
    max_y: i32,
}

pub fn parse_input(input: &str) -> Input {
    let mut data: Data = HashSet::new();
    let mut max_y: i32 = 0;

    input.split('\n').for_each(|l| {
        let values: Vec<Vec<i32>> = l
            .split(" -> ")
            .map(|item| item.split(',').map(|n| n.parse::<i32>().unwrap()).collect())
            .collect();

        for i in 1..values.len() {
            let from = &values[i - 1];
            let to = &values[i];
            max_y = cmp::max(max_y, from[1]);
            max_y = cmp::max(max_y, to[1]);

            //increment x
            if from[0] != to[0] {
                let diff = if to[0] - from[0] > 0 { 1 } else { -1 };
                let mut target = from[0];
                while target != to[0] {
                    data.insert((target, from[1]));
                    target += diff;
                }
            }

            //increment y
            if from[1] != to[1] {
                let diff = if to[1] - from[1] > 0 { 1 } else { -1 };
                let mut target = from[1];
                while target != to[1] {
                    data.insert((from[0], target));
                    target += diff;
                }
            }

            data.insert((to[0], to[1]));
        }
    });

    Input { data, max_y }
}

pub fn get_next_move(map: &Data, next_move: Option<(i32, i32)>) -> Option<(i32, i32)> {
    match next_move {
        Some(coords) => {
            let down = (coords.0, coords.1 + 1);
            let left = (coords.0 - 1, coords.1 + 1);
            let right = (coords.0 + 1, coords.1 + 1);

            if !map.contains(&down) {
                return Some(down);
            }

            if !map.contains(&left) {
                return Some(left);
            }

            if !map.contains(&right) {
                return Some(right);
            }

            None
        }
        None => None,
    }
}

pub fn part_a(input: &mut Input) -> u32 {
    let mut count: u32 = 0;

    loop {
        let mut next_move = Some((500, 0));
        let mut prev: Option<(i32, i32)> = None;
        while next_move.is_some() && next_move.unwrap().1 <= input.max_y {
            next_move = get_next_move(&input.data, next_move);
            if next_move.is_none() {
                input.data.insert(prev.unwrap());
                break;
            }
            prev = next_move;
        }

        if prev.is_none() || prev.unwrap().1 > input.max_y {
            break;
        }

        count += 1;
    }

    count
}

pub fn part_b(input: &mut Input) -> u32 {
    let mut count: u32 = 0;

    loop {
        let mut next_move = Some((500, 0));
        let mut prev: Option<(i32, i32)> = None;
        let mut moves = 0;

        while next_move.is_some() {
            next_move = get_next_move(&input.data, next_move);
            if next_move.is_none() || next_move.unwrap().1 == input.max_y + 2 {
                if prev.is_some() {
                    input.data.insert(prev.unwrap());
                }
                break;
            }
            moves += 1;
            prev = next_move;
        }

        count += 1;

        if moves == 0 {
            break;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a_sample_data() {
        let result = part_a(&mut parse_input(include_str!("sample_input.txt")));
        assert_eq!(result, 24);
    }

    #[test]
    fn test_part_a() {
        let result = part_a(&mut parse_input(include_str!("input.txt")));
        assert_eq!(result, 618);
    }

    #[test]
    fn test_part_b_sample_data() {
        let result = part_b(&mut parse_input(include_str!("sample_input.txt")));
        assert_eq!(result, 93);
    }

    #[test]
    fn test_part_b() {
        let result = part_b(&mut parse_input(include_str!("input.txt")));
        assert_eq!(result, 26358);
    }
}
