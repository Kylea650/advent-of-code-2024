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
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let part_one = process(&input, is_safe);
    let part_two = process(&input, is_safe_with_removal);
    println!("part 1: {}, part 2: {}", part_one, part_two);
}
