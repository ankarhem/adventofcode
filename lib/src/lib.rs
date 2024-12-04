pub struct Grid<'a, T> {
    data: &'a [Vec<T>],
    height: usize,
    width: usize,
    current_row: usize,
    current_col: usize,
}

impl<'a, T> Grid<'a, T> {
    pub fn new(data: &'a [Vec<T>], height: usize, width: usize) -> Self {
        Grid {
            data,
            height,
            width,
            current_row: 0,
            current_col: 0,
        }
    }
}

impl<'a, T> Iterator for Grid<'a, T> {
    type Item = Vec<&'a [T]>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.is_empty()
            || self.height > self.data.len()
            || self.width > self.data[0].len()
            || self.current_row > self.data.len() - self.height {
            return None;
        }

        let window = self.data[self.current_row..self.current_row + self.height]
            .iter()
            .map(|row| &row[self.current_col..self.current_col + self.width])
            .collect();

        self.current_col += 1;
        if self.current_col > self.data[0].len() - self.width {
            self.current_col = 0;
            self.current_row += 1;
        }

        Some(window)
    }
}

pub trait Window2DIterator<T> {
    fn window2d(&self, size: usize) -> Grid<T>;
    fn window2d_rect(&self, height: usize, width: usize) -> Grid<T>;
}

impl<T> Window2DIterator<T> for [Vec<T>] {
    fn window2d(&self, size: usize) -> Grid<T> {
        Grid::new(self, size, size)
    }

    fn window2d_rect(&self, height: usize, width: usize) -> Grid<T> {
        Grid::new(self, height, width)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2x2_windows() {
        let matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];

        let windows: Vec<_> = matrix.as_slice().window2d(2).collect();

        assert_eq!(windows.len(), 4);
        assert_eq!(windows[0], vec![&[1, 2][..], &[4, 5][..]]);
        assert_eq!(windows[1], vec![&[2, 3][..], &[5, 6][..]]);
        assert_eq!(windows[2], vec![&[4, 5][..], &[7, 8][..]]);
        assert_eq!(windows[3], vec![&[5, 6][..], &[8, 9][..]]);
    }

    #[test]
    fn test_rectangular_windows() {
        let matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];

        let windows: Vec<_> = matrix.as_slice().window2d_rect(2, 3).collect();

        assert_eq!(windows.len(), 2);
        assert_eq!(windows[0], vec![&[1, 2, 3][..], &[4, 5, 6][..]]);
        assert_eq!(windows[1], vec![&[4, 5, 6][..], &[7, 8, 9][..]]);
    }

    #[test]
    fn test_single_cell_windows() {
        let matrix = vec![
            vec![1, 2],
            vec![3, 4],
        ];

        let windows: Vec<_> = matrix.as_slice().window2d(1).collect();

        assert_eq!(windows.len(), 4);
        assert_eq!(windows[0], vec![&[1][..]]);
        assert_eq!(windows[1], vec![&[2][..]]);
        assert_eq!(windows[2], vec![&[3][..]]);
        assert_eq!(windows[3], vec![&[4][..]]);
    }

    #[test]
    fn test_empty_matrix() {
        let matrix: Vec<Vec<i32>> = vec![];
        let windows: Vec<_> = matrix.as_slice().window2d(1).collect();
        assert!(windows.is_empty());
    }

    #[test]
    fn test_window_larger_than_matrix() {
        let matrix = vec![
            vec![1, 2],
            vec![3, 4],
        ];

        let windows: Vec<_> = matrix.as_slice().window2d(3).collect();
        assert!(windows.is_empty());
    }

    #[test]
    fn test_iterator_behavior() {
        let matrix = vec![
            vec![1, 2],
            vec![3, 4],
        ];

        let mut iter = matrix.as_slice().window2d(2);

        assert_eq!(iter.next(), Some(vec![&[1, 2][..], &[3, 4][..]]));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_non_square_matrix() {
        let matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
        ];

        let windows: Vec<_> = matrix.as_slice().window2d_rect(1, 2).collect();

        assert_eq!(windows.len(), 4);
        assert_eq!(windows[0], vec![&[1, 2][..]]);
        assert_eq!(windows[1], vec![&[2, 3][..]]);
        assert_eq!(windows[2], vec![&[4, 5][..]]);
        assert_eq!(windows[3], vec![&[5, 6][..]]);
    }
}
