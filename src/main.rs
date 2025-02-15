// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use color::Color;
use document::Document;
use egui::{Color32, ColorImage, ViewportBuilder};

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
            doc: Document::new(1000, 1000, Color::new(255, 255, 255)),
        }
    }
}

impl eframe::App for PaintApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut pixels = vec![];
        for x in 0..self.doc.width() {
            for y in 0..self.doc.height() {
                let c = self.doc.at(x, y);
                pixels.push(Color32::from_rgb(c.r, c.g, c.b));
            }
        }

        let image = ColorImage {
            size: [self.doc.width(), self.doc.height()],
            pixels,
        };

        let texture = ctx.load_texture("canvas", image, Default::default());

        egui::CentralPanel::default().show(ctx, |ui| {
            let response = ui.image(&texture);
            let pointer = ctx.input(|i| i.pointer.clone());

            if let Some(pos) = pointer.interact_pos() {
                if response.rect.contains(pos) {
                    let x = (pos.x / response.rect.width() * self.doc.width() as f32) as usize;
                    let y = (pos.y / response.rect.height() * self.doc.height() as f32) as usize;

                    if x < self.doc.width()
                        && y < self.doc.height()
                        && (pointer.primary_pressed() || pointer.primary_down())
                    {
                        *self.doc.at_mut(x, y) = Color::red();
                    }
                }
            }
        });

        // ctx.request_repaint(); // Ensure continuous redraw while dragging
    }
}

fn main() {
    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_fullscreen(true),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "rs-paint",
        native_options,
        Box::new(|cc| Ok(Box::new(PaintApp::new(cc)))),
    );
}
