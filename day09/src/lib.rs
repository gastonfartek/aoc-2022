use std::collections::HashSet;

pub fn parse_input(input: &str) -> Vec<(&str, i32)> {
    input
        .lines()
        .map(|line| {
            let items: Vec<&str> = line.split(" ").collect();
            (items[0], items[1].parse::<i32>().unwrap())
        })
        .collect()
}
pub fn near(head: &(i32, i32), tail: &(i32, i32)) -> bool {
    (head.0 - tail.0).abs() <= 1 && (head.1 - tail.1).abs() <= 1
}

pub fn move_tail_next_to_head(head: (i32, i32), tail: &mut (i32, i32)) {
    if near(&head, tail) {
        return;
    }

    let mut diff;
    if head.0 != tail.0 {
        diff = (head.0 - tail.0) / (head.0 - tail.0).abs();
        tail.0 += diff;
    }

    if head.1 != tail.1 {
        diff = (head.1 - tail.1) / (head.1 - tail.1).abs();

        tail.1 += diff;
    }
}

pub fn part_a(data: Vec<(&str, i32)>) -> usize {
    let (mut head, mut tail) = ((0, 0), (0, 0));
    let mut visited = HashSet::new();
    visited.insert((0, 0));

    for i in 0..data.len() {
        let (move_, amount) = data[i];

        let mut orientation: i8 = 1;

        if move_ == "D" || move_ == "L" {
            orientation = -1;
        }

        for _ in 0..amount {
            if move_ == "D" || move_ == "U" {
                head.0 += orientation as i32;
            } else {
                head.1 += orientation as i32;
            }
            move_tail_next_to_head(head.clone(), &mut tail);
            visited.insert(tail.clone());
        }
    }
    visited.len()
}

pub fn part_b(data: Vec<(&str, i32)>) -> usize {
    let mut array = vec![(0, 0); 10];
    let mut visited = HashSet::new();
    visited.insert((0, 0));

    for i in 0..data.len() {
        let (move_, amount) = data[i];

        let mut orientation: i8 = 1;

        if move_ == "D" || move_ == "L" {
            orientation = -1;
        }

        for _ in 0..amount {
            if move_ == "D" || move_ == "U" {
                array[0].0 += orientation as i32;
            } else {
                array[0].1 += orientation as i32;
            }

            for j in 0..9 {
                let head = array[j];
                let tail = &mut array[j + 1];
                move_tail_next_to_head(head.clone(), tail);
                if j == 8 {
                    visited.insert(array[j + 1].clone());
                }
            }
        }
    }
    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_A: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const SAMPLE_INPUT_B: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_part_a_sample_input() {
        let result = part_a(parse_input(SAMPLE_INPUT_A));
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part_a() {
        let result = part_a(parse_input(include_str!("input.txt")));
        assert_eq!(result, 5902);
    }

    #[test]
    fn test_part_b_sample_input() {
        let result = part_b(parse_input(SAMPLE_INPUT_B));
        assert_eq!(result, 36);
    }

    #[test]
    fn test_part_b() {
        let result = part_b(parse_input(include_str!("input.txt")));
        assert_eq!(result, 2445);
    }
}
