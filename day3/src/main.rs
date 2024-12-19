use regex::Regex;

fn part_one(input: &str) -> u32 {
    // match 2 groups: ""a", "b")
    let re = Regex::new(r"mul\(([1-9]|[1-9][0-9]|[1-9][0-9][0-9]),([1-9]|[1-9][0-9]|[1-9][0-9][0-9])\)").unwrap();

    re.captures_iter(input).map(|c| c.extract()).fold(0, |acc, (_, [a, b])| {
        acc + (a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap())
    })
}

fn part_two(input: &str) -> u32 {
    // match 4 groups: "don't()", "do()", "a", "b")
    let re = Regex::new(r"(don't\(\))|(do\(\))|mul\(([1-9]|[1-9][0-9]|[1-9][0-9][0-9]),([1-9]|[1-9][0-9]|[1-9][0-9][0-9])\)").unwrap();
    let mut process = true;

    re.captures_iter(input).fold(0, |acc, c| {
        let mut sum = 0;

        if c.get(1).is_some_and(|x| x.as_str() == "don't()") {
            process = false;
        };
        if c.get(2).is_some_and(|x| x.as_str() == "do()") {
            process = true;
        };
        if process && c.get(3).is_some() && c.get(4).is_some() {
            sum = c.get(3).unwrap().as_str().parse::<u32>().unwrap() * c.get(4).unwrap().as_str().parse::<u32>().unwrap();
        }
        acc + sum
    })
}

fn main() {
    let input = std::fs::read_to_string("./inputs/day3.txt").unwrap();
    let part_one = part_one(&input);
    let part_two = part_two(&input);
    println!("part 1: {} part 2: {}", part_one, part_two);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 161);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), 48);
    }
}