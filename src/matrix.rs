// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use std::collections::VecDeque;

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

    /// Returns raw linear data.
    pub fn data(&self) -> &Vec<T> {
        &self.m_data
    }
}

impl<T> IntoIterator for Matrix<T> {
    type Item = T;

    type IntoIter = <std::vec::Vec<T> as std::iter::IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.m_data.into_iter()
    }
}

/// # Precondition
///   - (i, j) is valid index in matrix.
///
/// # Postcondition
///   - Flood fills matrix with new_e starting from (i, j).
pub fn flood_fill_matrix<T: Eq + Clone>(matrix: &mut Matrix<T>, i: usize, j: usize, new_e: T) {
    let old_e = matrix.at(i, j).clone();
    if old_e == new_e {
        return;
    }

    let mut stack = VecDeque::new();
    stack.push_back((i, j));

    while let Some((y, x)) = stack.pop_back() {
        let mut x1 = x;

        while x1 > 0 && *matrix.at(y, x1 - 1) == old_e {
            x1 -= 1;
        }

        let mut span_above = false;
        let mut span_below = false;

        while x1 < matrix.col() && *matrix.at(y, x1) == old_e {
            *matrix.at_mut(y, x1) = new_e.clone();

            if !span_above && y > 0 && *matrix.at(y - 1, x1) == old_e {
                stack.push_back((y - 1, x1));
                span_above = true;
            } else if span_above && y > 0 && *matrix.at(y - 1, x1) != old_e {
                span_above = false;
            }

            if !span_below && y < matrix.row() - 1 && *matrix.at(y + 1, x1) == old_e {
                stack.push_back((y + 1, x1));
                span_below = true;
            } else if span_below && y < matrix.row() - 1 && *matrix.at(y + 1, x1) != old_e {
                span_below = false;
            }
            x1 += 1;
        }
    }
}
