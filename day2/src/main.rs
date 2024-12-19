fn process(input: &str, safety_check_fn: fn(&Vec<u32>) -> bool) -> usize {
    input
        .lines()
        .map(|x| x.split_ascii_whitespace().map(|x| x.parse().unwrap()).collect())
        .filter(|x| safety_check_fn(x))
        .count()
}

fn is_safe(report: &Vec<u32>) -> bool {
    let check_ascending = report.is_sorted_by(|a, b| {a < b && a.abs_diff(*b) <= 3});
    let check_descending = report.is_sorted_by(|a, b| {a > b && a.abs_diff(*b) <= 3});
    check_ascending || check_descending
}

fn is_safe_with_removal(report: &Vec<u32>) -> bool {
    if is_safe(report) {
        return true
    }

    for i in 0..report.len() {
        let slice = report
            .into_iter()
            .enumerate()
            .filter(|&(idx, _)| idx != i)
            .map(|(_, &val)| val)
            .collect();

        if is_safe(&slice) {
            return true
        }
    }
    false
}

fn main() {
    let input = std::fs::read_to_string("./inputs/day2.txt").unwrap();
    let part_one = process(&input, is_safe);
    let part_two = process(&input, is_safe_with_removal);
    println!("part 1: {}, part 2: {}", part_one, part_two);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part_one() {
        let part_one = process(INPUT, is_safe);
        assert_eq!(part_one, 2);
    }

    #[test]
    fn test_part_two() {
        let part_two = process(INPUT, is_safe_with_removal);
        assert_eq!(part_two, 4);
    }
}