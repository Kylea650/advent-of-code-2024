fn parse_input(input: &str) -> Vec<Vec<u32>>{
    let parsed: Vec<Vec<&str>> = input
        .lines()
        .map(|x|x.split_ascii_whitespace().collect())
        .collect();

    (0..parsed[0].len())
        .map(|i| {
            let mut vec: Vec<u32> = parsed.iter()
                .map(|x| x[i].parse::<u32>().unwrap())
                .collect();
            vec.sort();
            vec
        })
        .collect()
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let parsed = parse_input(&input);
    
    let part_one = (0..parsed[0].len()).fold(0, |acc, idx| {
        acc + parsed[0][idx].abs_diff(parsed[1][idx])
    });

    let part_two = (0..parsed[0].len()).fold(0, |acc, idx| {
        let to_find = &parsed[0][idx];
        let num_found = parsed[1].iter().filter(|&x| x == to_find).count();
        acc + (to_find * num_found as u32)
    });
    
    println!("part 1: {} part 2: {}", part_one, part_two);
}
