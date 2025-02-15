// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{color::Color, document::Document, position::Position};

fn fill_color_dfs(doc: &mut Document, i: i32, j: i32, start_color: Color, color: Color) {
    if i < 0 || j < 0 || (i as usize) >= doc.row() || (j as usize) >= doc.col() {
        return;
    }

    let cur_color = doc.at_mut(i as usize, j as usize);
    if *cur_color != start_color || *cur_color == color {
        return;
    }

    *cur_color = color;
    fill_color_dfs(doc, i - 1, j, start_color, color);
    fill_color_dfs(doc, i + 1, j, start_color, color);
    fill_color_dfs(doc, i, j - 1, start_color, color);
    fill_color_dfs(doc, i, j + 1, start_color, color);
}

/// # Precondition
///   - pos is inside doc.
pub fn fill_color(doc: &mut Document, pos: Position, color: Color) {
    let i = pos.y;
    let j = pos.x;

    let start_color = *doc.at(i as usize, j as usize);
    fill_color_dfs(doc, i, j, start_color, color);
}
