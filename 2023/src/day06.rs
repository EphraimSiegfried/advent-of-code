use std::usize;

use itertools::Itertools;

pub fn part1(input: &str) -> String {
    let races = parse_input(input);
    races
        .iter()
        .map(|(time, distance)| num_record_beats(*time, *distance))
        .product::<i64>()
        .to_string()
}
pub fn part2(input: &str) -> String {
    let input = parse_input(input);
    let time: usize = input
        .iter()
        .map(|(t, _)| t.to_string())
        .join("")
        .parse()
        .unwrap();
    let dist: usize = input.iter().map(|(_, d)| d).join("").parse().unwrap();
    num_record_beats(time, dist).to_string()
}

fn num_record_beats(time: usize, distance: usize) -> i64 {
    let lb = lower_bound(time as f64, distance as f64);
    let ub = upper_bound(time as f64, distance as f64);
    ub + 1 - lb
}

fn lower_bound(n: f64, b: f64) -> i64 {
    ((0.5 * (n - (n * n - 4.0 * b).sqrt())) + 1.0).floor() as i64
}

fn upper_bound(n: f64, b: f64) -> i64 {
    ((0.5 * (n + (n * n - 4.0 * b).sqrt())) - 1.0).ceil() as i64
}

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    let nums: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.split(':')
                .last()
                .unwrap()
                .trim()
                .split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect()
        })
        .collect();
    nums[0]
        .iter()
        .zip(nums[1].iter())
        .map(|(&a, &b)| (a, b))
        .collect()
}
