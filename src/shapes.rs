// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    bounding_box::BoundingBox, point_location::PointLocation, position::Position, regular::Regular,
};

pub trait Shape: Regular {
    /// Returns new shape, enclosed in given bounding box.
    fn new_inside(bounding_box: BoundingBox) -> Self;

    /// Returns border of given shape.
    fn point_location(&self, point: Position) -> PointLocation;
}

pub mod ellipse;
pub use ellipse::*;
