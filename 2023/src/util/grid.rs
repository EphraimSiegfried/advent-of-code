use crate::util::point::Point;

#[derive(Debug, Clone)]
pub struct Grid {
    mat: Vec<Vec<char>>,
}

impl Grid {
    pub fn parse(input: &str) -> Grid {
        let mat = input.lines().map(|l| l.chars().collect()).collect();
        Grid { mat: mat }
    }

    pub fn at(&self, row_idx: usize, col_idx: usize) -> Option<char> {
        if !self.in_bounds(row_idx, col_idx) {
            return None;
        }
        Some(self.mat[row_idx as usize][col_idx as usize])
    }

    pub fn set(&mut self, row_idx: usize, col_idx: usize, c: char) {
        self.mat[row_idx][col_idx] = c;
    }

    pub fn find(&self, target: char) -> Vec<Point> {
        let mut result = Vec::new();
        for (row_idx, row) in self.mat.iter().enumerate() {
            for (col_idx, &element) in row.iter().enumerate() {
                if element == target {
                    result.push(Point(row_idx as i32, col_idx as i32));
                }
            }
        }
        result
    }

    pub fn in_bounds(&self, row_idx: usize, col_idx: usize) -> bool {
        let num_rows = self.mat.len();
        let num_cols = self.mat[0].len();
        row_idx < num_rows && col_idx < num_cols
    }

    pub fn print(&self) {
        for row in &self.mat {
            for ch in row {
                print!("{}", ch);
            }
            print!("\n");
        }
    }
    pub fn num_columns(&self) -> usize {
        self.mat[0].len()
    }

    pub fn num_rows(&self) -> usize {
        self.mat.len()
    }

    pub fn columns(&self) -> impl Iterator<Item = impl Iterator<Item = &char>> {
        let num_cols = self.mat[0].len();
        (0..num_cols).map(move |col_idx| self.mat.iter().map(move |row| &row[col_idx]))
    }

    pub fn rows(&self) -> impl Iterator<Item = impl Iterator<Item = &char>> {
        self.mat.iter().map(|row| row.iter())
    }
}
