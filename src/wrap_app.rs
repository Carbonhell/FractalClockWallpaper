use eframe::{egui, epi};
use crate::fractal_clock::FractalClock;
use std::cell::RefCell;
use std::rc::Rc;

pub struct WrapApp {
    pub clock: Rc<RefCell<FractalClock>>,
}

impl epi::App for WrapApp {
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        self.clock.borrow_mut().update(ctx, frame);

    }

    fn name(&self) -> &str {
        "Fractal clock"
    }

    fn max_size_points(&self) -> egui::Vec2 {
        egui::Vec2::new(10240.0, 20480.0)
    }

    fn clear_color(&self) -> egui::Rgba {
        egui::Rgba::TRANSPARENT // we set a `CentralPanel` fill color in `demo_windows.rs`
    }
}