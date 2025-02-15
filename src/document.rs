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

    /// Returns reference to color at (x, y) position
    ///
    /// # Precondition
    ///   - (x, y) is inside the doc.
    pub fn at(&self, x: usize, y: usize) -> &Color {
        let tile_x = x / self.m_tile_size;
        let tile_y = y / self.m_tile_size;
        let index = tile_y * self.m_width.div_ceil(self.m_tile_size) + tile_x;
        let tile = &self.m_tiles[index];

        let local_x = x % self.m_tile_size;
        let local_y = y % self.m_tile_size;

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
        let tile_x = x / self.m_tile_size;
        let tile_y = y / self.m_tile_size;
        let index = tile_y * self.m_width.div_ceil(self.m_tile_size) + tile_x;

        let tile = Arc::make_mut(&mut self.m_tiles[index]);

        let local_x = x % self.m_tile_size;
        let local_y = y % self.m_tile_size;

        tile.at_mut(local_y, local_x)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::color::Color;

    use super::Document;

    #[test]
    fn mutation() {
        let mut doc = Document::new(200, 300, Color::new(255, 255, 255));
        *doc.at_mut(0, 0) = Color::new(255, 0, 0);
        assert_eq!(*doc.at(0, 0), Color::new(255, 0, 0))
    }
}
