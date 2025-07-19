use itertools::enumerate;


fn get_matching(input: &str) -> Vec<usize> {
    input.lines().map(|line| {
        let nums = line.split(':').last().unwrap().split('|');
        let nums: Vec<Vec<usize>> = nums
            .map(|l| {
                l.trim()
                 .split(' ')
                 .filter(|s| !s.is_empty()) 
                 .map(|n| n.parse::<usize>().unwrap())
                 .collect::<Vec<usize>>()
            }).collect();
        nums[1].iter().filter(|x| nums[0].contains(x)).collect::<Vec<&usize>>().len()
    }).collect()
}
pub fn part1(input: &str) -> String {
    get_matching(input).iter().map(|num_matches| {
        let base: usize = 2;
        if *num_matches == 0 {
            0
        } else {
            base.pow((num_matches - 1).try_into().unwrap())
        }
    }).sum::<usize>().to_string()
}

pub fn part2(input: &str) -> String {
    let num_matches = get_matching(input);
    let mut num_cards = vec![1;num_matches.len()];
    for (id, matches) in enumerate(num_matches) {
        for won_card_id in (id+1)..(id+1+matches) {
            num_cards[won_card_id] += num_cards[id]
        }
    }
    num_cards.iter().sum::<usize>().to_string()
}
