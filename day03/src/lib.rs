pub fn calculate_value(letter: char) -> u32 {
    let code: u32 = letter as u32;

    if code >= 97 {
        return code - 96;
    }

    code - 38
}

pub fn part_a(input: &str) -> u32 {
    let mut total: u32 = 0;
    for line in input.lines() {
        let a = &line[..line.len() / 2];
        let b = &line[line.len() / 2..];

        for letter in a.chars() {
            if b.contains(letter) {
                total += calculate_value(letter);
                break;
            }
        }
    }
    total
}

pub fn part_b(input: &str) -> u32 {
    let data: Vec<&str> = input.lines().collect();

    let mut i: usize = 0;
    let mut total: u32 = 0;
    while i < data.len() {
        let b = data[i + 1];
        let c = data[i + 2];

        for letter in data[i].chars() {
            if b.contains(letter) && c.contains(letter) {
                total += calculate_value(letter);
                break;
            }
        }
        i += 3;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_sample_part_a() {
        let result: u32 = part_a(SAMPLE_INPUT);
        println!("total: {}", result);
        assert_eq!(result, 157);
    }

    #[test]
    fn test_part_a() {
        let result: u32 = part_a(include_str!("input.txt"));
        println!("total: {}", result);
        assert_eq!(result, 8401);
    }

    #[test]
    fn test_sample_part_b() {
        let result: u32 = part_b(SAMPLE_INPUT);
        println!("total: {}", result);
        assert_eq!(result, 70);
    }

    #[test]
    fn test_part_b() {
        let result: u32 = part_b(include_str!("input.txt"));
        println!("total: {}", result);
        assert_eq!(result, 2641);
    }
}
