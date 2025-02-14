// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::position::Position;

/// An AABB bounding box.
///
/// # Class Invariant
///   - top_left.x <= bottom_right.x && top_left.y <= bottom_right.y
#[derive(Clone, PartialEq, Eq, Debug, Copy)]
pub struct BoundingBox {
    /// Top left corner of bounding box
    pub top_left: Position,

    /// Bottom right corner of bounding box
    pub bottom_right: Position,
}

impl BoundingBox {
    /// Returns height of bounding box
    pub fn height(&self) -> i32 {
        self.bottom_right.y - self.top_left.y
    }

    /// Returns width of bounding box
    pub fn width(&self) -> i32 {
        self.bottom_right.x - self.top_left.x
    }

    /// Returns centre point of bounding box
    pub fn centre(&self) -> Position {
        let x = (self.top_left.x + self.bottom_right.x) / 2;
        let y = (self.top_left.y + self.bottom_right.y) / 2;
        Position { x, y }
    }
}
