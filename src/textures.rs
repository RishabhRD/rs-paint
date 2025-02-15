// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{color::Color, position::Position};

/// Models texture patterns
pub trait Texture {
    /// Returns color at given position according to texture pattern.
    fn color_at(&mut self, pos: Position, prev_color: Color) -> Color;
}

pub mod no_texture;
pub use no_texture::*;
