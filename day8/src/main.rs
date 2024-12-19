use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn parse_input(input: &str) -> (HashMap<char, Vec<(usize, usize)>>, (usize, usize)) {
    let mut result: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut x = Vec::new();
    let mut y = Vec::new();

    for (i, val) in input.lines().rev().enumerate() {
        y.push(i);
        for (j, char) in val.chars().enumerate() {
            x.push(j);
            if char.is_alphanumeric() {
                result.entry(char).or_insert_with(Vec::new).push((j, i));
            }
        }
    }
    let max = (x.into_iter().max().unwrap(), y.into_iter().max().unwrap());
    (result, max)
}

fn add_antinodes(coordinates: &Vec<(usize, usize)>, max_size: (usize, usize)) -> Vec<(usize, usize)> {
    coordinates.iter().combinations(2).map(|combination| {
        let mut antinodes = Vec::new();
        
        let antinode_1 = get_next_antinode(*combination[0], *combination[1], max_size, false);
        if antinode_1.is_some() {
            antinodes.push(antinode_1.unwrap());
        }

        let antinode_2 = get_next_antinode(*combination[0], *combination[1], max_size, true);
        if antinode_2.is_some() {
            antinodes.push(antinode_2.unwrap());
        }

        antinodes
    })
    .flatten()
    .collect()
}

fn get_next_antinode(a: (usize, usize), b: (usize, usize), max_size: (usize, usize), reverse: bool) -> Option<(usize, usize)> {
    match reverse {
        false => {
            let x = a.0 as isize * 2 - b.0  as isize;
            let y = a.1  as isize * 2 - b.1 as isize;
            if x >= 0 && y >= 0 && x <= max_size.0 as isize && y <= max_size.1 as isize { Some((x as usize, y as usize)) } else { None }
        },
        true => {
            let x = b.0 as isize * 2 - a.0 as isize;
            let y = b.1 as isize * 2 - a.1 as isize;
            if x >= 0 && y >= 0 && x <= max_size.0 as isize && y <= max_size.1 as isize { Some((x as usize, y as usize)) } else { None }
        },
    }
}

fn add_recurring_antinodes(coordinates: &Vec<(usize, usize)>, max_size: (usize, usize)) -> Vec<(usize, usize)> {
    let mut antinodes = Vec::new();

    for combination in coordinates.iter().combinations(2) {
        let mut combination_a = *combination[0];
        let mut combination_b = *combination[1];
        
        while let Some(antinode) = get_next_antinode(combination_a, combination_b, max_size, false) {
            antinodes.push(antinode);
            combination_b = combination_a;
            combination_a = antinode;
        }

        combination_a = *combination[0];
        combination_b = *combination[1];

        while let Some(antinode) = get_next_antinode(combination_a, combination_b, max_size, true) {
            antinodes.push(antinode);
            combination_a = combination_b;
            combination_b = antinode;
        }
    }
    antinodes
}

fn part_one(antennas: &HashMap<char, Vec<(usize, usize)>>, max_size: (usize, usize)) -> usize {
    let mut distinct_antinodes = HashSet::new();
    for (_, v) in antennas {
        let antinodes = add_antinodes(&v, max_size);
        for val in antinodes {
            distinct_antinodes.insert(val);
        }
    }
    distinct_antinodes.len()
}

fn part_two(antennas: &HashMap<char, Vec<(usize, usize)>>, max_size: (usize, usize)) -> usize {
    let mut distinct_antinodes = HashSet::new();
    for (_, v) in antennas {
        let antinodes = add_recurring_antinodes(&v, max_size);
        for val in antinodes {
            distinct_antinodes.insert(val);
        }

        for location in v {
            distinct_antinodes.insert(*location);
        }
    }
    distinct_antinodes.len()
}

fn main() {
    let input = std::fs::read_to_string("inputs/day8.txt").unwrap();
    let (antennas, max_size) = parse_input(&input);

    let part_one = part_one(&antennas, max_size);
    let part_two = part_two(&antennas, max_size);

    println!("Part 1: {} Part 2: {}", part_one, part_two);
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    const INPUT2: &str = "T....#....
...T......
.T....#...
.........#
..#.......
..........
...#......
..........
....#.....
..........";

    #[test]
    fn test_part_one() {
        let (antennas, max_size) = parse_input(INPUT);
        let part_one = part_one(&antennas, max_size);
        assert_eq!(part_one, 14);
    }

    #[test]
    fn test_part_two() {
        let (antennas, max_size) = parse_input(INPUT);
        let part_one = part_two(&antennas, max_size);
        assert_eq!(part_one, 34);
    }
}