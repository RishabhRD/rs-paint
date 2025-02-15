// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use std::sync::Arc;

use crate::{color::Color, matrix::Matrix};

pub type Tile = Matrix<Color>;

#[derive(Clone, Debug)]
pub struct Document {
    m_width: usize,
    m_height: usize,
    m_tiles: Vec<Arc<Tile>>,
    m_tile_size: usize,
}

impl Document {
    /// Creates a new document of width, height and given color using tile size
    /// 128 x 128.
    pub fn new(width: usize, height: usize, color: Color) -> Self {
        Self::new_with_tile_size(width, height, color, 128)
    }

    /// Creates a new document of width, height and given color using tile size
    /// tile_size x tile_size.
    pub fn new_with_tile_size(width: usize, height: usize, color: Color, tile_size: usize) -> Self {
        let tile = Arc::new(Tile::new(tile_size, tile_size, color));
        let tiles_x = width.div_ceil(tile_size);
        let tiles_y = height.div_ceil(tile_size);
        let tiles = vec![tile; tiles_x * tiles_y];

        Document {
            m_width: width,
            m_height: height,
            m_tiles: tiles,
            m_tile_size: tile_size,
        }
    }

    /// Returns width of document.
    pub fn width(&self) -> usize {
        self.m_width
    }

    /// Returns height of document.
    pub fn height(&self) -> usize {
        self.m_height
    }

    fn find_location(&self, x: usize, y: usize) -> (usize, usize, usize) {
        let num_tiles_in_row = self.width().div_ceil(self.m_tile_size);
        let pixel_number = y * self.width() + x;
        let num_pixels_in_a_row = self.width() * self.m_tile_size;
        let row_number = pixel_number / num_pixels_in_a_row;
        let col_number = x / self.m_tile_size;
        let tile_idx = num_tiles_in_row * row_number + col_number;
        let local_x = x % self.m_tile_size;
        let local_y = y % self.m_tile_size;
        (tile_idx, local_x, local_y)
    }

    /// Returns reference to color at (x, y) position
    ///
    /// # Precondition
    ///   - (x, y) is inside the doc.
    pub fn at(&self, x: usize, y: usize) -> &Color {
        let (tile_idx, local_x, local_y) = self.find_location(x, y);
        let tile = &self.m_tiles[tile_idx];
        tile.at(local_y, local_x)
    }

    /// Returns mutable reference to color at (x, y) position
    ///
    /// # Precondition
    ///   - (x, y) is inside the doc.
    ///
    /// # Postcondition
    ///   - Uses COW pattern, thus clones internal tile if given tile has some
    ///     other references too.
    pub fn at_mut(&mut self, x: usize, y: usize) -> &mut Color {
        let (tile_idx, local_x, local_y) = self.find_location(x, y);
        let tile = Arc::make_mut(&mut self.m_tiles[tile_idx]);
        tile.at_mut(local_y, local_x)
    }
}

pub struct FlatDocument {
    data: Matrix<Color>,
}

impl FlatDocument {
    /// Creates a new document of width, height and given color using tile size
    /// 128 x 128.
    pub fn new(width: usize, height: usize, color: Color) -> Self {
        Self::new_with_tile_size(width, height, color, 128)
    }

    /// Creates a new document of width, height and given color using tile size
    /// tile_size x tile_size.
    pub fn new_with_tile_size(
        width: usize,
        height: usize,
        color: Color,
        _tile_size: usize,
    ) -> Self {
        FlatDocument {
            data: Matrix::new(height, width, color),
        }
    }

    /// Returns width of document.
    pub fn width(&self) -> usize {
        self.data.cols()
    }

    /// Returns height of document.
    pub fn height(&self) -> usize {
        self.data.rows()
    }

    /// Returns reference to color at (x, y) position
    ///
    /// # Precondition
    ///   - (x, y) is inside the doc.
    pub fn at(&self, x: usize, y: usize) -> &Color {
        self.data.at(y, x)
    }

    /// Returns mutable reference to color at (x, y) position
    ///
    /// # Precondition
    ///   - (x, y) is inside the doc.
    ///
    /// # Postcondition
    ///   - Uses COW pattern, thus clones internal tile if given tile has some
    ///     other references too.
    pub fn at_mut(&mut self, x: usize, y: usize) -> &mut Color {
        self.data.at_mut(y, x)
    }
}
