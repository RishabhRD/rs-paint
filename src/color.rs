// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    pub fn white() -> Self {
        Color::new(255, 255, 255)
    }

    pub fn black() -> Self {
        Color::new(0, 0, 0)
    }

    pub fn red() -> Self {
        Color::new(255, 0, 0)
    }

    pub fn green() -> Self {
        Color::new(0, 255, 0)
    }

    pub fn blue() -> Self {
        Color::new(0, 0, 255)
    }
}
