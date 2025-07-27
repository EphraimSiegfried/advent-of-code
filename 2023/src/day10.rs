use Direction::*;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn all_directions() -> &'static [Direction] {
        &[North, South, East, West]
    }
    fn to_coords(&self) -> (isize, isize) {
        match self {
            North => (-1, 0),
            South => (1, 0),
            East => (0, 1),
            West => (0, -1),
        }
    }
    fn opposed(&self) -> Direction {
        match self {
            North => South,
            South => North,
            West => East,
            East => West,
        }
    }
}

pub fn part1(input: &str) -> String {
    let mut grid = parse_input(input);
    let steps = walk_loop(&mut grid);
    let farthest_point = steps as f64 / 2.0;
    farthest_point.to_string()
}

pub fn part2(input: &str) -> String {
    //     let mut grid = parse_input(input);
    //     let mut open: HashSet<(isize, isize)> = find(&grid, '.').into_iter().collect();
    //     walk_loop(&mut grid);
    //     let num_enclosed_tiles = 0;
    //
    //     while !open.is_empty() {
    //         let (closed, is_nested) = batching(&grid, *open.iter().next().unwrap());
    //         open = open.difference(&closed).cloned().collect();
    //     }
    input.to_string()
}

// fn batching(grid: &[Vec<char>], start: (isize, isize)) -> (HashSet<(isize, isize)>, bool) {
//     let mut closed = HashSet::new();
//     let mut open = VecDeque::new();
//     let mut is_nested = true;
//     open.push_front(start);
//     while !open.is_empty() {
//         let next = open.pop_front().unwrap();
//         if !closed.contains(&next) {
//             closed.insert(next);
//         }
//         for (row_idx, col_idx, _) in successors(next) {
//             if !in_bounds(&grid, col_idx, row_idx) {
//                 is_nested = false;
//                 continue;
//             }
//             let cur = grid[row_idx as usize][col_idx as usize];
//             if cur != '.' {
//                 if cur != '!' {
//                     is_nested = false;
//                 }
//                 continue;
//             }
//             open.push_back((row_idx, col_idx));
//         }
//     }
//     (closed, is_nested)
// }

fn successors(pos: (isize, isize)) -> Vec<(isize, isize, Direction)> {
    let mut successors = Vec::new();
    for dir in Direction::all_directions() {
        let (row_offset, col_offset) = dir.to_coords();
        let row_idx = pos.0 as isize + row_offset;
        let col_idx = pos.1 as isize + col_offset;
        successors.push((row_idx as isize, col_idx as isize, *dir));
    }
    successors
}

fn walk_loop(grid: &mut [Vec<char>]) -> usize {
    let (mut row_idx, mut col_idx) = find(&grid, 'S')[0];
    let pipe_map = create_pipe_hashmap();
    let mut steps = 1;
    let mut pipe_type = 'S';
    let mut current_dir = West; // any

    for (row_idx, col_idx, dir) in successors((row_idx, col_idx)) {
        if in_bounds(&grid, row_idx, col_idx) {
            pipe_type = grid[row_idx as usize][col_idx as usize];
            let Some(dirs) = pipe_map.get(&pipe_type) else {
                continue;
            };
            if dirs.contains(&dir.opposed()) {
                current_dir = dir.clone();
                break;
            };
        }
    }
    grid[row_idx as usize][col_idx as usize] = '!';

    while pipe_type != 'S' {
        let dirs = pipe_map.get(&pipe_type).unwrap();
        let pos = dirs
            .iter()
            .position(|&x| x == current_dir.opposed())
            .unwrap();
        current_dir = dirs[(pos + 1) % 2];
        let (row_offset, col_offset) = current_dir.to_coords();
        row_idx = row_idx + row_offset;
        col_idx = col_idx + col_offset;
        pipe_type = grid[row_idx as usize][col_idx as usize];
        steps += 1;
        grid[row_idx as usize][col_idx as usize] = '!';
    }
    steps
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn create_pipe_hashmap() -> HashMap<char, [Direction; 2]> {
    let mut pipe_map: HashMap<char, [Direction; 2]> = HashMap::new();
    pipe_map.insert('|', [North, South]);
    pipe_map.insert('-', [East, West]);
    pipe_map.insert('L', [North, East]);
    pipe_map.insert('J', [North, West]);
    pipe_map.insert('7', [South, West]);
    pipe_map.insert('F', [South, East]);
    pipe_map
}

fn in_bounds(grid: &[Vec<char>], col_idx: isize, row_idx: isize) -> bool {
    let num_rows = grid.len() as isize;
    let num_cols = grid[0].len() as isize;
    0 <= col_idx && col_idx <= num_cols && 0 <= row_idx && row_idx < num_rows
}

fn find(grid: &[Vec<char>], target: char) -> Vec<(isize, isize)> {
    let mut result = Vec::new();
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, &element) in row.iter().enumerate() {
            if element == target {
                result.push((row_idx as isize, col_idx as isize));
            }
        }
    }
    result
}
