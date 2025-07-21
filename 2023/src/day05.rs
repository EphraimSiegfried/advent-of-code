use regex::Regex;

type Part = Vec<[usize; 3]>;

pub fn part1(input: &str) -> String {
    let (seeds, parts) = parse_input(input);
    get_min_location_number(seeds, parts).to_string()
}

pub fn part2(input: &str) -> String {
    let (seed_ranges, parts) = parse_input(input);
    let mut seeds: Vec<usize> = Vec::new();
    let mut iter = seed_ranges.iter();
    while let (Some(start), Some(len)) = (iter.next(), iter.next()) {
        seeds.extend(*start..(start + len));
    }
    get_min_location_number(seeds, parts).to_string()
}

fn get_min_location_number(seeds: Vec<usize>, parts: Vec<Part>) -> usize {
    let mut location_numbers: Vec<usize> = Vec::new();
    for seed in seeds {
        let mut current_value = seed;
        for part in &parts {
            current_value = map_part(part, current_value);
        }
        location_numbers.push(current_value);
    }

    *location_numbers.iter().min().unwrap()
}

fn map_part(part: &Part, input: usize) -> usize {
    for [dest_start, src_start, len] in part {
        if *src_start <= input && input < src_start + len {
            let offset = input - src_start;
            return dest_start + offset;
        }
    }
    input
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<Part>) {
    let mut parts: Vec<Part> = Vec::new();
    let mut str_parts = input.split(':');
    let seeds = str_parts
        .nth(1)
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    for str_part in str_parts {
        let part = extract_numbers(str_part);
        if !part.is_empty() {
            parts.push(part);
        }
    }
    (seeds, parts)
}

fn extract_numbers(input: &str) -> Part {
    let mut numbers: Part = Vec::new();
    let re = Regex::new(r"(\d+) (\d+) (\d+)").unwrap();
    for line in input.lines() {
        if let Some(caps) = re.captures(line) {
            let arr_nums: [usize; 3] = [
                caps[1].parse().unwrap(),
                caps[2].parse().unwrap(),
                caps[3].parse().unwrap(),
            ];
            numbers.push(arr_nums);
        }
    }
    numbers
}
