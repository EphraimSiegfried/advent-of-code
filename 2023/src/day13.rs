use crate::util::grid::Grid;

pub fn part1(input: &str) -> String {
    let grids = input.split('\n').map(|p| Grid::parse(p));
    grids.map(|g| g);
    input.to_string()
}
pub fn part2(input: &str) -> String {
    input.to_string()
}

fn identical_pairs(vecs: &Vec<&Vec<char>>) -> Vec<(usize, usize)> {
    let mut pairs: Vec<(usize, usize)> = Vec::new();
    let mut enum_vecs: Vec<(usize, Vec<char>)> = vecs
        .iter()
        .enumerate()
        .map(|(i, r)| (i, (*r).clone()))
        .collect();

    while let Some((j, y)) = enum_vecs.pop() {
        for (i, x) in &enum_vecs {
            if *x == y {
                pairs.push((*i, j));
            }
        }
    }
    pairs
}

fn find_reflection(pairs: &Vec<(usize, usize)>) -> usize {
    let (mut x, mut &y) = pairs.iter().find(|&(x, y)| x.abs_diff(*y) == 1).unwrap();
    while pairs.contains(&(x, y)) {
        x -= 1;
        y += 1;
    }
    x
}
