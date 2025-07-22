use itertools::Itertools;

type Item<'a> = (&'a str, usize);
pub fn part1(input: &str) -> String {
    let items = parse_input(input);
    total_winnings(items, hand_strength, card_strength).to_string()
}

pub fn part2(input: &str) -> String {
    let items = parse_input(input);
    total_winnings(items, hand_strength_with_joker, card_strength_with_joker).to_string()
}

fn total_winnings<F, R>(mut items: Vec<Item>, hand_strength_fn: F, card_strength_fn: R) -> usize
where
    F: Fn(&str) -> usize,
    R: Fn(char) -> usize,
{
    items.sort_by(|a, b| {
        let a_strength = hand_strength_fn(a.0);
        let b_strength = hand_strength_fn(b.0);
        a_strength.cmp(&b_strength).then_with(|| {
            for (card_a, card_b) in a.0.chars().zip(b.0.chars()) {
                let cmp = card_strength_fn(card_a).cmp(&card_strength_fn(card_b));
                if cmp != std::cmp::Ordering::Equal {
                    return cmp;
                }
            }
            std::cmp::Ordering::Equal
        })
    });
    dbg!(&items);
    items
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| bid * (i + 1))
        .sum::<usize>()
}

fn card_strength(card: char) -> usize {
    "23456789TJQKA".find(card).unwrap()
}

fn card_strength_with_joker(card: char) -> usize {
    "J23456789TQKA".find(card).unwrap()
}

fn hand_strength_with_joker(hand: &str) -> usize {
    let counts = hand.chars().counts();
    let num_unique = counts.len();
    let Some(num_jokers) = counts.get(&'J') else {
        return hand_strength(hand);
    };
    if num_unique == 2 {
        return 6;
    }
    if *num_jokers == 5 || *num_jokers == 4 {
        return 6;
    } else if *num_jokers == 3 {
        return 5;
    } else if *num_jokers == 2 {
        if num_unique == 3 {
            return 5;
        } else {
            return 3;
        }
    } else {
        counts
            .keys()
            .filter_map(|&c| {
                if c == 'J' {
                    None
                } else {
                    Some(hand_strength(
                        hand.replace('J', c.to_string().as_str()).as_str(),
                    ))
                }
            })
            .max()
            .unwrap()
    }
}

fn hand_strength(hand: &str) -> usize {
    let counts = hand.chars().counts();
    let num_unique = counts.len();
    let mut values: Vec<usize> = counts.values().copied().collect();
    values.sort_unstable();

    if num_unique == 1 {
        return 6; // five of a kind
    } else if num_unique == 2 {
        if values.contains(&4) {
            return 5; // four of a kind
        } else {
            return 4; // full house
        }
    } else if num_unique == 3 {
        if values.contains(&3) {
            return 3; // three of a kind
        } else {
            return 2; // two pair
        }
    } else if num_unique == 4 {
        return 1; // one pair
    } else {
        return 0; // high card
    }
}

fn parse_input(input: &str) -> Vec<Item> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            let cards = split.next().unwrap();
            let bid = split.last().unwrap().parse::<usize>().unwrap();
            (cards, bid)
        })
        .collect()
}
