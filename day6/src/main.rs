use std::collections::HashSet;

#[derive(Debug)]
struct Guard {
    location: (usize, usize),
    direction: Direction,
    visited: HashSet<(usize, usize)>,
    visited_with_dir: Vec<(Direction, usize, usize)>,
    num_loops: u32,
}

impl Guard {
    fn from(location: (usize, usize), direction: Direction) -> Self {
        let mut visited = HashSet::new();
        visited.insert(location);

        let mut visited_with_dir = Vec::new();
        visited_with_dir.push((direction.clone(), location.0, location.1));

        Guard { location, direction, visited, visited_with_dir, num_loops: 0}
    }

    fn hit(location: &(usize, usize), obstacles: &Obstacles) -> bool {
        obstacles.locations.contains(location)
    }

    fn move_guard(&mut self, obstacles: &Obstacles) {
        match self.direction {
            Direction::Up => {
                let new_location = (self.location.0 - 1, self.location.1);
                if Self::hit(&new_location, obstacles) {
                    self.turn();
                } else {
                    self.location = new_location;
                }
            },
            Direction::Down => {
                let new_location = (self.location.0 + 1, self.location.1);
                if Self::hit(&new_location, obstacles) {
                    self.turn();
                } else {
                    self.location = new_location;
                }
            }
            Direction::Left => {
                let new_location = (self.location.0, self.location.1 - 1);
                if Self::hit(&new_location, obstacles) {
                    self.turn();
                } else {
                    self.location = new_location;
                }
            },
            Direction::Right => {
                let new_location = (self.location.0, self.location.1 + 1);
                if Self::hit(&new_location, obstacles) {
                    self.turn();
                } else {
                    self.location = new_location;
                }
            },
        }
        self.visited.insert(self.location);
        self.visited_with_dir.push((self.direction.clone(), self.location.0, self.location.1));
    }

    fn turn(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn in_loop(&self) -> bool {
        let set: HashSet<_> = self.visited_with_dir.iter().collect();
        if self.visited_with_dir.len() != set.len() {
            return true
        }
        false
    }

    fn add_loop(&mut self) {
        self.num_loops += 1;
    }

    fn reset(&mut self, location: (usize, usize), direction: Direction) {
        self.location = location;
        
        let mut visited = HashSet::new();
        visited.insert(location);
        self.visited = visited;

        let mut visited_with_dir = Vec::new();
        visited_with_dir.push((direction.clone(), location.0, location.1));
        self.visited_with_dir = visited_with_dir;

        self.direction = direction;
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Obstacles {
    locations: Vec<(usize, usize)>,
}

impl Obstacles {
    fn new() -> Self {
        Obstacles {
            locations: Vec::new(),
        }
    }

    fn add(&mut self, location: (usize, usize)) {
        self.locations.push(location);
    }

    fn remove_last(&mut self) {
        self.locations.pop();
    }
}

fn parse_input(input: &str) -> (Guard, Obstacles, (usize, usize)) {
    let mut obstacles = Obstacles::new();
    let mut guard: Guard = Guard::from((0, 0), Direction::Up);

    let map = input
        .lines()
        .enumerate()
        .map(|(i, x)| x.chars().enumerate().map(move |(j, y)| (i, j, y)).collect::<Vec<_>>()).collect::<Vec<_>>();

    for line in map.iter() {
        for (i, j, c) in line {
            match c {
                '^' => guard = Guard::from((*i, *j), Direction::Up),
                '#' => obstacles.add((*i, *j)),
                _ => (),
            }
        }
    }
    (guard, obstacles, (map.len(), map[0].len()))
}

fn process(guard: &mut Guard, obstacles: &Obstacles, bounds: &(usize, usize)) -> usize {
    while guard.location.0 < bounds.0 - 1 && guard.location.0 > 0 && guard.location.1 < bounds.1 - 1 && guard.location.1 > 0 {
        if guard.in_loop() {
            guard.add_loop();
            return 0
        }
        guard.move_guard(obstacles);
    }
    guard.visited.len()
}

fn main() {
    let input = std::fs::read_to_string("./inputs/day6.txt").unwrap();
    let (mut guard, mut obstacles, bounds) = parse_input(&input);
    let init_location = guard.location;
    let init_direction = guard.direction.clone();

    let part_one = process(&mut guard, &obstacles, &bounds);
    
    let visited: Vec<(usize, usize)> = guard.visited.iter().map(|x| x.clone()).collect();

    // this is very slow a brute force solution, but it works...
    // TODO: make this better
    visited.iter().for_each(|new_obstacle| {
        if new_obstacle != &init_location {
            guard.reset(init_location, init_direction);
            obstacles.add(*new_obstacle);
            process(&mut guard, &obstacles, &bounds);
            obstacles.remove_last();
        }
    });
    let part_two = guard.num_loops;

    println!("Part 1: {} Part 2: {}", part_one, part_two);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part_one() {
        let (mut guard, obstacles, bounds) = parse_input(INPUT);
        let part_one = process(&mut guard, &obstacles, &bounds);
        assert_eq!(part_one, 41);
    }

    #[test]
    fn test_part_two() {
        let (mut guard, mut obstacles, bounds) = parse_input(INPUT);
        let init_location = guard.location;
        let init_direction = guard.direction.clone();
    
        let _ = process(&mut guard, &obstacles, &bounds);
        
        let visited: Vec<(usize, usize)> = guard.visited.iter().map(|x| x.clone()).collect();

        visited.iter().for_each(|new_obstacle| {
            if new_obstacle != &init_location {
                guard.reset(init_location, init_direction);
                obstacles.add(*new_obstacle);
                process(&mut guard, &obstacles, &bounds);
                obstacles.remove_last();
            }
        });
        let part_two = guard.num_loops;
        assert_eq!(part_two, 6)
    }
}
