use std::collections::HashMap;

pub fn get_points_map() -> HashMap<&'static str, u8> {
    let mut points_map: HashMap<&str, u8> = HashMap::new();
    points_map.insert("A", 1);
    points_map.insert("B", 2);
    points_map.insert("C", 3);
    points_map.insert("X", 1);
    points_map.insert("Y", 2);
    points_map.insert("Z", 3);
    points_map
}

pub fn get_result_map() -> HashMap<u8, u8> {
    let mut result_map: HashMap<u8, u8> = HashMap::new();
    result_map.insert(1, 2);
    result_map.insert(2, 3);
    result_map.insert(3, 1);
    result_map
}

pub fn parse_input(input: &str) -> Vec<(u8, u8)> {
    let points_map = get_points_map();

    input
        .split('\n')
        .map(|l| {
            let items: Vec<&str> = l.split(' ').collect();
            (
                points_map.get(items.get(0).unwrap()).unwrap().to_owned(),
                points_map.get(items.get(1).unwrap()).unwrap().to_owned(),
            )
        })
        .collect()
}

pub fn part_a(input: Vec<(u8, u8)>) -> u32 {
    let results_map = get_result_map();

    let mut points: u32 = 0;
    for value in input {
        let (opponent, player) = value;
        points += u32::from(player);
        if opponent == player {
            points += 3;
            continue;
        }

        let winning_play = results_map.get(&opponent).unwrap().to_owned();
        if player == winning_play {
            points += 6;
        }
    }

    points
}

pub fn part_b(input: Vec<(u8, u8)>) -> u32 {
    let result_map = get_result_map();

    let find_loosing_value = |v: u8| -> Option<u8> {
        result_map
            .iter()
            .find_map(|(key, &val)| if val == v { Some(key.to_owned()) } else { None })
    };

    let mut points: u32 = 0;
    for v in input {
        let (opponent, result) = v;

        if result == 1 {
            //loose
            points += u32::from(find_loosing_value(opponent).unwrap());
            continue;
        }

        if result == 2 {
            //draw
            points += 3 + u32::from(opponent);
            continue;
        }

        points += 6 + u32::from(result_map.get(&opponent).unwrap().to_owned())
    }

    points
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn test_sample_part_a() {
        let result: u32 = part_a(parse_input(SAMPLE_INPUT));
        println!("player points: {}", result);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_part_a() {
        let result: u32 = part_a(parse_input(include_str!("input.txt")));
        println!("player points: {}", result);
        assert_eq!(result, 13005);
    }

    #[test]
    fn test_sample_part_b() {
        let result: u32 = part_b(parse_input(SAMPLE_INPUT));
        println!("player points: {}", result);
        assert_eq!(result, 12);
    }

    #[test]
    fn test_part_b() {
        let result: u32 = part_b(parse_input(include_str!("input.txt")));
        println!("player points: {}", result);
        assert_eq!(result, 11373);
    }
}
