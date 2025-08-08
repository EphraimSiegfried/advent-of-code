use crate::util::point::Point;

pub struct Grid {
    mat: Vec<Vec<char>>,
}

impl Grid {
    pub fn parse(input: &str) -> Grid {
        let mat = input.lines().map(|l| l.chars().collect()).collect();
        Grid { mat: mat }
    }

    pub fn at(&self, point: Point) -> Option<char> {
        let Point(row_idx, col_idx) = point;
        if !self.in_bounds(point) {
            return None;
        }
        Some(self.mat[row_idx as usize][col_idx as usize])
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

    pub fn in_bounds(&self, point: Point) -> bool {
        let Point(row_idx, col_idx) = point;
        let num_rows = self.mat.len();
        let num_cols = self.mat[0].len();
        0 <= row_idx && row_idx < num_rows as i32 && 0 <= col_idx && col_idx < num_cols as i32
    }

    pub fn print(&self) {
        for row in &self.mat {
            for ch in row {
                print!("{}", ch);
            }
            print!("\n");
        }
    }
    pub fn columns(&self) -> impl Iterator<Item = Vec<char>> + '_ {
        if self.mat.is_empty() || self.mat[0].is_empty() {
            // Return an empty iterator for an empty or malformed grid
            return Vec::new().into_iter();
        }

        let num_rows = self.mat.len();
        let num_cols = self.mat[0].len();

        (0..num_cols)
            .map(move |col_idx| {
                (0..num_rows)
                    .map(|row_idx| self.mat[row_idx][col_idx])
                    .collect::<Vec<char>>() // Collect inner iterator into a Vec<char>
            })
            .collect::<Vec<Vec<char>>>() // Collect the outer iterator into a Vec<Vec<char>>
            .into_iter() // Convert the Vec<Vec<char>> into an iterator
    }

    pub fn rows(&self) -> impl Iterator<Item = &Vec<char>> + '_ {
        self.mat.iter()
    }
}
