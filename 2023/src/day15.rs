use std::collections::HashMap;

type Boxes = HashMap<u32, Vec<(String, u32)>>;

pub fn part1(input: &str) -> String {
    parse(input)
        .iter()
        .map(|e| hash(e))
        .sum::<u32>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let mut boxes = HashMap::new();
    parse(input).iter().for_each(|e| process(*e, &mut boxes));
    focusing_power(&boxes).to_string()
}

fn parse(input: &str) -> Vec<&str> {
    input
        .split(|k| k == ',' || k == '\n')
        .filter(|e| !e.is_empty())
        .collect()
}

fn hash(s: &str) -> u32 {
    let mut result = 0;
    for c in s.chars() {
        result += c as u32;
        result *= 17;
        result %= 256;
    }
    result as u32
}

fn process(step: &str, boxes: &mut Boxes) {
    if step.contains('-') {
        let mut split = step.split('-');
        let label = split.next().unwrap();
        let box_num = hash(label);
        if let Some(lenses) = boxes.get_mut(&box_num) {
            lenses.retain(|(l, _)| *l != label);
        }
    } else {
        let mut split = step.split('=');
        let label = split.next().unwrap().to_string();

        let focal_length = split.next().unwrap().parse::<u32>().unwrap();
        let box_num = hash(&label);

        let new_lens = (label, focal_length);
        if let Some(lenses) = boxes.get_mut(&box_num) {
            if let Some(pos) = lenses.iter().position(|(l, _)| *l == new_lens.0) {
                lenses[pos] = new_lens;
            } else {
                lenses.push(new_lens);
            }
        } else {
            let mut lenses = Vec::new();
            lenses.push(new_lens);
            boxes.insert(box_num, lenses);
        }
    }
}

fn focusing_power(boxes: &Boxes) -> u32 {
    boxes
        .iter()
        .map(|(box_num, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(i, (_, focal_length))| (box_num + 1) * (i as u32 + 1) * focal_length)
                .sum::<u32>()
        })
        .sum()
}
