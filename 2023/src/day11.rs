use std::{result, usize};

use itertools::Itertools;
type Grid = Vec<Vec<char>>;
pub fn part1(input: &str) -> String {
    solve(input, 2)
}

pub fn part2(input: &str) -> String {
    solve(input, 1000000)
}

fn solve(input: &str, expansion_factor: usize) -> String {
    let grid: Grid = input.lines().map(|l| l.chars().collect()).collect();
    let rows_to_expand = rows_to_expand(&grid);
    let cols_to_expand = cols_to_expand(&grid);
    let galaxy_locations = find(&grid, '#');
    let pairs = galaxy_locations.iter().combinations(2);
    let distances = pairs.map(|v| {
        let (p1_row, p1_col) = v[0];
        let (p2_row, p2_col) = v[1];
        let num_expanded_rows = rows_to_expand
            .iter()
            .filter(|&row| p1_row.min(p2_row) < row && row < p1_row.max(p2_row))
            .count();
        let num_expanded_cols = cols_to_expand
            .iter()
            .filter(|&row| p1_col.min(p2_col) < row && row < p1_col.max(p2_col))
            .count();
        manhattan_distance(v[0], v[1])
            + (num_expanded_cols + num_expanded_rows) * (expansion_factor - 1)
    });
    distances.sum::<usize>().to_string()
}

fn manhattan_distance(p1: &(usize, usize), p2: &(usize, usize)) -> usize {
    (((p1.0 as i32) - (p2.0 as i32)).abs() + ((p1.1 as i32) - (p2.1 as i32)).abs()) as usize
}

fn cols_to_expand(grid: &Grid) -> Vec<usize> {
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    let mut result = Vec::new();
    for col_idx in 0..num_cols {
        let mut contains_galaxy = false;
        for row_idx in 0..num_rows {
            if grid[row_idx][col_idx] == '#' {
                contains_galaxy = true;
                break;
            }
        }
        if !contains_galaxy {
            result.push(col_idx);
        }
    }
    result
}

fn rows_to_expand(grid: &Grid) -> Vec<usize> {
    let mut result = Vec::new();
    for (i, row) in grid.iter().enumerate() {
        if !row.contains(&'#') {
            result.push(i);
        }
    }
    result
}

pub fn find(grid: &Grid, target: char) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, &element) in row.iter().enumerate() {
            if element == target {
                result.push((row_idx, col_idx));
            }
        }
    }
    result
}
