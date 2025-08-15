use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

use crate::util::grid::Grid;
use crate::util::point::{DOWN, LEFT, Point, RIGHT, UP};

pub fn part1(input: &str) -> String {
    let grid = Grid::parse(input);
    walk(&grid, (Point(0, 0), RIGHT)).to_string()
}
pub fn part2(input: &str) -> String {
    let grid = Grid::parse(input);
    let num_rows = grid.num_rows();
    let num_cols = grid.num_columns();
    let mut starts = Vec::new();
    for i in 0..num_rows {
        starts.push((Point(i as i32, 0), RIGHT));
        starts.push((Point(i as i32, (num_cols - 1) as i32), LEFT));
    }
    for i in 0..num_cols {
        starts.push((Point(0, i as i32), DOWN));
        starts.push((Point((num_rows - 1) as i32, i as i32), UP));
    }
    starts
        .iter()
        .map(|start| walk(&grid, *start))
        .max()
        .unwrap()
        .to_string()
}

fn walk(grid: &Grid, start: (Point, Point)) -> usize {
    let mut open = VecDeque::new();
    let mut visited = HashSet::new();
    open.push_back(start);
    while let Some((pos, dir)) = open.pop_front() {
        let Point(row_idx, col_idx) = pos;
        let Some(symbol) = grid.at(row_idx as usize, col_idx as usize) else {
            continue;
        };
        if !visited.insert((pos, dir)) {
            continue;
        };
        if symbol == '|' && (dir == LEFT || dir == RIGHT) {
            open.push_back((pos + UP, UP));
            open.push_back((pos + DOWN, DOWN));
        } else if symbol == '-' && (dir == UP || dir == DOWN) {
            open.push_back((pos + RIGHT, RIGHT));
            open.push_back((pos + LEFT, LEFT));
        } else if symbol == '\\' {
            let Point(x, y) = dir;
            let next_dir = Point(y, x);
            open.push_back((pos + next_dir, next_dir));
        } else if symbol == '/' {
            let Point(x, y) = dir;
            let next_dir = Point(-y, -x);
            open.push_back((pos + next_dir, next_dir));
        } else {
            open.push_back((pos + dir, dir));
        }
    }
    visited.iter().map(|(pos, _)| pos).unique().count()
}
