use std::collections::VecDeque;

#[derive(Debug, Clone, Default)]
pub enum OperationTypes {
    Add(i64),
    Multiply(i64),
    #[default]
    MultiplyOld,
}

#[derive(Debug, Clone, Default)]
pub struct Monkey {
    operation_type: OperationTypes,
    items: VecDeque<i64>,
    test: i64,
    result: (usize, usize),
}

pub fn parse_input(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|monkey| {
            let values: Vec<&str> = monkey.split("\n").collect();

            let items: VecDeque<i64> = values[1]
                .replace("Starting items: ", "")
                .split(',')
                .map(|s| s.trim().parse::<i64>().unwrap())
                .collect();

            let operation_string = values[2].replace("Operation: new = ", "");
            let operation_value = operation_string.replace("old + ", "").replace("old * ", "");
            let operation_type: OperationTypes;

            if operation_string.contains("old * old") {
                operation_type = OperationTypes::MultiplyOld;
            } else if operation_string.contains("+") {
                operation_type =
                    OperationTypes::Add(operation_value.trim().parse::<i64>().unwrap());
            } else {
                operation_type =
                    OperationTypes::Multiply(operation_value.trim().parse::<i64>().unwrap());
            }

            Monkey {
                items,
                operation_type,
                test: values[3]
                    .replace("Test: divisible by ", "")
                    .trim()
                    .parse::<i64>()
                    .unwrap(),
                result: (
                    values[4]
                        .replace("If true: throw to monkey", "")
                        .trim()
                        .parse::<usize>()
                        .unwrap(),
                    values[5]
                        .replace("If false: throw to monkey", "")
                        .trim()
                        .parse::<usize>()
                        .unwrap(),
                ),
            }
        })
        .collect()
}

pub fn get_worry_level(worryLevel: i64, operation: &OperationTypes) -> i64 {
    match operation {
        OperationTypes::Add(n) => worryLevel + n,
        OperationTypes::Multiply(n) => worryLevel * n,
        OperationTypes::MultiplyOld => worryLevel * worryLevel,
    }
}

pub fn part_a(input: &mut Vec<Monkey>) -> u32 {
    let mut monkeys = input.to_vec();
    let mut counts = vec![0; monkeys.len()];
    let mut rounds = 0;

    while rounds < 20 {
        for i in 0..monkeys.len() {
            let mut monkey = Monkey::default();
            std::mem::swap(&mut monkey, &mut monkeys[i]);
            while monkey.items.len() > 0 {
                counts[i] += 1;

                let item = monkey.items.pop_front().unwrap();

                let worry_level: f32 =
                    ((get_worry_level(item, &monkey.operation_type) as f32) / 3 as f32).floor();

                let next_monkey_index: usize;
                if worry_level % monkey.test as f32 > 0.0 {
                    next_monkey_index = monkey.result.1;
                } else {
                    next_monkey_index = monkey.result.0;
                }
                let next_monkey = &mut monkeys[next_monkey_index];

                next_monkey.items.push_back(worry_level as i64);
            }

            std::mem::swap(&mut monkey, &mut monkeys[i]);
        }

        rounds += 1;
    }
    counts.sort_by(|a, b| b.cmp(a));

    counts[0] * counts[1]
}

pub fn part_b(monkeys: &mut Vec<Monkey>) -> u64 {
    let total = monkeys.into_iter().fold(1, |a, b| b.test * a);

    let mut counts = vec![0; monkeys.len()];

    let mut rounds = 0;

    while rounds < 10000 {
        for i in 0..monkeys.len() {
            let mut monkey = Monkey::default();
            std::mem::swap(&mut monkey, &mut monkeys[i]);

            while monkey.items.len() > 0 {
                counts[i] += 1;
                let item = monkey.items.pop_front().unwrap();

                let mut worry_level = get_worry_level(item, &monkey.operation_type);

                worry_level %= total;

                let result_index = worry_level % monkey.test;

                let next_monkey;
                if result_index > 0 {
                    next_monkey = monkey.result.1
                } else {
                    next_monkey = monkey.result.0
                }

                monkeys[next_monkey].items.push_back(worry_level as i64);
            }

            std::mem::swap(&mut monkey, &mut monkeys[i]);
        }

        rounds += 1;
    }

    counts.sort_by(|a, b| b.cmp(a));
    counts[0] * counts[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a_sample_data() {
        let result = part_a(&mut parse_input(include_str!("sample_input.txt")));
        assert_eq!(result, 10605);
    }

    #[test]
    fn test_part_a() {
        let result = part_a(&mut parse_input(include_str!("input.txt")));
        assert_eq!(result, 110264);
    }

    #[test]
    fn test_part_b_sample_data() {
        let result = part_b(&mut parse_input(include_str!("sample_input.txt")));
        assert_eq!(result, 2713310158);
    }

    #[test]
    fn test_part_b() {
        let result = part_b(&mut parse_input(include_str!("input.txt")));
        assert_eq!(result, 23612457316);
    }
}
