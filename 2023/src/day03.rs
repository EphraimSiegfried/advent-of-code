use itertools::enumerate;

type Grid = Vec<Vec<char>>;

pub fn part1(input: &str) -> String {
    let mut valid_nums: Vec<u32> = Vec::new();
    let grid = get_grid(input);
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    let mut row_idx = 0;

    while row_idx < num_rows{
        let mut col_idx = 0;
        while col_idx < num_cols {
            if !grid[row_idx][col_idx].is_numeric(){
                col_idx += 1;
                continue
            }
            let mut has_surrounding_symbol = false;
            let mut str_num = String::new();
            while grid.get(row_idx).unwrap().get(col_idx).is_some() && grid[row_idx][col_idx].is_numeric() {
                str_num.push(grid[row_idx][col_idx]);
                has_surrounding_symbol |= surrounding_has(&grid, row_idx, col_idx, is_symbol) > 0;
                col_idx += 1;
            }
            if has_surrounding_symbol {
                valid_nums.push(str_num.parse().unwrap());
            }

            
        }
        row_idx += 1;
    }
    valid_nums.iter().sum::<u32>().to_string()
}

pub fn is_symbol(c: char) -> bool {
    !c.is_numeric() && c != '.'
}

pub fn part2(input: &str) -> String {
    let grid = get_grid(input);
    let mut sum = 0;
    
    for (row_idx, row) in enumerate(&grid) {
        for (col_idx, &ch) in enumerate(row) {
            if is_symbol(ch) {
                let mut exclude: Vec<(usize, usize)> = Vec::new();
                let mut nums: Vec<usize> = Vec::new();
                for i in -1isize..=1 {
                    for j in -1isize..=1 {
                        let x = row_idx as isize + i;
                        let y = col_idx as isize + j;
                        if !in_bounds(&grid, x, y) {continue};
                        let x = x as usize;
                        let y = y as usize;
                        if !exclude.contains(&(x, y)) && grid[x][y].is_numeric() {
                            let (num , mut ex)= extract_number(&grid, x, y);
                            nums.push(num);
                            exclude.append(&mut ex);
                        }
                    }
                }

                if nums.len() == 2 {
                    sum += nums.iter().product::<usize>();
                    dbg!(nums);
                }

            }

        }
    }
    sum.to_string()

}

fn surrounding_has<F>(grid: &Grid, row: usize, column: usize, pred: F) -> u32
    where 
        F: Fn(char) -> bool {

    let mut sum = 0;
    for i in -1isize..=1 {
        for j in -1isize..=1 {
            let x = row as isize + i;
            let y = column as isize + j;
            if in_bounds(&grid, x, y) && pred(grid[x as usize][y as usize]) {
                sum += 1
            }
        }
    }
    sum
}


fn in_bounds(grid: &Grid, row_idx: isize, col_idx: isize) -> bool {
    let num_rows = grid.len() as isize;
    let num_cols = grid[0].len() as isize;
    0 <= col_idx && col_idx <= num_cols && 0 <= row_idx && row_idx < num_rows
}

fn get_grid(input: &str) -> Grid {
    let mut grid: Grid = Vec::new();
    for line in input.lines() {
        let chars_in_line: Vec<char> = line.chars().collect();
        grid.push(chars_in_line);
    }
    grid
}

fn extract_number(grid: &Grid, row_idx: usize, col_idx: usize) -> (usize, Vec<(usize, usize)>) {
    let mut str_num = String::new();
    let mut exclude: Vec<(usize, usize)> = Vec::new();
    let mut i = col_idx;
    while i < grid[0].len() && grid[row_idx][i].is_numeric() {
        str_num.push(grid[row_idx][i]);
        exclude.push((row_idx, i));
        i += 1;
    }

    if col_idx as isize - 1 >= 0 {
        let mut j: isize = col_idx as isize - 1;
        while j >= 0 && grid[row_idx][j as usize].is_numeric() {
            str_num.insert(0, grid[row_idx][j as usize]);
            exclude.push((row_idx, j as usize));
            j -= 1;
        }
    }

    (str_num.parse().unwrap(), exclude)
}
