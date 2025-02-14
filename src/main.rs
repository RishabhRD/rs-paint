// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use matrix::Matrix;
use shapes::Shape;

pub mod bounding_box;
pub mod brushes;
pub mod canvas;
pub mod color;
pub mod document;
pub mod matrix;
pub mod position;
pub mod regular;
pub mod shapes;

fn main() {
    let mut matrix = Matrix::new(40, 40, ' ');
    let circle = shapes::Ellipse {
        centre: position::Position { x: 20, y: 20 },
        semi_minor_axis: 10,
        semi_major_axis: 10,
    };
    for pos in circle.border() {
        let i = pos.y;
        let j = pos.x;
        if i >= 0 && (i as usize) < matrix.row() && j >= 0 && (j as usize) < matrix.col() {
            *matrix.at_mut(i as usize, j as usize) = '.';
        }
    }

    for i in 0..matrix.row() {
        for j in 0..matrix.col() {
            print!("{}", matrix.at(i, j))
        }
        println!("\n");
    }
}
