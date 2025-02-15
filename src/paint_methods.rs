// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{color::Color, document::Document, matrix, position::Position};

/// # Precondition
///   - pos is inside doc.
pub fn fill_color(doc: &mut Document, pos: Position, color: Color) {
    matrix::flood_fill_matrix(doc, pos.y as usize, pos.x as usize, color);
}
