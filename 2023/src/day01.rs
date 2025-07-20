pub fn part1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let first_digit = line.chars().find(|c| c.is_ascii_digit()).unwrap();
            let last_digit = line.chars().rfind(|c| c.is_ascii_digit()).unwrap();
            digits_to_number(first_digit, last_digit)
        })
        .sum::<u32>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let patterns = [
        ("1", "one"),
        ("2", "two"),
        ("3", "three"),
        ("4", "four"),
        ("5", "five"),
        ("6", "six"),
        ("7", "seven"),
        ("8", "eight"),
        ("9", "nine"),
    ];

    let sum: u32 = input
        .lines()
        .map(|line| {
            let mut first_digit = ' ';
            for i in 0..line.len() {
                let sub = &line[i..];
                let mut found = false;
                for (digit, word) in patterns {
                    if sub.starts_with(word) || sub.starts_with(digit) {
                        first_digit = digit.chars().next().unwrap();
                        found = true;
                        break;
                    }
                }
                if found {
                    break;
                }
            }

            let mut last_digit = ' ';
            for i in (0..line.len()).rev() {
                let sub = &line[i..];
                let mut found = false;
                for (digit, word) in patterns {
                    if sub.starts_with(word) || sub.starts_with(digit) {
                        last_digit = digit.chars().next().unwrap();
                        found = true;
                        break;
                    }
                }
                if found {
                    break;
                }
            }

            digits_to_number(first_digit, last_digit)
        })
        .sum();

    sum.to_string()
}

fn digits_to_number(c1: char, c2: char) -> u32 {
    let mut number_str = String::new();
    number_str.push(c1);
    number_str.push(c2);
    number_str.parse::<u32>().unwrap()
}
