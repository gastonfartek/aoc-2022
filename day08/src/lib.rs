type Input = Vec<Vec<u8>>;
pub fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|n| n.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

pub fn part_a(matrix: Input) -> u32 {
    let is_visible = |y: usize, x: usize| -> bool {
        let value = matrix[y][x];
        let mut visible_points: u8 = 4;

        //up
        for (_, i) in (0..y).rev().enumerate() {
            if matrix[i][x] >= value {
                visible_points -= 1;
                break;
            }
        }

        //down
        for (_, i) in (y + 1..matrix.len()).enumerate() {
            if matrix[i][x] >= value {
                visible_points -= 1;
                break;
            }
        }

        // //left
        for (_, i) in (0..x).rev().enumerate() {
            if matrix[y][i] >= value {
                visible_points -= 1;
                break;
            }
        }

        // //right
        for (_, i) in (x + 1..matrix[y].len()).enumerate() {
            if matrix[y][i] >= value {
                visible_points -= 1;
                break;
            }
        }

        visible_points > 0
    };

    let mut visible: u32 = 0;
    for y in 1..matrix.len() - 1 {
        for x in 1..matrix[0].len() - 1 {
            if is_visible(y, x) {
                visible += 1;
            }
        }
    }

    visible += matrix.len() as u32 * 2;
    visible += (matrix.len() as u32 - 2) * 2;
    visible
}

pub fn part_b(matrix: Input) -> u32 {
    let calculate_score = |y: usize, x: usize| -> u32 {
        let value = matrix[y][x];
        let mut scores = vec![0, 0, 0, 0];

        for (_, i) in (0..y).rev().enumerate() {
            scores[0] += 1;
            if matrix[i][x] >= value {
                break;
            }
        }

        //down
        for (_, i) in (y + 1..matrix.len()).enumerate() {
            scores[1] += 1;
            if matrix[i][x] >= value {
                break;
            }
        }

        // //left
        for (_, i) in (0..x).rev().enumerate() {
            scores[2] += 1;
            if matrix[y][i] >= value {
                break;
            }
        }

        // //right
        for (_, i) in (x + 1..matrix[y].len()).enumerate() {
            scores[3] += 1;
            if matrix[y][i] >= value {
                break;
            }
        }

        scores.iter().fold(1, |a, b| a * b)
    };

    let mut prev: u32 = 0;
    for (_, y) in (1..matrix.len() - 1).enumerate() {
        for (_, x) in (1..matrix[0].len() - 1).enumerate() {
            let score = calculate_score(y, x);
            if score > prev {
                prev = score
            }
        }
    }

    prev
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_sample_part_a() {
        let result: u32 = part_a(parse_input(SAMPLE_INPUT));
        println!("part A sample input: {}", result);
        assert_eq!(21, result);
    }

    #[test]
    fn test_part_a() {
        let result: u32 = part_a(parse_input(include_str!("input.txt")));
        println!("part A input: {:#?}", result);
        assert_eq!(result, 1719);
    }

    #[test]
    fn test_sample_part_b() {
        let result: u32 = part_b(parse_input(SAMPLE_INPUT));
        println!("part B sample input: {}", result);
        assert_eq!(8, result);
    }

    #[test]
    fn test_part_b() {
        let result: u32 = part_b(parse_input(include_str!("input.txt")));
        println!("part A sample input: {:#?}", result);
        assert_eq!(590824, result);
    }
}
