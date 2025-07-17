pub mod day01;
pub mod day02;

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
        }
        _ => "Day not yet implemented".to_string(),
    }
}
