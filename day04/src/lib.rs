type AssignmentPairs = Vec<Vec<(u32, u32)>>;

pub fn parse_input(input: &str) -> AssignmentPairs {
    input
        .split('\n')
        .map(|line| {
            line.split(',')
                .map(|pair| {
                    let parsed: Vec<u32> = pair
                        .split('-')
                        .map(|number| number.parse::<u32>().unwrap())
                        .collect();
                    (
                        parsed.get(0).unwrap().to_owned(),
                        parsed.get(1).unwrap().to_owned(),
                    )
                })
                .collect()
        })
        .collect()
}

pub fn part_a(assignment_pairs: AssignmentPairs) -> u32 {
    let mut count = 0;

    for pair in assignment_pairs {
        let (a, b) = pair.get(0).unwrap();
        let (c, d) = pair.get(1).unwrap();

        if (a <= c && b >= d) || (a >= c && b <= d) {
            count += 1;
        }
    }

    count
}

pub fn part_b(assignment_pairs: AssignmentPairs) -> u32 {
    let mut count = 0;

    for pair in assignment_pairs {
        let (a, b) = pair.get(0).unwrap();
        let (c, d) = pair.get(1).unwrap();

        if (a >= c && a <= d) || (b >= c && b <= d) || (c <= a && d >= a) || (c >= a && c <= b) {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_sample_part_a() {
        let result: u32 = part_a(parse_input(SAMPLE_INPUT));
        println!("assignment pairs: {}", result);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part_a() {
        let result: u32 = part_a(parse_input(include_str!("input.txt")));
        println!("assignment pairs: {}", result);
        assert_eq!(result, 560);
    }

    #[test]
    fn test_sample_part_b() {
        let result: u32 = part_b(parse_input(SAMPLE_INPUT));
        println!("assignment pairs: {}", result);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part_b() {
        let result: u32 = part_b(parse_input(include_str!("input.txt")));
        println!("assignment pairs: {}", result);
        assert_eq!(result, 839);
    }
}
