use crate::util::grid::Grid;

pub fn part1(input: &str) -> String {
    let grids = input.split("\n\n").map(|p| Grid::parse(p));
    grids
        .map(|grid| {
            if let Some(num_cols) = solve_cols(&grid, None) {
                return 1 + num_cols;
            }
            if let Some(num_rows) = solve_rows(&grid, None) {
                return 100 * (1 + num_rows);
            }
            grid.print();
            panic!("No reflection found");
        })
        .sum::<usize>()
        .to_string()
}
pub fn part2(input: &str) -> String {
    let grids = input.split("\n\n").map(|p| Grid::parse(p));
    grids
        .map(|grid| {
            let candidates = candidate_grids(&grid);
            let num_cols_org = solve_cols(&grid, None);
            let num_rows_org = solve_rows(&grid, None);
            grid.print();
            for candidate in &candidates {
                print!("\n");
                candidate.print();
                if let Some(num_cols_new) = solve_cols(&candidate, num_cols_org) {
                    return dbg!(num_cols_new) + 1;
                }
                if let Some(num_rows_new) = solve_rows(&candidate, num_rows_org) {
                    return 100 * (dbg!(num_rows_new) + 1);
                }
            }
            panic!("no reflection found")
        })
        .sum::<usize>()
        .to_string()
}

fn solve_cols(grid: &Grid, exclude: Option<usize>) -> Option<usize> {
    find_reflection(
        &identical_pairs(grid.columns()),
        grid.num_columns(),
        exclude,
    )
}

fn solve_rows(grid: &Grid, exclude: Option<usize>) -> Option<usize> {
    find_reflection(&identical_pairs(grid.rows()), grid.num_rows(), exclude)
}

fn candidate_grids(grid: &Grid) -> Vec<Grid> {
    // ugly as fuck but whatever
    let mut grids = Vec::new();
    for (i, r1) in grid.rows().enumerate() {
        let row1_string = r1.collect::<String>();
        for (j, r2) in grid.rows().enumerate() {
            if i > j || (j - i) % 2 == 0 {
                continue;
            }
            let row2_string = r2.collect::<String>();
            if let Some(idx) = differ_by_one(row1_string.as_str(), row2_string.as_str()) {
                for row in [i, j] {
                    let mut new_grid = grid.clone();
                    let symbol = if grid.at(row, idx).unwrap() == '.' {
                        '#'
                    } else {
                        '.'
                    };
                    new_grid.set(row, idx, symbol);
                    grids.push(new_grid);
                }
            }
        }
    }
    for (i, r1) in grid.columns().enumerate() {
        let col1_string = r1.collect::<String>();
        for (j, r2) in grid.columns().enumerate() {
            if i > j || (j - i) % 2 == 0 {
                continue;
            }
            let col2_string = r2.collect::<String>();
            if let Some(idx) = differ_by_one(col1_string.as_str(), col2_string.as_str()) {
                for col in [i, j] {
                    let mut new_grid = grid.clone();
                    let symbol = if grid.at(idx, col).unwrap() == '.' {
                        '#'
                    } else {
                        '.'
                    };
                    new_grid.set(idx, col, symbol);
                    grids.push(new_grid);
                }
            }
        }
    }
    grids
}

fn differ_by_one(a: &str, b: &str) -> Option<usize> {
    let mut diff_index = None;
    for (i, (c1, c2)) in a.chars().zip(b.chars()).enumerate() {
        if c1 != c2 {
            if diff_index.is_some() {
                return None;
            }
            diff_index = Some(i);
        }
    }

    diff_index
}

fn identical_pairs<'a, Outer, Inner>(vecs: Outer) -> Vec<(usize, usize)>
where
    Outer: Iterator<Item = Inner>,
    Inner: IntoIterator<Item = &'a char>,
{
    let mut pairs: Vec<(usize, usize)> = Vec::new();
    let mut enum_vecs: Vec<(usize, Vec<char>)> = vecs
        .enumerate()
        .map(|(i, r)| (i, r.into_iter().copied().collect::<Vec<char>>()))
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

fn find_reflection(
    pairs: &[(usize, usize)],
    limit: usize,
    exclude: Option<usize>,
) -> Option<usize> {
    let candidates = pairs.iter().filter(|(x, y)| x.abs_diff(*y) == 1);
    for candidate in candidates {
        let (x_start, y_start) = candidate;
        let mut x_current = *x_start;
        let mut y_current = *y_start;
        while x_current > 0 && pairs.contains(&(x_current - 1, y_current + 1)) {
            x_current -= 1;
            y_current += 1;
        }
        if (x_current == 0 || y_current == limit - 1)
            && (exclude.is_none() || exclude.unwrap() != *x_start)
        {
            return Some(*x_start);
        }
    }
    None
}
