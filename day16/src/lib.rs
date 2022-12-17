use regex::Regex;
use std::{cmp, collections::HashMap};

// use cached::proc_macro::cached;
// use cached::SizedCache;

pub struct Data {
    flow_rates: HashMap<String, i32>,
    adj_list: HashMap<String, Vec<String>>,
}

pub fn parse_input(input: &str) -> Data {
    let mut flow_rates = HashMap::new();
    let mut adj_list = HashMap::new();

    input.split("\n").for_each(|line| {
        let parts: Vec<&str> = line.split(";").collect();
        let left: Vec<String> = parts[0]
            .replace("Valve", "")
            .trim()
            .replace("has flow rate=", "")
            .split(" ")
            .map(String::from)
            .collect();

        let tunnels: Vec<String> = Regex::new(r"\s*tunnel[s]? lead[s]? to valve[s]?\s*")
            .unwrap()
            .replace(parts[1], "")
            .split(',')
            .map(|s| String::from(s.trim()))
            .collect();

        flow_rates.insert(left[0].clone(), left[1].parse::<i32>().unwrap());
        adj_list.insert(left[0].clone(), tunnels);
    });

    Data {
        flow_rates,
        adj_list,
    }
}

// #[cached(
//     type = "SizedCache<String, i32>",
//     create = "{ SizedCache::with_size(100) }",
//     convert = r#"{ format!("{}{}{:?}", valve_key, minutes, opened) }"#
// )]
pub fn solve(
    data: &Data,
    valve_key: String,
    minutes: i32,
    opened: &mut Vec<String>,
    cache: &mut HashMap<String, i32>,
) -> i32 {
    opened.sort();
    let cache_key = format!("{}:{}:{:?}", valve_key, minutes, opened);

    if cache.contains_key(&cache_key) {
        return *cache.get(&cache_key).unwrap();
    }

    if minutes <= 0 {
        return 0;
    }

    let mut result = 0;

    if !opened.contains(&valve_key) {
        let total = (minutes - 1) * data.flow_rates[&valve_key];
        let mut newly_opened = opened.clone();
        newly_opened.push(valve_key.clone());

        for next_valve in &data.adj_list[&valve_key] {
            if total != 0 {
                result = cmp::max(
                    result,
                    total
                        + solve(
                            data,
                            next_valve.clone(),
                            minutes - 2,
                            &mut newly_opened,
                            cache,
                        ),
                )
            } else {
                result = cmp::max(
                    result,
                    solve(data, next_valve.clone(), minutes - 1, opened, cache),
                )
            }
        }
    }

    cache.insert(cache_key, result);

    result
}

pub fn part_a(data: &Data) -> i32 {
    let valve_key = String::from("AA");
    let mut cache: HashMap<String, i32> = HashMap::new();
    solve(data, valve_key, 30 as i32, &mut Vec::new(), &mut cache)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a_sample_data() {
        let result = part_a(&mut parse_input(include_str!("sample_input.txt")));
        assert_eq!(result, 1630);
    }

    #[test]
    fn test_part_a() {
        let result = part_a(&mut parse_input(include_str!("input.txt")));
        assert_eq!(result, 2183);
    }
}
