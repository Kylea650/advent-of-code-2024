#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
    Concat,
}

fn process(input: &str, filter_fn: fn(&(u64, Vec<u64>)) -> bool) -> u64 {
    input.lines().map(|equation| {
        let mut split = equation.split(": ");
        let test_value = split.next().unwrap().parse::<u64>().unwrap();
        let numbers: Vec<u64> = split
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        (test_value, numbers)
    })
    .filter(|x| filter_fn(x))
    .map(|(val, _)| val)
    .sum()
}

fn part_one(input: &(u64, Vec<u64>)) -> bool {
    let (test_value, numbers) = input;
    let configurations = get_binary_configurations(numbers.len());

    for configuration in configurations {
        let mut result = numbers[0];
        for (i, number) in numbers.iter().skip(1).enumerate() {
            match configuration[i] {
                Operation::Add => result += number,
                Operation::Multiply => result *= number,
                _ => panic!("lol"),
            }
        }
        if result == *test_value {
            return true;
        }
    }
    false
}

fn part_two(input: &(u64, Vec<u64>)) -> bool {
    let (test_value, numbers) = input;
    let configurations = get_ternary_configurations(numbers.len());

    for configuration in configurations {
        let mut result = numbers[0];
        for (i, number) in numbers.iter().skip(1).enumerate() {
            match configuration[i] {
                Operation::Add => result += number,
                Operation::Multiply => result *= number,
                Operation::Concat => {
                    let mut result_str = result.to_string();
                    result_str.push_str(&number.to_string());
                    result = result_str.parse().unwrap();
                }
            }
        }
        if result == *test_value {
            return true;
        }
    }
    false
}

fn get_binary_configurations(n: usize) -> Vec<Vec<Operation>> {
    let mut results = Vec::new();

    let max_configurations = 1 << n - 1;
    for i in 0..max_configurations {
        let mut configuration = Vec::new();
        for j in 0..n - 1 {
            if i & (1 << j) != 0 {
                configuration.push(Operation::Add);
            } else {
                configuration.push(Operation::Multiply);
            }
        }
        results.push(configuration);
    }
    results
}

fn get_ternary_configurations(n: usize) -> Vec<Vec<Operation>> {
    let mut results = Vec::new();

    let max_configurations = 3_u64.pow(n as u32) - 1;
    for i in 0..max_configurations {
        let mut configuration = Vec::new();
        for j in 0..n - 1 {
            match i / 3_u64.pow(j as u32) % 3 {
                0 => configuration.push(Operation::Add),
                1 => configuration.push(Operation::Multiply),
                2 => configuration.push(Operation::Concat),
                _ => panic!("lol"),
            }
        }
        results.push(configuration);
    }
    results
}

fn main() {
    let input = std::fs::read_to_string("inputs/day7.txt").unwrap();
    let part_one = process(&input, part_one);
    let part_two = process(&input, part_two);
    println!("Part 1: {} Part 2: {}", part_one, part_two);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part_one() {
        let part_one = process(&INPUT, part_one);
        assert_eq!(part_one, 3749);
    }

    #[test]
    fn test_part_two() {
        let part_two = process(&INPUT, part_two);
        assert_eq!(part_two, 11387);
    }
}
