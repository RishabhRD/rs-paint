// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use color::Color;
use document::Document;
use egui::{Color32, ColorImage};
use paint_methods::fill_color;
use position::Position;

pub mod bounding_box;
pub mod canvas;
pub mod color;
pub mod document;
pub mod matrix;
pub mod paint_methods;
pub mod point_location;
pub mod position;
pub mod regular;
pub mod shapes;
pub mod textures;

pub struct PaintApp {
    doc: Document,
}

impl PaintApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        PaintApp {
            doc: Document::new(200, 200, Color::new(255, 255, 255)),
        }
    }
}

impl eframe::App for PaintApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let pixels = self
            .doc
            .data()
            .iter()
            .map(|c| Color32::from_rgb(c.r, c.g, c.b))
            .collect();
        let image = ColorImage {
            size: [self.doc.col(), self.doc.row()],
            pixels,
        };

        let texture = ctx.load_texture("canvas", image, Default::default());
        egui::CentralPanel::default().show(ctx, |ui| {
            let response = ui.image(&texture);
            if let Some(pos) = ctx.input(|i| i.pointer.interact_pos()) {
                if response.rect.contains(pos) && ctx.input(|i| i.pointer.any_click()) {
                    let x = (pos.x / response.rect.width() * self.doc.col() as f32) as usize;
                    let y = (pos.y / response.rect.height() * self.doc.row() as f32) as usize;

                    if x < self.doc.col() && y < self.doc.row() {
                        fill_color(
                            &mut self.doc,
                            Position::new(x as i32, y as i32),
                            Color::new(255, 0, 0),
                        );
                    }
                }
            }
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "rs-paint",
        native_options,
        Box::new(|cc| Ok(Box::new(PaintApp::new(cc)))),
    );
}
