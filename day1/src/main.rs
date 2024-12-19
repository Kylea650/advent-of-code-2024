fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>){
    let (mut left, mut right) = input
        .lines()
        .map(|x|x.split_ascii_whitespace().collect::<Vec<&str>>())
        .fold((Vec::new(), Vec::new()), |(mut left, mut right), vec| {
            left.push(vec[0].parse().unwrap());
            right.push(vec[1].parse().unwrap());
            (left, right)
        });
    left.sort();
    right.sort();

    (left, right)
}

fn main() {
    let input = std::fs::read_to_string("./inputs/day1.txt").unwrap();
    let parsed = parse_input(&input);
    
    let part_one = (0..parsed.0.len()).fold(0, |acc, idx| {
        acc + parsed.0[idx].abs_diff(parsed.1[idx])
    });

    let part_two = (0..parsed.0.len()).fold(0, |acc, idx| {
        let to_find = &parsed.0[idx];
        let num_found = parsed.1.iter().filter(|&x| x == to_find).count();
        acc + (to_find * num_found as u32)
    });
    
    println!("part 1: {} part 2: {}", part_one, part_two);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part_one() {
        let parsed = parse_input(INPUT);

        let part_one = (0..parsed.0.len()).fold(0, |acc, idx| {
            acc + parsed.0[idx].abs_diff(parsed.1[idx])
        });
        assert_eq!(part_one, 11);
    }

    #[test]
    fn test_part_two() {
        let parsed = parse_input(INPUT);

        let part_two = (0..parsed.0.len()).fold(0, |acc, idx| {
            let to_find = &parsed.0[idx];
            let num_found = parsed.1.iter().filter(|&x| x == to_find).count();
            acc + (to_find * num_found as u32)
        });
        assert_eq!(part_two, 31);
    }
}