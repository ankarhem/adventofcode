#[derive(Debug)]
pub struct Grid<T> {
    data: Vec<Vec<T>>,
    width: usize,
    height: usize,
}

impl<T: Clone> Grid<T> {
    pub fn new(data: Vec<Vec<T>>) -> Self {
        let height = data.len();
        let width = if height > 0 { data[0].len() } else { 0 };
        Grid {
            data,
            width,
            height,
        }
    }

    pub fn rows(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        self.data.iter().map(|row| row.iter())
    }

    pub fn columns(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        (0..self.width).map(move |x| self.data.iter().map(move |row| &row[x]))
    }

    pub fn diagonals(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        let width = self.width;
        let height = self.height;

        (0..width + height - 1 + (width + height - 1)).map(move |i| {
            let is_anti = i >= width + height - 1;
            let i = if is_anti { i - (width + height - 1) } else { i };

            (0..height)
                .filter(move |&y| {
                    if !is_anti {
                        let x = i as isize - y as isize;
                        x >= 0 && x < width as isize
                    } else {
                        let x = y as isize + i as isize - (height as isize - 1);
                        x >= 0 && x < width as isize
                    }
                })
                .map(move |y| {
                    if !is_anti {
                        &self.data[y][i - y]
                    } else {
                        &self.data[y][y + i - (height - 1)]
                    }
                })
        })
    }

    pub fn windows(&self, w_width: usize, w_height: usize) -> WindowIter<T> {
        WindowIter {
            grid: self,
            w_width,
            w_height,
            current_x: 0,
            current_y: 0,
        }
    }
}

pub struct WindowIter<'a, T> {
    grid: &'a Grid<T>,
    w_width: usize,
    w_height: usize,
    current_x: usize,
    current_y: usize,
}

impl<'a, T: Clone> Iterator for WindowIter<'a, T> {
    type Item = Vec<Vec<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_y + self.w_height > self.grid.height {
            return None;
        }

        if self.current_x + self.w_width > self.grid.width {
            self.current_x = 0;
            self.current_y += 1;
            return self.next();
        }

        let mut window_data = Vec::with_capacity(self.w_height);
        for y in self.current_y..self.current_y + self.w_height {
            let row = self.grid.data[y][self.current_x..self.current_x + self.w_width].to_vec();
            window_data.push(row);
        }

        self.current_x += 1;
        Some(window_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_grid() -> Grid<i32> {
        let data = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]; // 3x3 grid
        Grid::new(data)
    }

    #[test]
    fn test_new() {
        let grid = create_test_grid();
        assert_eq!(grid.width, 3);
        assert_eq!(grid.height, 3);
        assert_eq!(grid.data, vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
    }

    #[test]
    fn test_rows() {
        let grid = create_test_grid();
        let rows: Vec<Vec<&i32>> = grid.rows().map(|row| row.collect()).collect();
        assert_eq!(
            rows,
            vec![vec![&1, &2, &3], vec![&4, &5, &6], vec![&7, &8, &9]]
        );
    }

    #[test]
    fn test_columns() {
        let grid = create_test_grid();
        let columns: Vec<Vec<&i32>> = grid.columns().map(|col| col.collect()).collect();
        assert_eq!(
            columns,
            vec![vec![&1, &4, &7], vec![&2, &5, &8], vec![&3, &6, &9]]
        );
    }

    #[test]
    fn test_windows() {
        let grid = create_test_grid();
        let windows: Vec<Vec<Vec<i32>>> = grid.windows(2, 2).collect();
        assert_eq!(
            windows,
            vec![
                vec![vec![1, 2], vec![4, 5]],
                vec![vec![2, 3], vec![5, 6]],
                vec![vec![4, 5], vec![7, 8]],
                vec![vec![5, 6], vec![8, 9]]
            ]
        );
    }

    #[test]
    fn test_empty_grid() {
        let grid: Grid<i32> = Grid::new(vec![]);
        assert_eq!(grid.width, 0);
        assert_eq!(grid.height, 0);
    }

    #[test]
    fn test_windows_edge_cases() {
        let grid = create_test_grid();
        let no_windows: Vec<Vec<Vec<i32>>> = grid.windows(4, 4).collect();
        assert!(no_windows.is_empty());
    }

    #[test]
    fn test_diagonals() {
        let grid = create_test_grid();
        let diagonals: Vec<Vec<&i32>> = grid.diagonals().map(|d| d.collect()).collect();
        assert_eq!(
            diagonals,
            vec![
                vec![&1],
                vec![&2, &4],
                vec![&3, &5, &7],
                vec![&6, &8],
                vec![&9],
                //  should also include the diagonals from the other side
                vec![&7],
                vec![&4, &8],
                vec![&1, &5, &9],
                vec![&2, &6],
                vec![&3],
            ]
        );
    }
}