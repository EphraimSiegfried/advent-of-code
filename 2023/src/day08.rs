use regex::Regex;
use std::collections::HashMap;

pub fn part1(input: &str) -> String {
    let (instr, map) = parse_input(input);
    let instr: Vec<char> = instr.chars().collect();
    let mut current = "AAA";
    let mut steps: usize = 0;
    while current != "ZZZ" {
        let dir = if instr[steps % instr.len()] == 'L' {
            0
        } else {
            1
        };
        current = map.get(current).unwrap()[dir];
        steps += 1;
    }
    steps.to_string()
}
pub fn part2(input: &str) -> String {
    let (instr, map) = parse_input(input);
    let instr: Vec<char> = instr.chars().collect();

    let current_nodes: Vec<&str> = map.keys().filter(|k| k.ends_with('A')).cloned().collect();
    let node_steps = current_nodes
        .iter()
        .map(|&current| {
            let mut curr = current;
            let mut steps: usize = 0;
            while !curr.ends_with('Z') {
                let dir = if instr[steps % instr.len()] == 'L' {
                    0
                } else {
                    1
                };
                curr = map.get(curr).unwrap()[dir];
                steps += 1;
            }
            steps as i64
        })
        .collect::<Vec<i64>>();
    dbg!(&node_steps);

    lcm_vec(&node_steps).to_string()
}
fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { a.abs() } else { gcd(b, a % b) }
}

fn lcm_two(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b).abs() / gcd(a, b)
    }
}

fn lcm_vec(numbers: &[i64]) -> i64 {
    if numbers.is_empty() {
        return 1;
    }
    let mut result = numbers[0];
    for &num in numbers.iter().skip(1) {
        result = lcm_two(result, num);
    }
    result
}
fn parse_input(input: &str) -> (&str, HashMap<&str, [&str; 2]>) {
    let mut map: HashMap<&str, [&str; 2]> = HashMap::new();
    let mut split = input.split("\n\n");
    let instr = split.next().unwrap();
    let re = Regex::new(r"(.{3}) = \((.{3}), (.{3})\)").unwrap();
    for (_, [from, left, right]) in re.captures_iter(split.next().unwrap()).map(|c| c.extract()) {
        map.insert(from, [left, right]);
    }
    (instr, map)
}
