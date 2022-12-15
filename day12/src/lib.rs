use std::{
    cmp::{self, Ordering},
    collections::{BinaryHeap, HashMap, HashSet},
};

#[derive(Debug, Clone)]
pub struct Input {
    start: (usize, usize),
    target: (usize, usize),
    data: Vec<Vec<usize>>,
}

#[derive(Clone, Debug)]
struct Adjacency {
    position: (usize, usize),
    cost: usize,
}

impl Adjacency {
    fn new(position: (usize, usize), cost: usize) -> Self {
        Adjacency { position, cost }
    }
}

impl Ord for Adjacency {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

impl PartialOrd for Adjacency {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cost.cmp(&other.cost).reverse())
    }
}

impl PartialEq for Adjacency {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for Adjacency {}

pub fn parse_input(input: &str) -> Input {
    let mut start: (usize, usize) = (0, 0);
    let mut target: (usize, usize) = (0, 0);
    let mut data: Vec<Vec<usize>> = Vec::new();
    let lines = input.lines();
    for (i, line) in lines.enumerate() {
        let mut vec: Vec<usize> = Vec::new();
        for (j, letter) in line.chars().enumerate() {
            let mut value = letter as usize;
            if letter == 'S' {
                start = (i, j);
                value = 'a' as usize
            }

            if letter == 'E' {
                target = (i, j);
                value = 'z' as usize
            }

            vec.push(value);
        }

        data.push(vec);
    }

    Input {
        start,
        target,
        data,
    }
}

pub fn part_a(input: Input) -> u32 {
    let mut heap: BinaryHeap<Adjacency> = BinaryHeap::new();
    let mut adjacency_list: HashMap<String, Vec<Adjacency>> = HashMap::new();
    let mut processed = HashSet::new();

    for row in 0..input.data.len() {
        for column in 0..input.data[0].len() {
            let key = format!("{},{}", row.to_string(), column.to_string());
            let value = input.data[row][column];
            let map = adjacency_list.entry(key.clone()).or_default();

            if (row + 1) < input.data.len() && input.data[row + 1][column] <= value + 1 {
                map.push(Adjacency::new((row + 1, column), 0));
            }

            if row > 0 && input.data[row - 1][column] <= (value + 1) {
                map.push(Adjacency::new((row - 1, column), 0));
            }

            if (column + 1) < input.data[row].len() && input.data[row][column + 1] <= value + 1 {
                map.push(Adjacency::new((row, column + 1), 0));
            }

            if column > 0 && input.data[row][column - 1] <= (value + 1) {
                map.push(Adjacency::new((row, column - 1), 0));
            }
        }
    }

    let start = Adjacency::new((input.start.0, input.start.1), 0);
    heap.push(start);
    let mut steps = 0;
    while heap.len() > 0 {
        let adjacency = heap.pop().unwrap();
        let (row, column) = adjacency.position;
        let cost = adjacency.cost;

        let key = format!("{},{}", row.to_string(), column.to_string());

        if processed.contains(&key) {
            continue;
        }

        processed.insert(key.clone());

        if row == input.target.0 && column == input.target.1 {
            steps = cost;
            break;
        }

        if adjacency_list.contains_key(&key) {
            for adj in &adjacency_list[&key] {
                heap.push(Adjacency {
                    cost: cost + 1,
                    position: adj.position,
                })
            }
        }
    }

    steps as u32
}

pub fn part_b(input: Input) -> u32 {
    let mut min_steps = u32::MAX;
    for (i, value) in input.data[0].iter().enumerate() {
        if *value == 'a' as usize {
            let steps = part_a(Input {
                start: (0, i),
                ..input.clone()
            });
            if steps > 0 {
                min_steps = cmp::min(min_steps, steps);
            }
        }
    }
    for i in 0..input.data.len() {
        if input.data[i][0] == 'a' as usize {
            let steps = part_a(Input {
                start: (i, 0),
                ..input.clone()
            });

            if steps > 0 {
                min_steps = cmp::min(min_steps, steps);
            }
        }
    }

    min_steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a_sample_data() {
        let result = part_a(parse_input(include_str!("sample_input.txt")));
        assert_eq!(result, 31);
    }
    #[test]
    fn test_part_a() {
        let result = part_a(parse_input(include_str!("input.txt")));
        assert_eq!(result, 504);
    }
    #[test]
    fn test_part_b_sample_data() {
        let result = part_b(parse_input(include_str!("sample_input.txt")));
        assert_eq!(result, 29);
    }
    #[test]
    fn test_part_b() {
        let result = part_b(parse_input(include_str!("input.txt")));
        assert_eq!(result, 500);
    }
}
