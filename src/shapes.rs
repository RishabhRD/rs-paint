// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{bounding_box::BoundingBox, position::Position, regular::Regular};

pub trait Shape: Regular {
    /// Returns new shape, enclosed in given bounding box.
    fn new_inside(bounding_box: BoundingBox) -> Self;

    /// Returns border of given shape.
    fn border(&self) -> impl Iterator<Item = Position>;
}

pub mod ellipse;
pub use ellipse::*;
