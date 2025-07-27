type Record = (String, Vec<usize>);
pub fn part1(input: &str) -> String {
    let records = parse(&input);
    records
        .iter()
        .map(|r| {
            let mut num_solutions = 0;
            solve(r.0.clone(), &r.1, &mut num_solutions);
            num_solutions
        })
        .sum::<usize>()
        .to_string()
}
pub fn part2(input: &str) -> String {
    let mut records = parse(&input);
    records = unfold(records);
    dbg!(&records);
    records
        .iter()
        .map(|r| {
            let mut num_solutions = 0;
            solve(r.0.clone(), &r.1, &mut num_solutions);
            num_solutions
        })
        .sum::<usize>()
        .to_string()
}

fn solve(field: String, sizes: &Vec<usize>, num_solutions: &mut usize) {
    if !field.contains('?') {
        if consistent(&field, &sizes) {
            *num_solutions += 1;
        }
        return;
    }
    if impossible(&field, &sizes) {
        return;
    }
    for p in ["#", "."] {
        solve(field.replacen("?", p, 1).to_string(), sizes, num_solutions);
    }
}

fn impossible(field: &String, sizes: &Vec<usize>) -> bool {
    field
        .split('?')
        .next()
        .unwrap()
        .split('.')
        .filter(|&s| !s.is_empty())
        .map(|r| r.len())
        .zip(sizes)
        .any(|(f, s)| &f > s)
}

fn consistent(field: &String, sizes: &Vec<usize>) -> bool {
    let field_lengths: Vec<usize> = field
        .split('.')
        .filter(|&s| !s.is_empty())
        .map(|r| r.len())
        .collect();

    field_lengths == *sizes
}

fn unfold(records: Vec<Record>) -> Vec<Record> {
    records
        .iter()
        .map(|r| (vec![r.0.clone(); 5].join("?").to_string(), r.1.repeat(5)))
        .collect()
}

fn parse(input: &str) -> Vec<Record> {
    input
        .lines()
        .map(|l| {
            let mut split = l.split(' ');
            let field = split.next().unwrap().to_string();
            let sizes: Vec<usize> = split
                .next()
                .unwrap()
                .split(',')
                .map(|e| e.parse::<usize>().unwrap())
                .collect();
            (field, sizes)
        })
        .collect()
}
