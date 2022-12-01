pub fn parse_items(input: &str) -> Vec<i32> {
    let data: Vec<&str> = input.trim().split('\n').map(|l: &str| l.trim()).collect();

    let mut counts: Vec<i32> = Vec::new();
    let mut count: i32 = 0;

    for item in data {
        if item.len() == 0 {
            counts.push(count);
            count = 0;
            continue;
        }

        count += item.parse::<i32>().unwrap();
    }

    counts.sort();
    counts.reverse();

    counts
}

pub fn part_a(input: &str) -> i32 {
    let counts: Vec<i32> = parse_items(input);
    counts.first().unwrap().clone()
}

pub fn part_b(input: &str) -> i32 {
    let counts: Vec<i32> = parse_items(input);
    counts.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parta() {
        let result: i32 = part_a(include_str!("input.txt"));
        println!("max number of calories: {}", result);
        assert_eq!(result, 65912);
    }

    #[test]
    fn test_partb() {
        let result: i32 = part_b(include_str!("input.txt"));
        println!("top 3 number of calories sum: {}", result);
        assert_eq!(result, 195625);
    }
}
