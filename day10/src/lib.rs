pub fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part_a(data: Vec<&str>) -> i32 {
    let mut x: i32 = 1;
    let mut compute: bool = false;
    let mut cycles_count: i32 = 0;
    let mut i: usize = 0;
    let mut total: i32 = 0;
    let mut signal_strength_cycle: i32 = 20;
    while i < data.len() {
        if cycles_count == signal_strength_cycle - 1 {
            total += signal_strength_cycle * x;
            signal_strength_cycle += 40;
        }

        match compute {
            true => {
                x += data[i].replace("addx ", "").parse::<i32>().unwrap();
                compute = false;
            }
            false => {
                compute = data[i].contains("addx");
            }
        };

        if !compute {
            i += 1;
        }

        cycles_count += 1;
    }
    total
}

pub fn part_b(data: Vec<&str>) -> String {
    let mut render = vec![vec!["."; 40]; 6];
    let mut x: i32 = 1;
    let mut compute = false;
    let mut cycles_count = 0;
    let mut i = 0;

    while i < data.len() {
        let drawing_position = cycles_count % 40;

        if drawing_position == x || drawing_position == x - 1 || drawing_position == x + 1 {
            let index: f32 = cycles_count as f32 / 40 as f32;
            render[index.floor() as usize][drawing_position as usize] = "#";
        }

        match compute {
            true => {
                x += data[i].replace("addx ", "").parse::<i32>().unwrap();
                compute = false;
            }
            false => {
                compute = data[i].contains("addx");
            }
        };

        if !compute {
            i += 1
        }

        cycles_count += 1;
    }

    render
        .iter()
        .map(|pixels| pixels.join(""))
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a_sample_data() {
        let result = part_a(parse_input(include_str!("sample_input.txt")));
        assert_eq!(result, 13140);
    }

    #[test]
    fn test_part_a() {
        let result = part_a(parse_input(include_str!("input.txt")));
        assert_eq!(result, 12840);
    }

    #[test]
    fn test_part_b_sample_data() {
        let result = part_b(parse_input(include_str!("sample_input.txt")));
        assert_eq!(
            result,
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
    }

    #[test]
    fn test_part_b() {
        let result = part_b(parse_input(include_str!("input.txt")));
        assert_eq!(
            result,
            "####.#..#...##.####.###....##.####.####.
...#.#.#.....#.#....#..#....#.#.......#.
..#..##......#.###..###.....#.###....#..
.#...#.#.....#.#....#..#....#.#.....#...
#....#.#..#..#.#....#..#.#..#.#....#....
####.#..#..##..#....###...##..#....####."
        );
    }
}
