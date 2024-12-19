use std::collections::{BTreeMap, HashMap};

fn parse_input(input: &str) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let mut split = input
        .split("\n\n");

    let mut map: HashMap<u32, Vec<u32>> = HashMap::new();

    let rules: Vec<(u32, u32)> = split
        .next().unwrap()
        .lines()
        .map(|line| {
            let mut split = line.split("|").map(|x| x.parse::<u32>().unwrap());
            (split.next().unwrap(), split.next().unwrap())
        })
        .collect();

    for value in rules.iter() {
        let node = map.entry(value.1).or_insert(vec![]);
        node.push(value.0);
        map.entry(value.0).or_insert(vec![]);
    }
    
    let updates: Vec<Vec<u32>> = split
        .next().unwrap()
        .lines()
        .map(|update| {
            update
                .split(",")
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    (map, updates)
}

fn process(updates: &Vec<Vec<u32>>, rule_map: &HashMap<u32, Vec<u32>>) -> (u32, u32) {
    let mut part_one = 0;
    let mut part_two = 0;

    for update_line in updates {

        let mapped_line: Vec<u32> = rule_map
            .into_iter()
            .filter(|(k, _)| update_line.contains(k))
            .map(|(k, v)| {
                let before = v.iter().filter(|x| update_line.contains(x)).map(|x| x.to_owned()).collect::<Vec<_>>().len() as u32;
                (before, *k)
            })
            .collect::<BTreeMap<u32, u32>>()
            .values()
            .map(|x| x.to_owned())
            .collect();

        let middle = {
            let index = mapped_line.len() / 2;
            mapped_line[index]
        };

        if *update_line == mapped_line {
            part_one += middle;
        } else {
            part_two += middle;
        }
    }
    (part_one, part_two)
}

fn main() {
    let input = std::fs::read_to_string("./inputs/day5.txt").unwrap();
    let parsed = parse_input(&input);

    let (part_one, part_two) = process(&parsed.1, &parsed.0);

    println!("part 1: {} part 2: {}", part_one, part_two);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part_one() {
        let parsed = parse_input(INPUT);
        let (part_one, _) = process(&parsed.1, &parsed.0);
        assert_eq!(part_one, 143);
    }

    #[test]
    fn test_part_two() {
        let parsed = parse_input(INPUT);
        let (_, part_two) = process(&parsed.1, &parsed.0);
        assert_eq!(part_two, 123);
    }
}
