// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use point_location::PointLocation;
use position::Position;
use shapes::Shape;

pub mod bounding_box;
pub mod brushes;
pub mod canvas;
pub mod color;
pub mod document;
pub mod matrix;
pub mod point_location;
pub mod position;
pub mod regular;
pub mod shapes;

fn main() {
    let circle = shapes::Ellipse {
        centre: position::Position { x: 20, y: 20 },
        semi_minor_axis: 10,
        semi_major_axis: 10,
    };

    for i in 0..40 {
        for j in 0..40 {
            if circle.point_location(Position { x: j, y: i }) == PointLocation::OnBorder {
                print!("x");
            } else {
                print!(" ");
            }
        }
        println!("\n");
    }
}
