use aoc_2023::solve;
use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    day: u8,

    #[arg(short, long, default_value_t = 1)]
    part: u8,
}

fn main() {
    let args = Args::parse();
    let input_path = format!("input/day{:02}.txt", args.day);
    let input = fs::read_to_string(&input_path)
        .unwrap_or_else(|_| panic!("Could not read input file: {}", &input_path));

    let solution = solve(args.day, args.part, &input);
    println!("Solution for Day {}, Part {}: {}", args.day, args.part, solution);
}
