// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Matrix<T> {
    m_row: usize,
    m_col: usize,
    m_data: Vec<T>,
}

impl<T> Matrix<T> {
    /// Creates a new matrix of size row * col
    pub fn new(row: usize, col: usize, value: T) -> Self
    where
        T: Clone,
    {
        Matrix {
            m_row: row,
            m_col: col,
            m_data: vec![value; row * col],
        }
    }

    /// Returns number of rows in matrix
    pub fn row(&self) -> usize {
        self.m_row
    }

    /// Returns number of columns in matrix
    pub fn col(&self) -> usize {
        self.m_col
    }

    /// Returns reference to ith row.
    ///
    /// Requires: i < self.row()
    pub fn row_at(&self, i: usize) -> &[T] {
        let start = i * self.m_col;
        let end = start + self.m_col;
        &self.m_data[start..end]
    }

    /// Returns mutable reference to ith row.
    ///
    /// Requires: i < self.row()
    pub fn row_at_mut(&mut self, i: usize) -> &mut [T] {
        let start = i * self.m_col;
        let end = start + self.m_col;
        &mut self.m_data[start..end]
    }

    /// Returns reference to element at (i, j).
    ///
    /// Requires: i < self.row && j < self.col
    pub fn at(&self, i: usize, j: usize) -> &T {
        let coord = i * self.m_col + j;
        &self.m_data[coord]
    }

    /// Returns reference to element at (i, j).
    ///
    /// Requires: i < self.row && j < self.col
    pub fn at_mut(&mut self, i: usize, j: usize) -> &mut T {
        let coord = i * self.m_col + j;
        &mut self.m_data[coord]
    }

    /// Swaps element at (i1, j1) with (i2, j2)
    ///
    /// Requires: i1, i2 < self.row && j1, j2 < self.col
    pub fn swap(&mut self, i1: usize, j1: usize, i2: usize, j2: usize) {
        let coord1 = i1 * self.m_col + j1;
        let coord2 = i2 * self.m_col + j2;
        self.m_data.swap(coord1, coord2);
    }
}
