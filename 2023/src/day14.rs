use crate::util::grid::Grid;
pub fn part1(input: &str) -> String {
    let grid = Grid::parse(input);
    let num_rows = grid.num_rows();
    grid.columns()
        .map(|col| {
            let mut current_cube_idx = 0;
            let mut num_round = 0;
            let mut load = 0;
            for (i, &c) in col.enumerate() {
                if c == 'O' {
                    num_round += 1;
                }
                if c == '#' || i == num_rows - 1 {
                    for j in 0..num_round {
                        load += num_rows - current_cube_idx - j
                    }
                    current_cube_idx = i + 1;
                    num_round = 0;
                }
            }
            load
        })
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let grid = Grid::parse(input);
    roll(&grid);

    print!("\n");
    input.to_string()
}

fn roll(grid: &Grid) {
    let new_grid = grid.rows().map(|col| {
        let l = col.collect::<Vec<&char>>();
        roll_line(&l)
    });
    print!("\n");
    for row in new_grid {
        let s: String = row.into_iter().collect();
        println!("{}", s);
    }
}

fn roll_line(line: &[&char]) -> Vec<char> {
    let mut result = vec!['.'; line.len()];
    let mut last_cube_idx = 0;
    for (i, &c) in line.iter().enumerate() {
        if *c == '#' {
            result[i] = '#';
            last_cube_idx = i + 1;
        } else if *c == 'O' {
            result[last_cube_idx] = 'O';
            last_cube_idx += 1;
        }
    }
    result
}
