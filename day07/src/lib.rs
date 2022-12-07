use std::collections::HashMap;

pub fn parse_input(input: &str) -> HashMap<String, u32> {
    let commands: Vec<&str> = input.lines().collect();
    let mut current_path: Vec<&str> = Vec::new();
    let mut folder_sizes: HashMap<String, u32> = HashMap::new();

    commands.iter().for_each(|command| {
        let values: Vec<&str> = command.split(" ").collect();

        if values[0] == "$" && values[1] == "cd" {
            if values[2] == "/" {
                current_path = Vec::from(["/"]);
            } else if values[2] == ".." {
                current_path.pop();
            } else {
                current_path.push(values[2]);
            }
        }

        if values[0] != "$" && values[0] != "dir" {
            let mut prev: Vec<&str> = Vec::new();
            current_path.iter().for_each(|k| {
                prev.push(k);
                let key = prev.join("/");
                *folder_sizes.entry(key).or_insert(0) += values[0].parse::<u32>().unwrap();
            })
        }
    });

    folder_sizes
}

pub fn part_a(folder_sizes: HashMap<String, u32>) -> u32 {
    folder_sizes
        .values()
        .cloned()
        .collect::<Vec<u32>>()
        .into_iter()
        .filter(|v| *v <= 100000)
        .sum()
}

pub fn part_b(folder_sizes: HashMap<String, u32>) -> u32 {
    let free_space_needed = 30000000 - (70000000 - folder_sizes.get("/").unwrap());
    folder_sizes
        .values()
        .cloned()
        .collect::<Vec<u32>>()
        .into_iter()
        .fold(u32::MAX, |prev, value| {
            if value >= free_space_needed && value < prev {
                return value;
            }

            prev
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_sample_part_a() {
        let result: u32 = part_a(parse_input(SAMPLE_INPUT));
        println!("part A sample input: {}", result);
        assert_eq!(95437, result);
    }

    #[test]
    fn test_part_a() {
        let result: u32 = part_a(parse_input(include_str!("input.txt")));
        println!("part A input: {:#?}", result);
        assert_eq!(result, 1182909);
    }

    #[test]
    fn test_sample_part_b() {
        let result: u32 = part_b(parse_input(SAMPLE_INPUT));
        println!("part B sample input: {}", result);
        assert_eq!(24933642, result);
    }

    #[test]
    fn test_part_b() {
        let result: u32 = part_b(parse_input(include_str!("input.txt")));
        println!("part A sample input: {:#?}", result);
        assert_eq!(2832508, result);
    }
}
