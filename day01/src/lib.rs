pub fn parse_items(input: &str) -> Vec<u32> {
    input
        .trim()
        .split("\n\n")
        .map(|lines| -> u32 {
            lines
                .split('\n')
                .map(|l| l.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect()
}

pub fn part_a(input: &str) -> u32 {
    let counts: Vec<u32> = parse_items(input);
    counts.iter().max().unwrap().clone()
}

pub fn part_b(input: &str) -> u32 {
    let mut counts: Vec<u32> = parse_items(input);
    counts.sort_by(|a, b| b.cmp(a));
    counts.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_sample_parta() {
        let result: u32 = part_a(SAMPLE_INPUT);
        println!("max number of calories: {}", result);
        assert_eq!(result, 24000);
    }
    #[test]
    fn test_sample_partb() {
        let result: u32 = part_b(SAMPLE_INPUT);
        println!("top 3 number of calories sum: {}", result);
        assert_eq!(result, 45000);
    }

    #[test]
    fn test_parta() {
        let result: u32 = part_a(include_str!("input.txt"));
        println!("max number of calories: {}", result);
        assert_eq!(result, 65912);
    }

    #[test]
    fn test_partb() {
        let result: u32 = part_b(include_str!("input.txt"));
        println!("top 3 number of calories sum: {}", result);
        assert_eq!(result, 195625);
    }
}
