use std::cmp::max;

pub fn part1(input: &str) -> String {
    input
        .lines()
        .enumerate()
        .map(|(id, line)| {
            let split = line.split(':');
            let records = split.last().unwrap().split(';');
            for record in records {
                let samples = record.split(',');
                for sample in samples {
                    let mut s = sample.split(' ');
                    let num_cubes: u32 = s.nth(1).unwrap().parse().unwrap();
                    let cube_color = s.last().unwrap();
                    let possible_game = match cube_color {
                        "red" => num_cubes <= 12,
                        "green" => num_cubes <= 13,
                        "blue" => num_cubes <= 14,
                        _ => panic!("Unexpected cube color"),
                    };
                    if !possible_game {
                        return 0;
                    }
                }
            }
            id + 1
        })
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let split = line.split(':');
            let records = split.last().unwrap().split(';');
            let mut max_red = 0;
            let mut max_green = 0;
            let mut max_blue = 0;

            for record in records {
                let samples = record.split(',');
                for sample in samples {
                    let mut s = sample.split(' ');
                    let num_cubes: u32 = s.nth(1).unwrap().parse().unwrap();
                    let cube_color = s.last().unwrap();
                    match cube_color {
                        "red" => max_red = max(num_cubes, max_red),
                        "green" => max_green = max(num_cubes, max_green),
                        "blue" => max_blue = max(num_cubes, max_blue),
                        _ => panic!("Unexpected cube color"),
                    };
                }
            }
            max_red * max_green * max_blue
        })
        .sum::<u32>()
        .to_string()
}
