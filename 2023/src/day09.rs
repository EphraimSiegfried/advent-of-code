use itertools::Itertools;

pub fn part1(input: &str) -> String {
    let dataset = parse_input(input);
    dataset
        .iter()
        .map(|data| extrapolate_next(data.clone()))
        .sum::<isize>()
        .to_string()
}
pub fn part2(input: &str) -> String {
    let dataset = parse_input(input);
    dataset
        .iter()
        .map(|data| extrapolate_prev(data.clone()))
        .sum::<isize>()
        .to_string()
}

fn parse_input(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|e| e.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .collect()
}

fn diff(data: Vec<isize>) -> Vec<isize> {
    data.iter().tuple_windows().map(|(a, b)| b - a).collect()
}

fn extrapolate_next(data: Vec<isize>) -> isize {
    if data.iter().all(|&e| e == 0) {
        return 0;
    }
    data.last().unwrap() + extrapolate_next(diff(data.clone()))
}

fn extrapolate_prev(data: Vec<isize>) -> isize {
    if data.iter().all(|&e| e == 0) {
        return 0;
    }
    data.first().unwrap() - extrapolate_prev(diff(data.clone()))
}
