// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{bounding_box::BoundingBox, position::Position};

use super::Shape;

#[derive(Clone, PartialEq, Eq, Debug, Copy)]
pub struct Ellipse {
    pub centre: Position,
    pub semi_major_axis: i32,
    pub semi_minor_axis: i32,
}

struct EllipseBorderIterator<'a> {
    ellipse: &'a Ellipse,
    angle: f64,
    step: f64,
}

impl<'a> EllipseBorderIterator<'a> {
    pub fn new(ellipse: &'a Ellipse) -> Self {
        Self {
            ellipse,
            angle: 0.0,
            step: 1.0, // Step in degrees
        }
    }
}

impl Iterator for EllipseBorderIterator<'_> {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.angle >= 360.0 {
            return None;
        }

        let h = self.ellipse.centre.x as f64;
        let k = self.ellipse.centre.y as f64;
        let a = self.ellipse.semi_major_axis as f64;
        let b = self.ellipse.semi_minor_axis as f64;

        let theta = self.angle.to_radians();
        let x = h + a * theta.cos();
        let y = k + b * theta.sin();

        self.angle += self.step;

        Some(Position {
            x: x.round() as i32,
            y: y.round() as i32,
        })
    }
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

    fn border(&self) -> impl Iterator<Item = Position> {
        EllipseBorderIterator::new(self)
    }
}
