use std::collections::BTreeMap;

fn parse_input(input: &str) -> BTreeMap<(usize, usize), char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(x, row)| {
            row
                .chars()
                .enumerate()
                .map(move |(y, col)| ((x, y), col))
        })
        .collect()
}

fn part_one(letters: &BTreeMap<(usize, usize), char>) -> usize {
    let mut count = 0;
    let mut cur = String::new();

    // because we are looking for XMAS and SAMX, we only need to check half the 8 directions
    // to avoid double counting
    for (coordinate, letter) in letters.iter() {

        // check N
        cur.push(*letter);
        let mut new_x = coordinate.0;
        while new_x > 0 && cur.len() < 4 {
            new_x -= 1;
            cur.push(*letters.get(&(new_x, coordinate.1)).unwrap_or(&'.'));
        }
        if cur == "XMAS" || cur == "SAMX" {
            count += 1;
        }
        cur.clear();

        // check NE
        cur.push(*letter);
        let mut new_x = coordinate.0;
        let mut new_y = coordinate.1;
        while new_x > 0 && cur.len() < 4 {
            new_x -= 1;
            new_y += 1;
            cur.push(*letters.get(&(new_x, new_y)).unwrap_or(&'.'));
        }
        if cur == "XMAS" || cur == "SAMX" {
            count += 1;
        }
        cur.clear();

        // check E
        cur.push(*letter);
        let mut new_y = coordinate.1;
        while cur.len() < 4 {
            new_y += 1;
            cur.push(*letters.get(&(coordinate.0, new_y)).unwrap_or(&'.'));
        }
        if cur == "XMAS" || cur == "SAMX" {
            count += 1;
        }
        cur.clear();

        // check SE
        cur.push(*letter);
        let mut new_x = coordinate.0;
        let mut new_y = coordinate.1;
        while cur.len() < 4 {
            new_x += 1;
            new_y += 1;
            cur.push(*letters.get(&(new_x, new_y)).unwrap_or(&'.'));
        }
        if cur == "XMAS" || cur == "SAMX" {
            count += 1;
        }
        cur.clear();
    }
    count
}

fn part_two(letters: &BTreeMap<(usize, usize), char>) -> usize {
    let mut count = 0;

    for (coordinate, letter) in letters.iter() {
        if letter != &'A' || coordinate.0 == 0 || coordinate.1 == 0 {
            continue;
        }

        let nw = letters.get(&(coordinate.0 - 1, coordinate.1 - 1)).unwrap_or(&'.');
        let ne = letters.get(&(coordinate.0 - 1, coordinate.1 + 1)).unwrap_or(&'.');
        let sw = letters.get(&(coordinate.0 + 1, coordinate.1 - 1)).unwrap_or(&'.');
        let se = letters.get(&(coordinate.0 + 1, coordinate.1 + 1)).unwrap_or(&'.');

        let cross_one = match nw {
            'M' => {
                match se {
                    'S' => true,
                    _ => false,
                }
            },
            'S' => {
                match se {
                    'M' => true,
                    _ => false,
                }
            },
            _ => false,
        };

        let cross_two = match ne {
            'M' => {
                match sw {
                    'S' => true,
                    _ => false,
                }
            },
            'S' => {
                match sw {
                    'M' => true,
                    _ => false,
                }
            },
            _ => false,
        };

        if cross_one && cross_two {
            count += 1;
        }
    }
    count
}

fn main() {
    let input = std::fs::read_to_string("./inputs/day4.txt").unwrap();
    let letters = parse_input(&input);
    let part_one = part_one(&letters);
    let part_two = part_two(&letters);
    println!("part 1: {} part 2: {}", part_one, part_two);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_part_one() {
        let letters = parse_input(INPUT);
        let part_one = part_one(&letters);
        assert_eq!(part_one, 18);
    }

    #[test]
    fn test_part_two() {
        let letters = parse_input(INPUT);
        let part_two = part_two(&letters);
        assert_eq!(part_two, 9);
    }
}