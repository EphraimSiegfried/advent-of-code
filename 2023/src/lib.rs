pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;

pub fn solve(day: u8, part: u8, input: &str) -> String {
    match day {
        1 => match part {
            1 => day01::part1(input),
            2 => day01::part2(input),
            _ => "Invalid part".to_string(),
        },
        2 => match part {
            1 => day02::part1(input),
            2 => day02::part2(input),
            _ => "Invalid part".to_string(),
        },
        3 => match part {
            1 => day03::part1(input),
            2 => day03::part2(input),
            _ => "Invalid part".to_string(),
        },
        4 => match part {
            1 => day04::part1(input),
            2 => day04::part2(input),
            _ => "Invalid part".to_string(),
        },
        5 => match part {
            1 => day05::part1(input),
            2 => day05::part2(input),
            _ => "Invalid part".to_string(),
        },
        6 => match part {
            1 => day06::part1(input),
            2 => day06::part2(input),
            _ => "Invalid part".to_string(),
        },
        7 => match part {
            1 => day07::part1(input),
            2 => day07::part2(input),
            _ => "Invalid part".to_string(),
        },
        _ => "Day not yet implemented".to_string(),
    }
}
