// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use super::Texture;

pub struct NoTexture {}

impl Texture for NoTexture {
    fn color_at(
        &mut self,
        _: crate::position::Position,
        prev_color: crate::color::Color,
    ) -> crate::color::Color {
        prev_color
    }
}
