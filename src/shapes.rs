// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{bounding_box::BoundingBox, position::Position, regular::Regular};

pub trait Shape: Regular {
    /// Returns new shape, enclosed in given bounding box.
    fn new_inside(bounding_box: BoundingBox) -> Self;

    /// Returns true if given shape border exists at current position.
    fn is_border(&self, position: Position) -> bool;
}

pub mod ellipse;
