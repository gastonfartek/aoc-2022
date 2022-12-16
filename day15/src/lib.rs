use std::collections::HashSet;

pub struct Data {
    sensor: Vec<i32>,
    beacon: Vec<i32>,
}

pub fn parse_input(input: &str) -> Vec<Data> {
    input
        .split("\n")
        .map(|line| {
            let parts: Vec<&str> = line.split(":").collect();

            let sensor: Vec<i32> = parts[0]
                .replace("Sensor at x=", "")
                .replace(" y=", "")
                .split(",")
                .map(|n| n.parse::<i32>().unwrap())
                .collect();

            let beacon: Vec<i32> = parts[1]
                .replace(" closest beacon is at x=", "")
                .replace(" y=", "")
                .split(",")
                .map(|n| n.parse::<i32>().unwrap())
                .collect();

            Data { sensor, beacon }
        })
        .collect()
}

pub fn part_a(data: Vec<Data>, y: i32) -> i32 {
    let mut not_beacons: HashSet<i32> = HashSet::new();

    data.iter().for_each(|input| {
        let result =
            (input.sensor[0] - input.beacon[0]).abs() + (input.sensor[1] - input.beacon[1]).abs();

        let mut count = result.clone();

        for i in 0..=result {
            if input.sensor[1] + i != y && input.sensor[1] - i != y {
                count -= 1;
                continue;
            }

            for j in 0..=count {
                if input.sensor[0] - j != input.beacon[0] {
                    not_beacons.insert(input.sensor[0] - j);
                }

                if input.sensor[0] + j != input.beacon[0] {
                    not_beacons.insert(input.sensor[0] + j);
                }
            }

            count -= 1;
        }
    });
    (not_beacons.len() + 1) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a_sample_data() {
        let result = part_a(parse_input(include_str!("sample_input.txt")), 10);
        assert_eq!(result, 26);
    }

    #[test]
    fn test_part_a() {
        let result = part_a(parse_input(include_str!("input.txt")), 2000000);
        assert_eq!(result, 5335787);
    }
}
