// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{bounding_box::BoundingBox, point_location::PointLocation, position::Position};

use super::Shape;

#[derive(Clone, PartialEq, Eq, Debug, Copy)]
pub struct Ellipse {
    pub centre: Position,
    pub semi_major_axis: i32,
    pub semi_minor_axis: i32,
}

impl Shape for Ellipse {
    fn new_inside(bounding_box: BoundingBox) -> Self {
        let centre = bounding_box.centre();
        let semi_major_axis = bounding_box.width() / 2;
        let semi_minor_axis = bounding_box.height() / 2;
        Self {
            centre,
            semi_major_axis,
            semi_minor_axis,
        }
    }

    fn point_location(&self, point: Position) -> PointLocation {
        let h = self.centre.x as f64;
        let k = self.centre.y as f64;
        let a = self.semi_major_axis as f64;
        let b = self.semi_minor_axis as f64;
        let x = point.x as f64;
        let y = point.y as f64;

        // Ellipse equation
        let equation = ((x - h).powi(2) / a.powi(2)) + ((y - k).powi(2) / b.powi(2));

        if (equation - 1.0).abs() < 1e-6 {
            PointLocation::OnBorder
        } else if equation < 1.0 {
            PointLocation::Inside
        } else {
            PointLocation::Outside
        }
    }
}
